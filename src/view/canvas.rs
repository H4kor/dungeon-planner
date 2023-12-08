use crate::common::{Rgb, Vec2};
use crate::config::ACTIVE_ROOM_COLOR;
use crate::door::{Door, DoorDrawOptions};
use crate::room::{NextVert, Wall, WallId};
use crate::state::{EditMode, State, StateCommand, StateController, StateEventSubscriber};
use cairo::glib::{clone, Closure};
use gtk::gdk::ffi::{GDK_BUTTON_PRIMARY, GDK_BUTTON_SECONDARY};
use gtk::{glib, prelude::*, GestureClick, GestureDrag};
use gtk::{DrawingArea, EventControllerMotion};
use std::cell::{Cell, RefCell};
use std::rc::Rc;

use super::primitives::{Line, Primitive};

pub struct Canvas {
    pub widget: DrawingArea,
    selected_wall: Option<WallId>,
    last_pos: Option<Vec2<i32>>,
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
            selected_wall: None,
            last_pos: None,
        }));
        // let selected_wall: Rc<Cell<Option<WallId>>> = Rc::new(Cell::new(None));

        drawing_area.set_draw_func(
            clone!( @strong canvas, @weak control => move |_area, ctx, w, h| {
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

                // draw rooms
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
                                match canvas.borrow().selected_wall() {
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
                    let prims = room.draw(vert_opt, active, None);
                    for prim in prims {
                        prim.draw(ctx)
                    }
                }

                // draw doors
                for door in control.dungeon().doors.iter() {
                    let options = match control.state.active_door_id == door.id {
                        true => DoorDrawOptions{
                            color: Some(ACTIVE_ROOM_COLOR),
                        },
                        false => DoorDrawOptions::empty(),
                    };
                    let prims = door.draw(
                        control
                            .dungeon()
                            .room(door.part_of)
                            .unwrap()
                            .wall(door.on_wall)
                            .unwrap(),
                        options
                    );
                    for prim in prims {
                        prim.draw(ctx)
                    }
                }

                /*
                 * Mode Specific Drawing
                 */
                match control.state.mode {
                    EditMode::Select => {}
                    EditMode::AppendRoom => {}
                    EditMode::SplitEdge => {
                        // Highlight nearest wall
                        let canvas = canvas.borrow();
                        if canvas.selected_wall() == None {
                            canvas.highlighted_nearest_wall(&control.state, cp, ctx);
                        }
                    }
                    EditMode::AddDoor => {
                        // Highlight nearest wall
                        let canvas = canvas.borrow();
                        match canvas.selected_wall() {
                            None => canvas.highlighted_nearest_wall(&control.state, cp, ctx),
                            Some(wall_id) => {
                                if let Some(room) = control.state.active_room() {
                                    let wall = room.wall(wall_id).unwrap();
                                    let door_pos = wall.nearest_relative_pos(control.state.cursor.pos);

                                    let door = Door::new(
                                        room.id.unwrap(), None,
                                        50.0, // TODO: adjustable
                                        wall.id,
                                        door_pos
                                    );

                                    let prims = door.draw(wall, DoorDrawOptions::empty());
                                    for p in prims {
                                        p.draw(ctx)
                                    }
                                }
                            }
                        }
                    }
                }

                // debug circle
                // ctx.set_source_rgb(1.0, 0.0, 0.0);
                // ctx.arc(200.0, 200.0, 20.0, 0.0, 2.0 * std::f64::consts::PI); // full circle
                // ctx.fill().unwrap()
            }),
        );

        let pos_controller = EventControllerMotion::new();
        pos_controller.connect_motion(clone!( @strong canvas, @weak control => move |_con, x, y| {
            let control = &mut *control.borrow_mut();
            control.state.cursor.set_pos(Vec2 { x: x, y: y });
            canvas.borrow().update();
        }));
        let gesture_click = GestureClick::builder()
            .button(GDK_BUTTON_PRIMARY as u32)
            .build();

        gesture_click.connect_pressed(clone!( @strong canvas, @weak control => move |_, _, _, _| {
            let control = &mut *control.borrow_mut();
            match control.state.mode {
                EditMode::Select => {
                    let door_id = control.state.dungeon.door_at(control.state.cursor.pos);
                    if let Some(id) = door_id {
                        control.apply(StateCommand::SelectDoor(Some(id)));
                    } else {
                        let room_id = control.state.dungeon.room_at(control.state.cursor.pos);
                        control.apply(StateCommand::SelectRoom(room_id));
                    }
                }
                EditMode::AppendRoom => {
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
                EditMode::SplitEdge => {
                    let selected_wall = canvas.borrow().selected_wall();
                    match selected_wall {
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
                                    canvas.borrow_mut().set_selected_wall(None);
                                }
                            }
                        }
                        None => {
                            canvas.borrow_mut().select_nearest_wall(&control.state);
                        }
                }},
                EditMode::AddDoor => {
                    let selected_wall = canvas.borrow().selected_wall();
                    match selected_wall {
                        None => {
                            canvas.borrow_mut().select_nearest_wall(&control.state);
                        }
                        Some(wall_id) => {
                                if let Some(room) = control.state.active_room() {
                                    let wall = room.wall(wall_id).unwrap();
                                    let door_pos = wall.nearest_relative_pos(control.state.cursor.pos);

                                    let door = Door::new(
                                        room.id.unwrap(), None,
                                        50.0, // TODO: adjustable
                                        wall.id,
                                        door_pos
                                    );
                                    canvas.borrow_mut().set_selected_wall(None);
                                    control.apply(StateCommand::AddDoor(door));
                                }
                        }
                    }
                },
            }
            canvas.borrow().update();
        }));

        let gesture_drag = GestureDrag::builder()
            .button(GDK_BUTTON_SECONDARY as u32)
            .build();
        gesture_drag.connect_begin(
            clone!(@strong canvas => move |_, _| canvas.borrow_mut().set_last_pos(None)),
        );
        gesture_drag.connect_drag_update(clone!(@strong canvas, @weak control => move |_, x, y| {
            let mut control = control.borrow_mut();
            let mut view_obj = control.state.view;
            let last = canvas.borrow().last_pos().unwrap_or(Vec2 { x: 0, y: 0 });
            let cur = Vec2 {
                x: x as i32,
                y: y as i32,
            };
            view_obj.move_view(last - cur);
            control.state.view = view_obj;
            canvas.borrow_mut().set_last_pos(Some(cur));
        }));

        drawing_area.add_controller(gesture_drag);
        drawing_area.add_controller(gesture_click);
        drawing_area.add_controller(pos_controller);

        control.borrow_mut().subscribe_any(canvas.clone());
        canvas
    }

    pub fn update(&self) {
        self.widget.queue_draw()
    }

    pub fn selected_wall(&self) -> Option<WallId> {
        self.selected_wall
    }

    pub fn set_selected_wall(&mut self, wall_id: Option<WallId>) {
        self.selected_wall = wall_id
    }

    pub fn select_nearest_wall(&mut self, state: &State) {
        if let Some(active_room_id) = state.active_room_id {
            if let Some(room) = state.dungeon.room(active_room_id) {
                if let Some(wall) =
                    room.nearest_wall(state.cursor.pos + state.view.world_min().into())
                {
                    self.set_selected_wall(Some(wall.id))
                }
            }
        }
    }

    pub fn last_pos(&self) -> Option<Vec2<i32>> {
        self.last_pos
    }

    pub fn set_last_pos(&mut self, pos: Option<Vec2<i32>>) {
        self.last_pos = pos
    }

    pub fn highlighted_nearest_wall(&self, state: &State, pos: Vec2<f64>, ctx: &cairo::Context) {
        if let Some(active_room_id) = state.active_room_id {
            if let Some(room) = state.dungeon.room(active_room_id) {
                match room.nearest_wall(pos) {
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
