use crate::common::{Rgb, Vec2};
use gtk;

pub trait Primitive {
    fn draw(&self, ctx: &gtk::cairo::Context);
}

pub struct Line {
    pub from: Vec2<f64>,
    pub to: Vec2<f64>,
    pub color: Rgb,
    pub width: f64,
}

pub struct Polygon {
    pub points: Vec<Vec2<f64>>,
    pub fill_color: Rgb,
    pub fill_opacity: f64,
    pub stroke_color: Rgb,
    pub stroke_width: f64,
}

impl Primitive for Line {
    fn draw(&self, ctx: &gtk::cairo::Context) {
        ctx.set_line_width(self.width);
        ctx.set_source_rgb(self.color.r, self.color.g, self.color.b);
        ctx.move_to(self.from.x, self.from.y);
        ctx.line_to(self.to.x, self.to.y);
        ctx.stroke().unwrap();
    }
}

impl Primitive for Polygon {
    fn draw(&self, ctx: &gtk::cairo::Context) {
        if self.points.len() < 2 {
            return;
        }
        ctx.set_source_rgba(
            self.fill_color.r,
            self.fill_color.g,
            self.fill_color.b,
            self.fill_opacity,
        );
        ctx.move_to(self.points[0].x, self.points[0].y);
        for p in self.points[1..].iter() {
            ctx.line_to(p.x, p.y);
        }
        ctx.close_path();
        ctx.fill_preserve().unwrap();
        ctx.set_line_join(gtk::cairo::LineJoin::Bevel);
        ctx.set_line_width(self.stroke_width);
        ctx.set_source_rgb(
            self.stroke_color.r,
            self.stroke_color.g,
            self.stroke_color.b,
        );
        ctx.stroke().unwrap();
    }
}
