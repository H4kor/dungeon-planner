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

impl Primitive for Line {
    fn draw(&self, ctx: &gtk::cairo::Context) {
        ctx.set_line_width(self.width);
        ctx.set_source_rgb(self.color.r, self.color.g, self.color.b);
        ctx.move_to(self.from.x, self.from.y);
        ctx.line_to(self.to.x, self.to.y);
        ctx.stroke().unwrap();
    }
}
