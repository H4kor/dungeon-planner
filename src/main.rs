mod chamber;
mod common;
mod config;
mod door;
mod dungeon;
mod edit_actions;
mod export;
mod file_actions;
mod object;
pub mod observers;
mod state;
mod storage;
mod view;

use cairo::glib::{clone, Propagation};
use file_actions::save_as_dialog;
use gtk::gdk::Display;
use gtk::gio::{ActionEntry, Menu, MenuItem, MenuModel};
use gtk::{glib, ApplicationWindow, CssProvider, MessageDialog};
use gtk::{prelude::*, PopoverMenuBar};
use observers::HistoryObserver;
use state::StateController;
use std::cell::RefCell;
use std::rc::Rc;
use view::buttons::{AddChamberButton, EditModeButton};
use view::canvas::Canvas;
use view::chamber_edit::ChamberEdit;
use view::chamber_list::ChamberList;
use view::door_edit::DoorEdit;
use view::door_list::DoorList;
use view::dungeon_edit::DungeonEdit;
use view::entity_tabs::EntityTabs;
use view::object_edit::ObjectEdit;
use view::object_list::ObjectList;

const APP_ID: &str = "org.rerere.DungeonPlanner";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = adw::Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Set keyboard accelerator to trigger "win.close".
    app.set_accels_for_action("win.close", &["<Ctrl>W"]);
    app.set_accels_for_action("file.new", &["<Ctrl>N"]);
    app.set_accels_for_action("file.open", &["<Ctrl>O"]);
    app.set_accels_for_action("file.save", &["<Ctrl>S"]);
    app.set_accels_for_action("file.save_as", &["<Ctrl><Shift>S"]);
    app.set_accels_for_action("file.export_pdf", &["<Ctrl>P"]);
    app.set_accels_for_action("file.player_map_pdf", &["<Ctrl><Shift>P"]);

    app.set_accels_for_action("edit.unselect", &["Escape"]);
    app.set_accels_for_action("edit.delete_selected", &["<Ctrl><Alt>X"]);
    app.set_accels_for_action("edit.undo", &["<Ctrl>Z"]);
    app.set_accels_for_action("edit.add_chamber", &["<Alt>C"]);

    // modes
    app.set_accels_for_action("edit.mode_select", &["<Alt>S"]);
    app.set_accels_for_action("edit.mode_append_chamber", &["<Alt>A"]);
    app.set_accels_for_action("edit.mode_split_edge", &["<Alt>F"]);
    app.set_accels_for_action("edit.mode_add_door", &["<Alt>D"]);

    app.connect_startup(|_| load_css());

    // Run the application
    app.run()
}

fn load_css() {
    // Load the CSS file and add it to the provider
    let provider = CssProvider::new();
    provider.load_from_data(include_str!("style.css"));

    // Add the provider to the default screen
    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn build_ui(app: &adw::Application) {
    let control = Rc::new(RefCell::new(StateController::new()));

    let history = HistoryObserver::new(control.clone(), None);

    /*
     * |--------|-----------------------|
     * |  Tools |                       |
     * |--------|                       |
     * |chamber |          Canvas       |
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
    let add_chamber_button = AddChamberButton::new(control.clone());
    let select_chamber_button = EditModeButton::new(
        control.clone(),
        state::EditMode::Select,
        include_bytes!("../assets/icons/select.png").to_vec(),
        "Select",
    );
    let split_edge_button = EditModeButton::new(
        control.clone(),
        state::EditMode::SplitEdge,
        include_bytes!("../assets/icons/split_wall.png").to_vec(),
        "Split Wall",
    );
    let append_verts_button = EditModeButton::new(
        control.clone(),
        state::EditMode::AppendChamber,
        include_bytes!("../assets/icons/append_chamber.png").to_vec(),
        "Draw Chamber",
    );
    let delete_corner_button = EditModeButton::new(
        control.clone(),
        state::EditMode::RemoveVertex,
        include_bytes!("../assets/icons/remove_corner.png").to_vec(),
        "Remove Corner",
    );

    let add_door_button = EditModeButton::new(
        control.clone(),
        state::EditMode::AddDoor,
        include_bytes!("../assets/icons/add_door.png").to_vec(),
        "Insert Door",
    );

    let add_object_button = EditModeButton::new(
        control.clone(),
        state::EditMode::AddObject,
        include_bytes!("../assets/icons/add_object.png").to_vec(),
        "Insert Object",
    );

    tool_box.append(&add_chamber_button.widget);
    tool_box.append(&select_chamber_button.borrow().widget);
    tool_box.append(&append_verts_button.borrow().widget);
    tool_box.append(&split_edge_button.borrow().widget);
    tool_box.append(&delete_corner_button.borrow().widget);
    tool_box.append(&add_door_button.borrow().widget);
    tool_box.append(&add_object_button.borrow().widget);
    side_box.append(&tool_box);

    let dungeon_tab = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();
    let dungeon_edit = DungeonEdit::new(control.clone());
    dungeon_tab.append(&dungeon_edit.borrow().widget);

    let chamber_tab = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();
    let chamber_list = ChamberList::new(control.clone());
    let chamber_edit = ChamberEdit::new(control.clone());
    chamber_tab.append(&chamber_list.borrow().scrolled_window);
    chamber_tab.append(&chamber_edit.borrow().widget);

    let door_tab = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();
    let door_list = DoorList::new(control.clone());
    let door_edit = DoorEdit::new(control.clone());
    door_tab.append(&door_list.borrow().scrolled_window);
    door_tab.append(&door_edit.borrow().widget);

    let object_tab = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();
    let object_list = ObjectList::new(control.clone());
    let object_edit = ObjectEdit::new(control.clone());
    object_tab.append(&object_list.borrow().scrolled_window);
    object_tab.append(&object_edit.borrow().widget);

    let object_tabs = EntityTabs::new(
        control.clone(),
        dungeon_tab,
        chamber_tab,
        door_tab,
        object_tab,
    );
    side_box.append(&object_tabs.borrow().widget);

    let main_box = gtk::Paned::builder()
        .start_child(&side_box)
        .end_child(&canvas.borrow().widget)
        .build();

    let file_menu = Menu::new();
    file_menu.insert_item(0, &MenuItem::new(Some("New Dungeon"), Some("file.new")));
    file_menu.insert_item(5, &MenuItem::new(Some("Open ..."), Some("file.open")));
    file_menu.insert_item(10, &MenuItem::new(Some("Save ..."), Some("file.save")));
    file_menu.insert_item(
        11,
        &MenuItem::new(Some("Save As ..."), Some("file.save_as")),
    );
    file_menu.insert_item(
        20,
        &MenuItem::new(Some("Export PDF ..."), Some("file.export_pdf")),
    );
    file_menu.insert_item(
        21,
        &MenuItem::new(Some("Export Player Map ..."), Some("file.player_map_pdf")),
    );
    file_menu.insert_item(
        22,
        &MenuItem::new(Some("Export Cutout Map ..."), Some("file.cutout_pdf")),
    );

    let edit_menu = Menu::new();
    edit_menu.insert_item(0, &MenuItem::new(Some("Undo"), Some("edit.undo")));
    edit_menu.insert_item(
        10,
        &MenuItem::new(Some("Add new Chamber"), Some("edit.add_chamber")),
    );
    let mode_menu = Menu::new();
    mode_menu.insert_item(0, &MenuItem::new(Some("Select"), Some("edit.mode_select")));
    mode_menu.insert_item(
        0,
        &MenuItem::new(Some("Append Chamber"), Some("edit.mode_append_chamber")),
    );
    mode_menu.insert_item(
        0,
        &MenuItem::new(Some("Split Wall"), Some("edit.mode_split_edge")),
    );
    mode_menu.insert_item(
        0,
        &MenuItem::new(Some("Add Door"), Some("edit.mode_add_door")),
    );
    edit_menu.insert_submenu(20, Some("Change Mode"), &mode_menu);

    let menu = Menu::new();
    menu.insert_submenu(0, Some("File"), &file_menu);
    menu.insert_submenu(1, Some("Edit"), &edit_menu);
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
        .title("Dungeon Planner - Unsaved Dungeon")
        .child(&window_box)
        .maximized(true)
        .show_menubar(true)
        .build();

    // actions
    let file_actions = file_actions::file_actions(control.clone(), history.clone(), window.clone());
    window.insert_action_group("file", Some(&file_actions));

    let edit_actions = edit_actions::edit_actions(control.clone(), history.clone());
    window.insert_action_group("edit", Some(&edit_actions));

    let action_close = ActionEntry::builder("close")
        .activate(|window: &ApplicationWindow, _, _| {
            window.close();
        })
        .build();
    window.add_action_entries([action_close]);
    let force_close = Rc::new(RefCell::new(false));
    window.connect_close_request(clone!(@strong control, @strong history, @strong force_close => move |window| {
        if history.borrow().unsaved_state() {
            if *force_close.borrow() == true {
                return Propagation::Proceed
            }
            let unsaved_dialog = MessageDialog::builder()
                .message_type(gtk::MessageType::Warning)
                .buttons(gtk::ButtonsType::YesNo)
                .text("You have unsaved changes. Do you want to save these?")
                .modal(true)
                .build();
            unsaved_dialog.connect_response(
                clone!( @weak control, @weak history, @weak window, @strong force_close => move |dialog, r| {
                    println!("unsaved_dialog.connect_response: {:?}", r);
                    match r {
                        gtk::ResponseType::Yes => {
                            match history.clone().borrow().save_file() {
                                Some(_) => history.borrow_mut().save_to_file(),
                                None => save_as_dialog(
                                    "Save Dungeon ...".to_owned(),
                                    control.clone(), history.clone(), window.clone(),
                                    Box::new(clone!( @weak control, @weak history, @weak window => move || {
                                    }))
                                ),
                            }
                        }
                        gtk::ResponseType::No => {
                            *force_close.borrow_mut() = true;
                            window.close()
                        },
                        _ => (),
                    }
                    dialog.close();
                }),
            );
            unsaved_dialog.show();
            Propagation::Stop
        } else {
            Propagation::Proceed
        }
    }));

    #[cfg(debug_assertions)]
    {
        observers::DebugObserver::new(control.clone());
    }

    window.present();
}
