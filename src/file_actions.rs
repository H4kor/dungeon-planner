use crate::export::{to_full_player_map_pdf, to_pdf, to_player_cutout_pdf};
use crate::observers::HistoryObserver;
use crate::state::StateController;
use crate::storage;
use cairo::glib::clone;
use gtk::gio::{ActionEntry, SimpleActionGroup};
use gtk::{glib, ApplicationWindow, FileFilter, MessageDialog};
use gtk::{prelude::*, FileChooserDialog};
use std::cell::RefCell;
use std::rc::Rc;

pub fn dungeon_file_filter() -> FileFilter {
    let ff = FileFilter::new();
    ff.add_suffix("dungeon");
    ff
}

pub fn pdf_filter() -> FileFilter {
    let ff = FileFilter::new();
    ff.add_suffix("pdf");
    ff
}

fn save_as_dialog<F: Fn() + 'static>(
    title: String,
    control: Rc<RefCell<StateController>>,
    history: Rc<RefCell<HistoryObserver>>,
    window: ApplicationWindow,
    after_success: F,
) {
    let file_dialog = FileChooserDialog::builder()
        .title(title)
        .action(gtk::FileChooserAction::Save)
        .select_multiple(false)
        .create_folders(true)
        .modal(true)
        .filter(&dungeon_file_filter())
        .build();
    file_dialog.add_button("Save", gtk::ResponseType::Accept);
    file_dialog.add_button("Cancel", gtk::ResponseType::Cancel);
    file_dialog.connect_response(
        clone!(@weak control, @weak history, @weak window => move |dialog, r| {
            match r {
                gtk::ResponseType::Accept => {
                    let file = dialog.file().unwrap();
                    let mut path = file.parse_name().to_string();
                    if !path.ends_with(".dungeon") {
                        path += ".dungeon";
                    }
                    history.borrow_mut().change_file(path.clone());
                    history.borrow_mut().save_to_file();
                    let title = format!("Dungeon Planner - {path}");
                    window.set_title(Some(&title));
                    dialog.close();
                    after_success();
                }
                gtk::ResponseType::Cancel => dialog.close(),
                gtk::ResponseType::DeleteEvent => (),
                _ => todo!(),
            }
        }),
    );
    file_dialog.show();
}

fn open_dialog(
    control: Rc<RefCell<StateController>>,
    history: Rc<RefCell<HistoryObserver>>,
    window: ApplicationWindow,
) {
    let file_dialog = FileChooserDialog::builder()
        .title("Open Dungeon File ...")
        .action(gtk::FileChooserAction::Open)
        .select_multiple(false)
        .create_folders(true)
        .modal(true)
        .filter(&dungeon_file_filter())
        .build();
    file_dialog.add_button("Open", gtk::ResponseType::Accept);
    file_dialog.add_button("Cancel", gtk::ResponseType::Cancel);
    file_dialog.connect_response(
        clone!(@weak control, @weak history, @weak window => move |dialog, r| {
            match r {
                gtk::ResponseType::Accept => {
                    let file = dialog.file().unwrap();
                    let path = file.parse_name().to_string();
                    control.borrow_mut().reset();
                    history.borrow_mut().reset();
                    history.borrow_mut().change_file(path.clone());
                    let cmds = storage::load_dungeon(path.clone());
                    history.borrow_mut().set_history(cmds.clone());
                    for cmd in cmds {
                        control.borrow_mut().apply_silent(cmd);    
                    }
                    control.borrow_mut().reload();
                    let title = format!("Dungeon Planner - {path}");
                    window.set_title(Some(&title));
                    dialog.close();
                }
                gtk::ResponseType::Cancel => dialog.close(),
                gtk::ResponseType::DeleteEvent => (),
                _ => todo!(),
            }
        }),
    );
    file_dialog.show();
}

pub fn unsaved_dialog(
    control: Rc<RefCell<StateController>>,
    history: Rc<RefCell<HistoryObserver>>,
    window: ApplicationWindow,
) {
    let unsaved_dialog = MessageDialog::builder()
        .message_type(gtk::MessageType::Warning)
        .buttons(gtk::ButtonsType::YesNo)
        .text("You have unsaved changes. Do you want to save these?")
        .modal(true)
        .build();
    unsaved_dialog.connect_response(clone!( @weak control, @weak history, @weak window => move |dialog, r| {
        println!("unsaved_dialog.connect_response: {:?}", r);
        match r {
            gtk::ResponseType::Yes => {
                match history.clone().borrow().save_file() {
                    Some(_) => history.borrow_mut().save_to_file(),
                    None => save_as_dialog(
                        "Save Dungeon ...".to_owned(),
                        control.clone(), history.clone(), window.clone(),
                        Box::new(clone!( @weak control, @weak history, @weak window => move || {
                            history.borrow_mut().reset();
                            control.borrow_mut().reset();
                            let title = format!("Dungeon Planner - Unsaved Dungeon");
                            window.set_title(Some(&title));
                        }))
                    ),
                }
            }
            gtk::ResponseType::No => {
                history.borrow_mut().reset();
                control.borrow_mut().reset();
                let title = format!("Dungeon Planner - Unsaved Dungeon");
                window.set_title(Some(&title));
            },
            _ => (),
        }
        dialog.close();
    }));
    unsaved_dialog.show();
}

pub fn unsaved_dialog_open(
    control: Rc<RefCell<StateController>>,
    history: Rc<RefCell<HistoryObserver>>,
    window: ApplicationWindow,
) {
    let unsaved_dialog = MessageDialog::builder()
        .message_type(gtk::MessageType::Warning)
        .buttons(gtk::ButtonsType::YesNo)
        .text("You have unsaved changes. Do you want to save these?")
        .modal(true)
        .build();
    unsaved_dialog.connect_response(clone!( @weak control, @weak history, @weak window => move |dialog, r| {
        println!("unsaved_dialog.connect_response: {:?}", r);
        match r {
            gtk::ResponseType::Yes => {
                let file = history.clone().borrow().save_file();
                match file {
                    Some(_) => history.borrow_mut().save_to_file(),
                    None => save_as_dialog(
                        "Save Dungeon ...".to_owned(),
                        control.clone(), history.clone(), window.clone(),
                        Box::new(clone!( @weak control, @weak history, @weak window => move || {
                            open_dialog(control, history, window)
                        }))
                    ),
                }
            }
            gtk::ResponseType::No => open_dialog(control, history, window),
            _ => (),
        }
        dialog.close();
    }));
    unsaved_dialog.show();
}

pub fn file_actions(
    control: Rc<RefCell<StateController>>,
    history: Rc<RefCell<HistoryObserver>>,
    window: ApplicationWindow,
) -> SimpleActionGroup {
    let file_actions = SimpleActionGroup::new();
    let action_file_new = ActionEntry::builder("new")
        .activate(
            clone!( @weak control, @weak history, @weak window => move |_group: &SimpleActionGroup, _, _| {
                if !history.borrow().unsaved_state() {
                    history.borrow_mut().reset();
                    control.borrow_mut().reset();
                    let title = format!("Dungeon Planner - Unsaved Dungeon");
                    window.set_title(Some(&title));
                } else {
                    unsaved_dialog(
                        control.clone(), history.clone(), window.clone(),
                    );
                }
            }),
        )
        .build();

    let action_file_open = ActionEntry::builder("open")
        .activate(clone!( @weak control, @weak history, @weak window => move |_group: &SimpleActionGroup, _, _| {
            match history.clone().borrow().unsaved_state() {
                true => {
                    unsaved_dialog_open(
                        control.clone(), history.clone(), window.clone(),
                    );
                    
                },
                false => open_dialog(control, history, window)
            }
        }))
        .build();

    let action_file_save = ActionEntry::builder("save")
        .activate(clone!( @weak control, @weak history, @strong window => move |_group: &SimpleActionGroup, _, _| {
            let save_file = history.borrow().save_file();
            match save_file {
                Some(_) => {
                    history.borrow_mut().save_to_file();
                },
                None => {
                    save_as_dialog("Save Dungeon ...".to_owned(), control, history, window.clone(), Box::new(||{}));
                },
            }
        }))
        .build();

    let action_file_save_as = ActionEntry::builder("save_as")
        .activate(clone!( @weak control, @weak history, @weak window => move |_group: &SimpleActionGroup, _, _| {
            save_as_dialog("Save Dungeon As ...".to_owned(), control, history, window, Box::new(||{}));
        }))
        .build();

    let action_file_export_pdf = ActionEntry::builder("export_pdf")
        .activate(clone!( @weak control, @weak history => move |_group: &SimpleActionGroup, _, _| {
            let file_dialog = FileChooserDialog::builder()
                .title("Export Dungeon ...")
                .action(gtk::FileChooserAction::Save)
                .select_multiple(false)
                .create_folders(true)
                .modal(true)
                .filter(&pdf_filter())
                .build();
            file_dialog.add_button("Export", gtk::ResponseType::Accept);
            file_dialog.add_button("Cancel", gtk::ResponseType::Cancel);
            file_dialog.connect_response(clone!(@weak control, @weak history => move |dialog, r| {
                match r {
                    gtk::ResponseType::Accept => {
                        let file = dialog.file().unwrap();
                        let mut path = file.parse_name().to_string();
                        if !path.ends_with(".pdf") {
                            path += ".pdf";
                        }
                        to_pdf(&control.borrow().state.dungeon, path);
                        dialog.close();
                    }
                    gtk::ResponseType::Cancel => dialog.close(),
                    gtk::ResponseType::DeleteEvent => (),
                    _ => todo!(),
                }
            }));
            file_dialog.show();
        }))
        .build();

    let action_file_export_player_map_pdf = ActionEntry::builder("player_map_pdf")
        .activate(clone!( @weak control, @weak history => move |_group: &SimpleActionGroup, _, _| {
            let file_dialog = FileChooserDialog::builder()
                .title("Export Player Map ...")
                .action(gtk::FileChooserAction::Save)
                .select_multiple(false)
                .create_folders(true)
                .modal(true)
                .filter(&pdf_filter())
                .build();
            file_dialog.add_button("Export", gtk::ResponseType::Accept);
            file_dialog.add_button("Cancel", gtk::ResponseType::Cancel);
            file_dialog.connect_response(clone!(@weak control, @weak history => move |dialog, r| {
                match r {
                    gtk::ResponseType::Accept => {
                        let file = dialog.file().unwrap();
                        let mut path = file.parse_name().to_string();
                        if !path.ends_with(".pdf") {
                            path += ".pdf";
                        }
                        to_full_player_map_pdf(&control.borrow().state.dungeon, path);
                        dialog.close();
                    }
                    gtk::ResponseType::Cancel => dialog.close(),
                    gtk::ResponseType::DeleteEvent => (),
                    _ => todo!(),
                }
            }));
            file_dialog.show();
        }))
        .build();    


    let action_file_export_cutout_pdf = ActionEntry::builder("cutout_pdf")
        .activate(clone!( @weak control, @weak history => move |_group: &SimpleActionGroup, _, _| {
            let file_dialog = FileChooserDialog::builder()
                .title("Export Cutout Map ...")
                .action(gtk::FileChooserAction::Save)
                .select_multiple(false)
                .create_folders(true)
                .modal(true)
                .filter(&pdf_filter())
                .build();
            file_dialog.add_button("Export", gtk::ResponseType::Accept);
            file_dialog.add_button("Cancel", gtk::ResponseType::Cancel);
            file_dialog.connect_response(clone!(@weak control, @weak history => move |dialog, r| {
                match r {
                    gtk::ResponseType::Accept => {
                        let file = dialog.file().unwrap();
                        let mut path = file.parse_name().to_string();
                        if !path.ends_with(".pdf") {
                            path += ".pdf";
                        }
                        to_player_cutout_pdf(&control.borrow().state.dungeon, path);
                        dialog.close();
                    }
                    gtk::ResponseType::Cancel => dialog.close(),
                    gtk::ResponseType::DeleteEvent => (),
                    _ => todo!(),
                }
            }));
            file_dialog.show();
        }))
        .build();    

    file_actions.add_action_entries([
        action_file_new,
        action_file_open,
        action_file_save,
        action_file_save_as,
        action_file_export_pdf,
        action_file_export_player_map_pdf,
        action_file_export_cutout_pdf,
    ]);

    file_actions
}
