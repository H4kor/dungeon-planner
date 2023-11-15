use crate::common::{Rgb, Vec2};
use crate::state::commands::AddVertexToRoomCommand;
use crate::state::StateController;
use gtk::gdk::ffi::{GDK_BUTTON_PRIMARY, GDK_BUTTON_SECONDARY};
use gtk::{prelude::*, GestureClick, GestureDrag};
use gtk::{DrawingArea, EventControllerMotion};
use std::boxed::Box;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

use super::primitives::{Line, Primitive};

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

                // draw nearest wall
                match control.state.dungeon.nearest_wall(cp) {
                    None => (),
                    Some((_, wall)) => Line {
                        from: Vec2::<f64> {
                            x: wall.p1.x as f64,
                            y: wall.p1.y as f64,
                        },
                        to: Vec2::<f64> {
                            x: wall.p2.x as f64,
                            y: wall.p2.y as f64,
                        },
                        color: Rgb {
                            r: 1.0,
                            g: 0.0,
                            b: 0.0,
                        },
                        width: 10.0,
                    }
                    .draw(ctx),
                }

                // debug circle
                // ctx.set_source_rgb(1.0, 0.0, 0.0);
                // ctx.arc(200.0, 200.0, 20.0, 0.0, 2.0 * std::f64::consts::PI); // full circle
                // ctx.fill().unwrap()
            });
        }

        let pos_controller = EventControllerMotion::new();
        {
            let control = control.clone();
            let canvas = canvas.clone();
            pos_controller.connect_motion(move |_con, x, y| {
                let control = &mut *control.borrow_mut();
                control.state.cursor.set_pos(Vec2 { x: x, y: y });
                canvas.borrow().update();
            });
        }
        let gesture_click = GestureClick::builder()
            .button(GDK_BUTTON_PRIMARY as u32)
            .build();

        {
            let control = control.clone();
            let canvas = canvas.clone();
            gesture_click.connect_pressed(move |_, _, _, _| {
                let control = &mut *control.borrow_mut();
                if let Some(active_room_id) = control.state.active_room_id {
                    if let Some(room) = control.state.dungeon.room(active_room_id) {
                        let room_id = room.id.unwrap();
                        control.apply(RefCell::new(Box::new(AddVertexToRoomCommand {
                            room_id: room_id,
                            pos: control.state.grid.snap(Vec2 {
                                x: control.state.cursor.pos.x as i32,
                                y: control.state.cursor.pos.y as i32,
                            }),
                        })));
                    }
                }
                canvas.borrow().update();
            });
        }

        let gesture_drag = GestureDrag::builder()
            .button(GDK_BUTTON_SECONDARY as u32)
            .build();

        {
            {
                let control = control.clone();
                let last_pos: Rc<Cell<Option<Vec2<i32>>>> = Rc::new(Cell::new(None));
                {
                    let last_pos = last_pos.clone();
                    gesture_drag.connect_begin(move |_, _| last_pos.set(None));
                }
                gesture_drag.connect_drag_update(move |_, x, y| {
                    let mut control = control.borrow_mut();
                    let mut view_obj = control.state.view;
                    let last = last_pos.get().unwrap_or(Vec2 { x: 0, y: 0 });
                    let cur = Vec2 {
                        x: x as i32,
                        y: y as i32,
                    };
                    view_obj.move_view(last - cur);
                    control.state.view = view_obj;
                    last_pos.set(Some(cur));
                });
            }
        }

        drawing_area.add_controller(gesture_drag);
        drawing_area.add_controller(gesture_click);
        drawing_area.add_controller(pos_controller);
        canvas
    }

    pub fn update(&self) {
        self.widget.queue_draw()
    }
}
