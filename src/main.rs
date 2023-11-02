mod common;
mod dungeon;
pub mod observers;
mod room;
mod state;
mod storage;
mod view;

use crate::common::Vec2;
use crate::state::commands::menu::SelectRoomCommand;
use dungeon::Dungeon;
use gtk::{gdk, prelude::*};
use gtk::{glib, Application, ApplicationWindow};
use observers::{DebugObserver, StorageObserver};
use state::StateController;
use std::cell::RefCell;
use std::rc::Rc;
use view::buttons::AddRoomButton;
use view::canvas::Canvas;
use view::grid::Grid;
use view::room_edit::RoomEdit;
use view::room_list::RoomList;
use view::View;

const APP_ID: &str = "org.rerere.DungeonPlanner";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    let control = Rc::new(RefCell::new(StateController::new(
        Dungeon::new(),
        Grid::new(),
        View::new(),
    )));

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

    let main_box = gtk::Box::builder().build();
    let side_box = gtk::Box::builder()
        .width_request(300)
        .orientation(gtk::Orientation::Vertical)
        .build();

    let tool_box = gtk::FlowBox::builder()
        .selection_mode(gtk::SelectionMode::None)
        .build();

    let canvas = Canvas::new(control.clone());
    let add_room_button = AddRoomButton::new(control.clone());
    let room_list = RoomList::new(control.clone());
    let room_edit = RoomEdit::new(control.clone());

    // new room
    tool_box.append(&add_room_button.widget);
    // selection button
    tool_box.append(&gtk::Button::builder().icon_name("edit-find").build());
    // edit button
    tool_box.append(&gtk::Button::builder().icon_name("document-edit").build());
    // delete button
    tool_box.append(&gtk::Button::builder().icon_name("edit-delete").build());

    side_box.append(&tool_box);
    side_box.append(&room_list.borrow().scrolled_window);
    side_box.append(&room_edit.borrow().widget);

    main_box.append(&side_box);
    main_box.append(&canvas.borrow().widget);

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Dungeon Planner")
        .child(&main_box)
        .build();

    let control_key = gtk::EventControllerKey::new();

    {
        let control = control.clone();
        let canvas = canvas.clone();
        control_key.connect_key_pressed(move |_, key, _, _| {
            let mut control = control.borrow_mut();
            let mut view_obj = control.state.view;
            const SPEED: i32 = 10;
            match key {
                gdk::Key::Right => view_obj.move_view(Vec2 { x: SPEED, y: 0 }),
                gdk::Key::Left => view_obj.move_view(Vec2 { x: -SPEED, y: 0 }),
                gdk::Key::Up => view_obj.move_view(Vec2 { x: 0, y: -SPEED }),
                gdk::Key::Down => view_obj.move_view(Vec2 { x: 0, y: SPEED }),
                gdk::Key::Escape => {
                    control.apply(RefCell::new(Box::new(SelectRoomCommand { room_id: None })))
                }
                _ => (),
            }
            control.state.view = view_obj;
            canvas.borrow().update();
            glib::Propagation::Proceed
        });
    }

    DebugObserver::new(control.clone());
    storage::load_dungeon(control.clone());
    StorageObserver::new(control.clone());

    // Present window
    window.add_controller(control_key);

    window.present();
}
