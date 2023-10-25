mod common;
mod view;

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use gtk::cairo::ffi::{cairo_fill, cairo_rectangle, cairo_set_source_rgb};
use gtk::prelude::*;
use gtk::DrawingArea;
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
        canvas.set_draw_func(move |area, ctx, w, h| {
            println!("{} {} {} {}", area, ctx, w, h);
            ctx.set_source_rgb(1.0, 0.8, 1.0);
            ctx.paint().unwrap();

            let world_min = view.get().world_min();
            let prims = grid.get().draw(world_min, world_min + Vec2 { x: w, y: h });

            for prim in prims {
                prim.draw(ctx)
            }

            // println!("Drawing rect");
            // ctx.set_line_width(19.0);
            // ctx.set_source_rgb(1.0, 0.0, 0.0);
            // ctx.rectangle(0.25, 0.25, 0.5, 0.5);
            // ctx.rectangle(
            //     0.1 * w as f64,
            //     0.5 * h as f64,
            //     0.5 * w as f64,
            //     0.5 * h as f64,
            // );
            // match ctx.stroke() {
            //     Err(x) => {
            //         println!("Error");
            //     }
            //     _ => {}
            // }
            // println!("Drawing rect");
        });
    }
    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&main_box)
        .build();

    // Present window
    window.present();
}
