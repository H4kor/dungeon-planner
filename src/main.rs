mod chamber;
mod common;
mod config;
mod door;
mod dungeon;
mod edit_actions;
mod export;
mod file_actions;
pub mod observers;
mod state;
mod storage;
mod view;

use gtk::gdk::Display;
use gtk::gio::{ActionEntry, Menu, MenuItem, MenuModel};
use gtk::{glib, ApplicationWindow, CssProvider, Label, Notebook};
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
    let select_chamber_button =
        EditModeButton::new(control.clone(), state::EditMode::Select, "edit-find");
    let split_edge_button =
        EditModeButton::new(control.clone(), state::EditMode::SplitEdge, "edit-cut");
    let append_verts_button = EditModeButton::new(
        control.clone(),
        state::EditMode::AppendChamber,
        "document-edit",
    );
    let delete_corner_button = EditModeButton::new(
        control.clone(),
        state::EditMode::RemoveVertex,
        "list-remove",
    );

    let add_door_button =
        EditModeButton::new(control.clone(), state::EditMode::AddDoor, "insert-link");

    tool_box.append(&add_chamber_button.widget);
    tool_box.append(&select_chamber_button.borrow().widget);
    tool_box.append(&append_verts_button.borrow().widget);
    tool_box.append(&split_edge_button.borrow().widget);
    tool_box.append(&delete_corner_button.borrow().widget);
    tool_box.append(&add_door_button.borrow().widget);
    side_box.append(&tool_box);

    let object_tabs = Notebook::builder().build();
    side_box.append(&object_tabs);

    let chamber_tab = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();
    let chamber_tab_label = Label::new(Some("Chambers"));
    object_tabs.append_page(&chamber_tab, Some(&chamber_tab_label));
    let chamber_list = ChamberList::new(control.clone());
    let chamber_edit = ChamberEdit::new(control.clone());
    chamber_tab.append(&chamber_list.borrow().scrolled_window);
    chamber_tab.append(&chamber_edit.borrow().widget);

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
        20,
        &MenuItem::new(Some("Export Player Map ..."), Some("file.player_map_pdf")),
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

    // DebugObserver::new(control.clone());

    window.present();
}
