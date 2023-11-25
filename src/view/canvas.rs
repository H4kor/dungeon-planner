use crate::common::{Rgb, Vec2};
use crate::room::{NextVert, WallId};
use crate::state::{EditMode, StateCommand, StateController, StateEventSubscriber};
use gtk::gdk::ffi::{GDK_BUTTON_PRIMARY, GDK_BUTTON_SECONDARY};
use gtk::{prelude::*, GestureClick, GestureDrag};
use gtk::{DrawingArea, EventControllerMotion};
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
        let selected_wall: Rc<Cell<Option<WallId>>> = Rc::new(Cell::new(None));

        {
            let control = control.clone();
            let selected_wall = selected_wall.clone();
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
                let cp = control.state.cursor.pos + control.state.view.world_min().into();
                let next_vert = control.state.grid.snap(cp.into());

                for room in control.dungeon().rooms.iter() {
                    let active = control.state.active_room_id == room.id;
                    let vert_opt = match active {
                        true => match control.state.mode {
                            EditMode::AppendRoom => Some(NextVert {
                                in_wall_id: None,
                                pos: next_vert,
                            }),
                            EditMode::SplitEdge => {
                                println!("{:?}", selected_wall.get());
                                match selected_wall.get() {
                                    Some(wall) => Some(NextVert {
                                        in_wall_id: Some(wall),
                                        pos: next_vert,
                                    }),
                                    None => None,
                                }
                            }
                            _ => None,
                        },
                        false => None,
                    };
                    let prims = room.draw(vert_opt, active);
                    for prim in prims {
                        prim.draw(ctx)
                    }
                }

                // draw nearest wall
                if selected_wall.get() == None && control.state.mode == EditMode::SplitEdge {
                    if let Some(active_room_id) = control.state.active_room_id {
                        if let Some(room) = control.state.dungeon.room(active_room_id) {
                            match room.nearest_wall(cp) {
                                None => (),
                                Some(wall) => Line {
                                    from: wall.p1.into(),
                                    to: wall.p2.into(),
                                    color: Rgb {
                                        r: 1.0,
                                        g: 0.0,
                                        b: 0.0,
                                    },
                                    width: 3.0,
                                }
                                .draw(ctx),
                            }
                        }
                    }
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
                match control.state.mode {
                    crate::state::EditMode::Select => {
                        let room_id = control.state.dungeon.room_at(control.state.cursor.pos);
                        control.apply(StateCommand::SelectRoom(room_id));
                    }
                    crate::state::EditMode::AppendRoom => {
                        if let Some(active_room_id) = control.state.active_room_id {
                            if let Some(room) = control.state.dungeon.room_mut(active_room_id) {
                                let room_id = room.id.unwrap();
                                control.apply(StateCommand::AddVertexToRoom(
                                    room_id,
                                    control.state.grid.snap(
                                        (control.state.cursor.pos
                                            + control.state.view.world_min().into())
                                        .into(),
                                    ),
                                ));
                            }
                        }
                    }
                    crate::state::EditMode::SplitEdge => match selected_wall.get() {
                        Some(wall_id) => {
                            if let Some(active_room_id) = control.state.active_room_id {
                                if let Some(room) = control.state.dungeon.room_mut(active_room_id) {
                                    let room_id = room.id.unwrap();
                                    control.apply(StateCommand::SplitWall(
                                        room_id,
                                        wall_id,
                                        control.state.grid.snap(
                                            (control.state.cursor.pos
                                                + control.state.view.world_min().into())
                                            .into(),
                                        ),
                                    ));
                                    selected_wall.set(None);
                                }
                            }
                        }
                        None => {
                            if let Some(active_room_id) = control.state.active_room_id {
                                if let Some(room) = control.state.dungeon.room_mut(active_room_id) {
                                    if let Some(wall) = room.nearest_wall(control.state.cursor.pos)
                                    {
                                        selected_wall.set(Some(wall.id))
                                    }
                                }
                            }
                        }
                    },
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

        control.borrow_mut().subscribe_any(canvas.clone());
        canvas
    }

    pub fn update(&self) {
        self.widget.queue_draw()
    }
}

impl StateEventSubscriber for Canvas {
    fn on_state_event(
        &mut self,
        _state: &mut crate::state::State,
        _event: crate::state::events::StateEvent,
    ) -> Vec<StateCommand> {
        self.widget.queue_draw();
        vec![]
    }
}
