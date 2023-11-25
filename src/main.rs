mod common;
mod config;
mod dungeon;
pub mod observers;
mod room;
mod state;
mod storage;
mod view;

use gtk::{gdk, prelude::*};
use gtk::{glib, Application, ApplicationWindow};
use observers::{DebugObserver, HistoryObserver};
use state::{StateCommand, StateController};
use std::cell::RefCell;
use std::rc::Rc;
use view::buttons::AddRoomButton;
use view::canvas::Canvas;
use view::room_edit::RoomEdit;
use view::room_list::RoomList;

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
    let control = Rc::new(RefCell::new(StateController::new()));

    let history = HistoryObserver::new(control.clone(), "dungeon.txt".to_owned());

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
        let history = history.clone();
        control_key.connect_key_pressed(move |_, key, _, modifier| {
            let mut control = control.borrow_mut();
            match key {
                gdk::Key::Escape => control.apply(StateCommand::SelectRoom(None)),

                _ => (),
            }
            match modifier {
                // CTRL + [ ]
                gdk::ModifierType::CONTROL_MASK => {
                    println!("Control!");
                    match key {
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
                    }
                }
                x => println!("Modifier {:?}", x),
            }
            canvas.borrow().update();
            glib::Propagation::Proceed
        });
    }

    DebugObserver::new(control.clone());
    storage::load_dungeon(control.clone());
    history.borrow_mut().activate();

    // Present window
    window.add_controller(control_key);

    window.present();
}
