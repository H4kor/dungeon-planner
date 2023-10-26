mod common;
mod view;

use std::cell::Cell;
use std::rc::Rc;

use gtk::DrawingArea;
use gtk::{gdk, prelude::*};
use gtk::{glib, Application, ApplicationWindow, Box, Button};

use crate::common::Vec2;

const APP_ID: &str = "org.gtk_rs.HelloWorld1";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    let main_box = Box::builder().build();

    let canvas = DrawingArea::builder()
        .width_request(800)
        .height_request(600)
        .hexpand(true)
        .vexpand(true)
        .valign(gtk::Align::Fill)
        .halign(gtk::Align::Fill)
        .build();

    let button = Button::builder()
        .label("Press me!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    main_box.append(&button);
    main_box.append(&canvas);

    let view = Rc::new(Cell::new(view::View::new()));
    let grid = Rc::new(Cell::new(view::grid::Grid::new()));

    {
        let view = view.clone();
        let grid = grid.clone();
        canvas.set_draw_func(move |_area, ctx, w, h| {
            // fill with background color
            ctx.set_source_rgb(0.8, 0.95, 0.8);
            ctx.paint().unwrap();

            // apply "camera"
            let world_min = view.get().world_min();
            println!("view {} {}", world_min.x, world_min.y);
            ctx.translate(-world_min.x as f64, -world_min.y as f64);

            // draw grid
            let prims = grid.get().draw(world_min, world_min + Vec2 { x: w, y: h });
            for prim in prims {
                prim.draw(ctx)
            }

            // debug circle
            ctx.set_source_rgb(1.0, 0.0, 0.0);
            ctx.arc(200.0, 200.0, 20.0, 0.0, 2.0 * std::f64::consts::PI); // full circle
            ctx.fill().unwrap()
        });
    }

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&main_box)
        .build();

    let event_controller = gtk::EventControllerKey::new();

    {
        let view = view.clone();
        let canvas = canvas.clone();
        event_controller.connect_key_pressed(move |_, key, _, _| {
            let mut view_obj = view.get();
            const SPEED: i32 = 10;
            match key {
                gdk::Key::Right => view_obj.move_view(Vec2 { x: SPEED, y: 0 }),
                gdk::Key::Left => view_obj.move_view(Vec2 { x: -SPEED, y: 0 }),
                gdk::Key::Up => view_obj.move_view(Vec2 { x: 0, y: -SPEED }),
                gdk::Key::Down => view_obj.move_view(Vec2 { x: 0, y: SPEED }),
                _ => (),
            }
            view.set(view_obj);
            canvas.queue_draw();
            glib::Propagation::Proceed
        });
    }
    // Present window
    window.add_controller(event_controller);
    window.present();
}
