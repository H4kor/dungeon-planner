use crate::common::Vec2;
use crate::state::commands::AddVertexToRoomCommand;
use crate::state::StateController;
use gtk::gdk::ffi::GDK_BUTTON_PRIMARY;
use gtk::gdk::ButtonEvent;
use gtk::gdk::EventType::ButtonPress;
use gtk::glib::Propagation;
use gtk::{prelude::*, EventControllerLegacy};
use gtk::{DrawingArea, EventControllerMotion};
use std::cell::RefCell;
use std::rc::Rc;

pub struct Canvas {
    pub widget: DrawingArea,
}

impl Canvas {
    pub fn new(control: Rc<RefCell<StateController>>) -> Rc<RefCell<Self>> {
        let drawing_area = DrawingArea::builder()
            .width_request(800)
            .height_request(600)
            .hexpand(true)
            .vexpand(true)
            .valign(gtk::Align::Fill)
            .halign(gtk::Align::Fill)
            .build();

        let canvas = Rc::new(RefCell::new(Canvas {
            widget: drawing_area.clone(),
        }));

        {
            let control = control.clone();
            drawing_area.set_draw_func(move |_area, ctx, w, h| {
                let control = control.borrow();
                // fill with background color
                ctx.set_source_rgb(0.1411764705882353, 0.28627450980392155, 0.49411764705882355);
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
            let control = control.clone();
            let canvas = canvas.clone();
            mouse_controller.connect_motion(move |_con, x, y| {
                let control = &mut *control.borrow_mut();
                control.state.cursor.set_pos(Vec2 { x: x, y: y });
                canvas.borrow().update();
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
                        match control.state.active_room_id {
                            None => (),
                            Some(active_room_id) => {
                                match control.state.dungeon.room(active_room_id) {
                                    None => (),
                                    Some(room) => {
                                        let room_id = room.id.unwrap();
                                        control.apply(RefCell::new(std::boxed::Box::new(
                                            AddVertexToRoomCommand {
                                                room_id: room_id,
                                                pos: control.state.grid.snap(Vec2 {
                                                    x: control.state.cursor.pos.x as i32,
                                                    y: control.state.cursor.pos.y as i32,
                                                }),
                                            },
                                        )));
                                    }
                                }
                            }
                        }
                        canvas.borrow().update();
                    }
                }
                Propagation::Proceed
            });
        }
        drawing_area.add_controller(mouse_controller);
        drawing_area.add_controller(event_controller);
        canvas
    }

    pub fn update(&self) {
        println!("update called");
        self.widget.queue_draw()
    }
}
