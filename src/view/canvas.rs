use crate::chamber::{ChamberDrawOptions, NextVert, WallId};
use crate::common::{Rgb, Vec2};
use crate::config::{
    BACKGROUND_COLOR, PRIMARY_CHAMBER_COLOR, SECONDARY_ACTIVE_COLOR, TERTIARY_ACTIVE_COLOR,
};
use crate::door::{Door, DoorDrawOptions};
use crate::state::{EditMode, State, StateCommand, StateController, StateEventSubscriber};
use cairo::glib::{clone, Propagation};
use cairo::Context;
use gtk::gdk::ffi::{GDK_BUTTON_PRIMARY, GDK_BUTTON_SECONDARY};
use gtk::{gdk, glib, prelude::*, EventControllerKey, GestureClick, GestureDrag};
use gtk::{DrawingArea, EventControllerMotion};
use std::cell::RefCell;
use std::rc::Rc;

use super::primitives::{Line, Point, Primitive};

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
            .can_focus(true)
            .focus_on_click(true)
            .focusable(true)
            .valign(gtk::Align::Fill)
            .halign(gtk::Align::Fill)
            .build();

        let canvas = Rc::new(RefCell::new(Canvas {
            widget: drawing_area.clone(),
            selected_wall: None,
            last_pos: None,
        }));

        drawing_area.set_draw_func(
            clone!( @strong canvas, @weak control => move |_area, ctx, w, h| {
                canvas.borrow().draw(ctx, w, h, control);
            }),
        );

        let pos_controller = EventControllerMotion::new();
        pos_controller.connect_motion(clone!( @strong canvas, @weak control => move |_con, x, y| {
            canvas.borrow().motion(control, x, y);
        }));

        let gesture_click = GestureClick::builder()
            .button(GDK_BUTTON_PRIMARY as u32)
            .build();

        gesture_click.connect_pressed(
            clone!( @strong canvas, @weak control, @weak drawing_area => move |_, _, _, _| {
                drawing_area.grab_focus();
                let cmds = canvas.borrow_mut().click(control.clone());
                for cmd in cmds {
                    control.borrow_mut().apply(cmd)
                }
            }),
        );

        let gesture_drag = GestureDrag::builder()
            .button(GDK_BUTTON_SECONDARY as u32)
            .build();
        gesture_drag
            .connect_begin(clone!(@strong canvas => move |_, _| canvas.borrow_mut().drag_begin()));
        gesture_drag.connect_drag_update(clone!(@strong canvas, @weak control => move |_, x, y| {
            canvas.borrow_mut().drag_update(control, x, y);
        }));

        let key_controller = EventControllerKey::new();
        key_controller.connect_key_pressed(clone!(@strong control => move |_, key, _, _| {
            let mut control = control.borrow_mut();
            match key {
                gdk::Key::Delete => {
                    if let Some(door_id) = control.state.active_door_id {
                        control
                            .apply(StateCommand::DeleteDoor(door_id))
                    }
                    if let Some(chamber_id) = control.state.active_chamber_id {
                        control
                            .apply(StateCommand::DeleteChamber(chamber_id))
                    }
                },
                _ => (),
            }
            Propagation::Proceed
        }));
        drawing_area.add_controller(key_controller);
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
        if let Some(active_chamber_id) = state.active_chamber_id {
            if let Some(chamber) = state.dungeon.chamber(active_chamber_id) {
                if let Some(wall) = chamber.nearest_wall(state.cursor_world_pos()) {
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
        if let Some(active_chamber_id) = state.active_chamber_id {
            if let Some(chamber) = state.dungeon.chamber(active_chamber_id) {
                match chamber.nearest_wall(pos) {
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

    fn draw(&self, ctx: &Context, w: i32, h: i32, control: Rc<RefCell<StateController>>) {
        let control = control.borrow();
        // fill with background color
        ctx.set_source_rgb(BACKGROUND_COLOR.r, BACKGROUND_COLOR.g, BACKGROUND_COLOR.b);
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

        // draw chambers
        let cp = control.state.cursor_world_pos();
        let next_vert = control.state.grid.snap(cp.into());

        for chamber in control.dungeon().chambers.iter() {
            let active = control.state.active_chamber_id == Some(chamber.id);
            let vert_opt = match active {
                true => match control.state.mode {
                    EditMode::AppendChamber => Some(NextVert {
                        in_wall_id: None,
                        pos: next_vert,
                    }),
                    EditMode::SplitEdge => match self.selected_wall() {
                        Some(wall) => Some(NextVert {
                            in_wall_id: Some(wall),
                            pos: next_vert,
                        }),
                        None => None,
                    },
                    _ => None,
                },
                false => None,
            };
            let prims = chamber.draw(
                vert_opt,
                match active {
                    false => {
                        if let Some(door) = control.state.active_door() {
                            if door.part_of == chamber.id {
                                Some(ChamberDrawOptions {
                                    color: Some(SECONDARY_ACTIVE_COLOR),
                                    fill: None,
                                })
                            } else if door.leads_to == Some(chamber.id) {
                                Some(ChamberDrawOptions {
                                    color: Some(TERTIARY_ACTIVE_COLOR),
                                    fill: None,
                                })
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                    true => Some(ChamberDrawOptions {
                        color: Some(PRIMARY_CHAMBER_COLOR),
                        fill: None,
                    }),
                },
            );
            for prim in prims {
                prim.draw(ctx)
            }
        }

        // draw doors
        for door in control.dungeon().doors.iter() {
            let options = match control.state.active_door_id == Some(door.id) {
                true => DoorDrawOptions {
                    color: Some(PRIMARY_CHAMBER_COLOR),
                },
                false => DoorDrawOptions::empty(),
            };
            let prims = door.draw(
                control
                    .dungeon()
                    .chamber(door.part_of)
                    .unwrap()
                    .wall(door.on_wall)
                    .unwrap(),
                options,
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
            EditMode::AppendChamber => {}
            EditMode::SplitEdge => {
                // Highlight nearest wall
                if self.selected_wall() == None {
                    self.highlighted_nearest_wall(&control.state, cp, ctx);
                }
            }
            EditMode::AddDoor => {
                // Highlight nearest wall
                match self.selected_wall() {
                    None => self.highlighted_nearest_wall(&control.state, cp, ctx),
                    Some(wall_id) => {
                        if let Some(chamber) = control.state.active_chamber() {
                            let wall = chamber.wall(wall_id).unwrap();
                            let door_pos =
                                wall.nearest_relative_pos(control.state.cursor_world_pos());

                            let door = Door::new(
                                chamber.id, None, 50.0, // TODO: adjustable
                                wall.id, door_pos,
                            );

                            let prims = door.draw(wall, DoorDrawOptions::empty());
                            for p in prims {
                                p.draw(ctx)
                            }
                        }
                    }
                }
            }
            EditMode::RemoveVertex => {
                if let Some(chamber) = control.state.active_chamber() {
                    match chamber.nearest_corner(control.state.cursor_world_pos()) {
                        Some((w1, w2)) => {
                            let color = Rgb {
                                r: 1.0,
                                g: 0.0,
                                b: 0.0,
                            };
                            Line {
                                from: w1.p1.into(),
                                to: w1.p2.into(),
                                color: color,
                                width: 3.0,
                            }
                            .draw(ctx);
                            Line {
                                from: w2.p1.into(),
                                to: w2.p2.into(),
                                color: color,
                                width: 3.0,
                            }
                            .draw(ctx);
                            Point {
                                at: w1.p2.into(),
                                color: color,
                            }
                            .draw(ctx)
                        }
                        None => {}
                    }
                }
            }
        }
    }

    fn motion(&self, control: Rc<RefCell<StateController>>, x: f64, y: f64) {
        let control = &mut *control.borrow_mut();
        control.state.cursor.set_pos(Vec2 { x: x, y: y });
        self.update();
    }

    fn click_select(&mut self, control: &mut StateController) -> Vec<StateCommand> {
        let door_id = control
            .state
            .dungeon
            .door_at(control.state.cursor_world_pos());
        if let Some(id) = door_id {
            vec![StateCommand::SelectDoor(Some(id))]
        } else {
            let chamber_id = control
                .state
                .dungeon
                .chamber_at(control.state.cursor_world_pos());
            vec![StateCommand::SelectChamber(chamber_id)]
        }
    }

    fn click_append_chamber(&mut self, control: &mut StateController) -> Vec<StateCommand> {
        if let Some(active_chamber_id) = control.state.active_chamber_id {
            if let Some(chamber) = control.state.dungeon.chamber_mut(active_chamber_id) {
                let chamber_id = chamber.id;
                return vec![StateCommand::AddVertexToChamber(
                    chamber_id,
                    control
                        .state
                        .grid
                        .snap((control.state.cursor_world_pos()).into()),
                )];
            }
        }
        vec![]
    }

    fn click_split_edge(&mut self, control: &mut StateController) -> Vec<StateCommand> {
        let selected_wall = self.selected_wall();
        match selected_wall {
            Some(wall_id) => {
                if let Some(active_chamber_id) = control.state.active_chamber_id {
                    if let Some(chamber) = control.state.dungeon.chamber_mut(active_chamber_id) {
                        let chamber_id = chamber.id;
                        self.set_selected_wall(None);
                        return vec![StateCommand::SplitWall(
                            chamber_id,
                            wall_id,
                            control
                                .state
                                .grid
                                .snap(control.state.cursor_world_pos().into()),
                        )];
                    }
                }
            }
            None => {
                self.select_nearest_wall(&control.state);
            }
        }
        return vec![];
    }

    fn click_add_door(&mut self, control: &mut StateController) -> Vec<StateCommand> {
        let selected_wall = self.selected_wall();
        match selected_wall {
            None => {
                self.select_nearest_wall(&control.state);
            }
            Some(wall_id) => {
                if let Some(chamber) = control.state.active_chamber() {
                    let wall = chamber.wall(wall_id).unwrap();
                    let door_pos = wall.nearest_relative_pos(control.state.cursor_world_pos());

                    let door = Door::new(
                        chamber.id, None, 50.0, // TODO: adjustable
                        wall.id, door_pos,
                    );
                    self.set_selected_wall(None);
                    return vec![StateCommand::AddDoor(door)];
                }
            }
        }
        return vec![];
    }

    fn click_remove_vertex(&mut self, control: &mut StateController) -> Vec<StateCommand> {
        if let Some(chamber) = control.state.active_chamber() {
            match chamber.nearest_corner(control.state.cursor_world_pos()) {
                Some((w1, _)) => return vec![StateCommand::CollapseWall(chamber.id, w1.id)],
                None => {}
            };
        }
        return vec![];
    }

    fn click(&mut self, control: Rc<RefCell<StateController>>) -> Vec<StateCommand> {
        let control = &mut *control.borrow_mut();
        let commands = match control.state.mode {
            EditMode::Select => self.click_select(control),
            EditMode::AppendChamber => self.click_append_chamber(control),
            EditMode::SplitEdge => self.click_split_edge(control),
            EditMode::AddDoor => self.click_add_door(control),
            EditMode::RemoveVertex => self.click_remove_vertex(control),
        };
        self.update();
        commands
    }

    fn drag_begin(&mut self) {
        self.set_last_pos(None)
    }

    fn drag_update(&mut self, control: Rc<RefCell<StateController>>, x: f64, y: f64) {
        let mut control = control.borrow_mut();
        let mut view_obj = control.state.view;
        let last = self.last_pos().unwrap_or(Vec2 { x: 0, y: 0 });
        let cur = Vec2 {
            x: x as i32,
            y: y as i32,
        };
        view_obj.move_view(last - cur);
        control.state.view = view_obj;
        self.set_last_pos(Some(cur));
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
