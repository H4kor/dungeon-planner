mod common;
mod config;
mod dungeon;
mod export;
mod file_actions;
pub mod observers;
mod room;
mod state;
mod storage;
mod view;

use cairo::glib::clone;
use export::to_pdf;
use gtk::gio::{ActionEntry, Menu, MenuItem, MenuModel, SimpleAction, SimpleActionGroup};
use gtk::{gdk, prelude::*, FileChooserDialog, PopoverMenuBar};
use gtk::{glib, Application, ApplicationWindow};
use observers::{DebugObserver, HistoryObserver};
use state::{StateCommand, StateController};
use std::cell::RefCell;
use std::rc::Rc;
use view::buttons::{AddRoomButton, AppendRoomButton, SelectRoomButton, SplitEdgeButton};
use view::canvas::Canvas;
use view::room_edit::RoomEdit;
use view::room_list::RoomList;

const APP_ID: &str = "org.rerere.DungeonPlanner";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Set keyboard accelerator to trigger "win.close".
    app.set_accels_for_action("win.close", &["<Ctrl>W"]);
    app.set_accels_for_action("file.new", &["<Ctrl>N"]);
    app.set_accels_for_action("file.open", &["<Ctrl>O"]);
    app.set_accels_for_action("file.save", &["<Ctrl>S"]);
    app.set_accels_for_action("file.save_as", &["<Ctrl><Shift>S"]);
    app.set_accels_for_action("file.export_pdf", &["<Ctrl>P"]);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    let control = Rc::new(RefCell::new(StateController::new()));

    let history = HistoryObserver::new(control.clone(), None);

    /*
     * |--------|-----------------------|
     * |  Tools |                       |
     * |--------|                       |
     * |  Room  |          Canvas       |
     * |  List  |                       |
     * |--------|                       |
     * |Context |                       |
     * |--------|-----------------------|
     */

    let side_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();

    let tool_box = gtk::FlowBox::builder()
        .selection_mode(gtk::SelectionMode::None)
        .build();

    let canvas = Canvas::new(control.clone());
    let add_room_button = AddRoomButton::new(control.clone());
    let select_room_button = SelectRoomButton::new(control.clone());
    let split_edge_button = SplitEdgeButton::new(control.clone());
    let append_verts_button = AppendRoomButton::new(control.clone());
    let room_list = RoomList::new(control.clone());
    let room_edit = RoomEdit::new(control.clone());

    // new room
    tool_box.append(&add_room_button.widget);
    tool_box.append(&select_room_button.widget);
    tool_box.append(&split_edge_button.widget);
    tool_box.append(&append_verts_button.widget);

    // // selection button
    // tool_box.append(&gtk::Button::builder().icon_name("edit-find").build());
    // // edit button
    // tool_box.append(&gtk::Button::builder().icon_name("document-edit").build());
    // // delete button
    // tool_box.append(&gtk::Button::builder().icon_name("edit-delete").build());

    side_box.append(&tool_box);
    side_box.append(&room_list.borrow().scrolled_window);
    side_box.append(&room_edit.borrow().widget);

    let main_box = gtk::Paned::builder()
        .start_child(&side_box)
        .end_child(&canvas.borrow().widget)
        .build();

    let file_menu = Menu::new();
    let file_menu_new = MenuItem::new(Some("New Dungeon"), Some("file.new"));
    let file_menu_open = MenuItem::new(Some("Open ..."), Some("file.open"));
    let file_menu_save = MenuItem::new(Some("Save ..."), Some("file.save"));
    let file_menu_save_as = MenuItem::new(Some("Save As ..."), Some("file.save_as"));
    let file_menu_print = MenuItem::new(Some("Export PDF ..."), Some("file.export_pdf"));
    file_menu.insert_item(0, &file_menu_new);
    file_menu.insert_item(5, &file_menu_open);
    file_menu.insert_item(10, &file_menu_save);
    file_menu.insert_item(11, &file_menu_save_as);
    file_menu.insert_item(20, &file_menu_print);

    // https://gtk-rs.org/gtk4-rs/stable/latest/book/actions.html?highlight=Act#menus

    let menu = Menu::new();
    menu.insert_submenu(0, Some("File"), &file_menu);
    let menu_model: MenuModel = menu.into();

    let menubar = PopoverMenuBar::from_model(Some(&menu_model));

    let window_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();
    window_box.append(&menubar);
    window_box.append(&main_box);

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Dungeon Planner")
        .child(&window_box)
        .maximized(true)
        .show_menubar(true)
        .build();

    // actions
    let file_actions = file_actions::file_actions(control.clone(), history.clone());
    window.insert_action_group("file", Some(&file_actions));

    let action_close = ActionEntry::builder("close")
        .activate(|window: &ApplicationWindow, _, _| {
            window.close();
        })
        .build();
    window.add_action_entries([action_close]);

    // controllers
    let control_key = gtk::EventControllerKey::new();
    {
        let control = control.clone();
        let canvas = canvas.clone();
        let history = history.clone();
        control_key.connect_key_pressed(move |_, key, _, modifier| {
            let mut control = control.borrow_mut();
            match key {
                gdk::Key::Escape => control.apply(StateCommand::SelectRoom(None)),

                _ => (),
            }
            match modifier {
                // CTRL + [ ]
                gdk::ModifierType::CONTROL_MASK => match key {
                    gdk::Key::z => {
                        control.reset();
                        let cmds = {
                            history.borrow_mut().undo();
                            history.borrow_mut().get_stack()
                        };

                        history.borrow_mut().start_restore();
                        for cmd in cmds {
                            control.apply(cmd)
                        }
                        history.borrow_mut().end_restore();
                    }
                    _ => (),
                },
                x => println!("Modifier {:?}", x),
            }
            canvas.borrow().update();
            glib::Propagation::Proceed
        });
    }

    DebugObserver::new(control.clone());
    history.borrow_mut().activate();

    // Present window
    window.add_controller(control_key);

    window.present();
}
