mod common;
mod dungeon;
mod room;
mod state;
mod view;

use std::cell::RefCell;
use std::rc::Rc;

use dungeon::Dungeon;
use gtk::gdk::ffi::GDK_BUTTON_PRIMARY;
use gtk::gdk::ButtonEvent;
use gtk::gdk::EventType::ButtonPress;
use gtk::glib::Propagation;
use gtk::{gdk, prelude::*, EventControllerLegacy};
use gtk::{glib, Application, ApplicationWindow};
use gtk::{DrawingArea, EventControllerMotion};
use state::StateController;
use view::add_room_button::AddRoomButton;
use view::grid::Grid;
use view::room_edit::RoomEdit;
use view::room_list::RoomList;
use view::View;

use crate::common::Vec2;
use crate::state::commands::AddVertexToRoomCommand;

const APP_ID: &str = "org.rerere.DungeonPlanner";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_canvas(control: Rc<RefCell<StateController>>) -> DrawingArea {
    let canvas = DrawingArea::builder()
        .width_request(800)
        .height_request(600)
        .hexpand(true)
        .vexpand(true)
        .valign(gtk::Align::Fill)
        .halign(gtk::Align::Fill)
        .build();

    {
        let control = control.clone();
        canvas.set_draw_func(move |_area, ctx, w, h| {
            let control = control.borrow();
            // fill with background color
            ctx.set_source_rgb(0.8, 0.95, 0.8);
            ctx.paint().unwrap();

            // apply "camera"
            let world_min = control.state.view.world_min();
            ctx.translate(-world_min.x as f64, -world_min.y as f64);

            // draw grid
            let prims = control
                .state
                .grid
                .draw(world_min, world_min + Vec2 { x: w, y: h });
            for prim in prims {
                prim.draw(ctx)
            }

            // draw room
            let cp = control.state.cursor.pos;
            let next_vert = control.state.grid.snap(Vec2::<i32> {
                x: cp.x as i32,
                y: cp.y as i32,
            });

            for room in control.dungeon().rooms.iter() {
                let mut vert_opt = None;
                if control.state.active_room_id == room.id {
                    vert_opt = Some(next_vert)
                }
                let prims = room.draw(vert_opt);
                for prim in prims {
                    prim.draw(ctx)
                }
            }

            // debug circle
            // ctx.set_source_rgb(1.0, 0.0, 0.0);
            // ctx.arc(200.0, 200.0, 20.0, 0.0, 2.0 * std::f64::consts::PI); // full circle
            // ctx.fill().unwrap()
        });
    }

    let mouse_controller = EventControllerMotion::new();
    {
        let canvas = canvas.clone();
        let control = control.clone();
        mouse_controller.connect_motion(move |_con, x, y| {
            let control = &mut *control.borrow_mut();
            control.state.cursor.set_pos(Vec2 { x: x, y: y });
            canvas.queue_draw();
        });
    }
    let event_controller = EventControllerLegacy::new();

    {
        let control = control.clone();
        let canvas = canvas.clone();
        event_controller.connect_event(move |_controller, event| {
            let control = &mut *control.borrow_mut();
            if event.event_type() == ButtonPress {
                let button_event = event.clone().downcast::<ButtonEvent>().unwrap();
                if button_event.button() == GDK_BUTTON_PRIMARY as u32 {
                    println!("got mouse button: {}", button_event.button());

                    match control.state.active_room_id {
                        None => (),
                        Some(active_room_id) => match control.state.dungeon.room(active_room_id) {
                            None => (),
                            Some(room) => {
                                let room_id = room.id.unwrap();
                                control.apply(std::boxed::Box::new(AddVertexToRoomCommand {
                                    room_id: room_id,
                                    pos: control.state.grid.snap(Vec2 {
                                        x: control.state.cursor.pos.x as i32,
                                        y: control.state.cursor.pos.y as i32,
                                    }),
                                }));
                            }
                        },
                    }
                    canvas.queue_draw();
                }
            }
            Propagation::Proceed
        });
    }
    canvas.add_controller(mouse_controller);
    canvas.add_controller(event_controller);

    canvas
}

fn build_ui(app: &Application) {
    let control = Rc::new(RefCell::new(StateController::new(
        Dungeon::new(),
        Grid::new(),
        View::new(),
    )));

    let main_box = gtk::Box::builder().build();
    let menu_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();

    let canvas = build_canvas(control.clone());

    let add_room_button = AddRoomButton::new(control.clone());
    let room_list = RoomList::new(control.clone());
    let room_edit = RoomEdit::new(control.clone());

    menu_box.append(&add_room_button.widget);
    menu_box.append(&room_list.borrow().scrolled_window);
    menu_box.append(&room_edit.widget);

    main_box.append(&menu_box);
    main_box.append(&canvas);

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
                _ => (),
            }
            control.state.view = view_obj;
            canvas.queue_draw();
            glib::Propagation::Proceed
        });
    }

    // Present window
    window.add_controller(control_key);
    window.present();
}
