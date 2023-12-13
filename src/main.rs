mod common;
mod config;
mod door;
mod dungeon;
mod edit_actions;
mod export;
mod file_actions;
pub mod observers;
mod room;
mod state;
mod storage;
mod view;

use gtk::gio::{ActionEntry, Menu, MenuItem, MenuModel};
use gtk::{glib, Application, ApplicationWindow, Label, Notebook};
use gtk::{prelude::*, PopoverMenuBar};
use observers::{DebugObserver, HistoryObserver};
use state::StateController;
use std::cell::RefCell;
use std::rc::Rc;
use view::buttons::{AddRoomButton, EditModeButton};
use view::canvas::Canvas;
use view::door_edit::DoorEdit;
use view::door_list::DoorList;
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

    app.set_accels_for_action("edit.unselect", &["Escape"]);
    app.set_accels_for_action("edit.delete_selected", &["Delete"]);
    app.set_accels_for_action("edit.undo", &["<Ctrl>Z"]);

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
    let select_room_button =
        EditModeButton::new(control.clone(), state::EditMode::Select, "edit-find");
    let split_edge_button =
        EditModeButton::new(control.clone(), state::EditMode::SplitEdge, "edit-cut");
    let append_verts_button = EditModeButton::new(
        control.clone(),
        state::EditMode::AppendRoom,
        "document-edit",
    );
    let add_door_button =
        EditModeButton::new(control.clone(), state::EditMode::AddDoor, "insert-link");

    tool_box.append(&add_room_button.widget);
    tool_box.append(&select_room_button.borrow().widget);
    tool_box.append(&split_edge_button.borrow().widget);
    tool_box.append(&append_verts_button.borrow().widget);
    tool_box.append(&add_door_button.borrow().widget);
    side_box.append(&tool_box);

    let object_tabs = Notebook::builder().build();
    side_box.append(&object_tabs);

    let room_tab = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();
    let room_tab_label = Label::new(Some("Rooms"));
    object_tabs.append_page(&room_tab, Some(&room_tab_label));
    let room_list = RoomList::new(control.clone());
    let room_edit = RoomEdit::new(control.clone());
    room_tab.append(&room_list.borrow().scrolled_window);
    room_tab.append(&room_edit.borrow().widget);

    let door_tab = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();
    let door_tab_label = Label::new(Some("Doors"));
    object_tabs.append_page(&door_tab, Some(&door_tab_label));
    let door_list = DoorList::new(control.clone());
    let door_edit = DoorEdit::new(control.clone());
    door_tab.append(&door_list.borrow().scrolled_window);
    door_tab.append(&door_edit.borrow().widget);

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

    let edit_menu = Menu::new();
    let edit_menu_undo = MenuItem::new(Some("Undo"), Some("edit.undo"));
    edit_menu.insert_item(0, &edit_menu_undo);

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

    DebugObserver::new(control.clone());
    history.borrow_mut().activate();

    window.present();
}
