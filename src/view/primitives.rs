use crate::common::{BBox, Rgb, Vec2};
use gtk;

pub trait Primitive {
    fn draw(&self, ctx: &gtk::cairo::Context);
    // tight bounding box with (min, max) as output
    fn bbox(&self) -> BBox;
}

pub struct Point {
    pub at: Vec2<f64>,
    pub color: Rgb,
}

pub struct Circle {
    pub at: Vec2<f64>,
    pub width: f64,
    pub radius: f64,
    pub color: Rgb,
    pub dashed: bool,
}

pub struct Line {
    pub from: Vec2<f64>,
    pub to: Vec2<f64>,
    pub color: Rgb,
    pub width: f64,
    pub dashed: bool,
}

pub struct Polygon {
    pub points: Vec<Vec2<f64>>,
    pub fill_color: Rgb,
    pub fill_opacity: f64,
    pub stroke_color: Rgb,
    pub stroke_width: f64,
    pub dashed: bool,
}

pub struct Text {
    pub text: String,
    pub color: Rgb,
    pub at: Vec2<f64>,
    pub size: f64,
}

impl Primitive for Point {
    fn draw(&self, ctx: &gtk::cairo::Context) {
        ctx.set_source_rgb(self.color.r, self.color.g, self.color.b);
        ctx.arc(self.at.x, self.at.y, 10.0, 0.0, 2.0 * std::f64::consts::PI); // full circle
        ctx.fill().unwrap()
    }

    fn bbox(&self) -> BBox {
        BBox {
            min: Vec2 {
                x: self.at.x - 5.0,
                y: self.at.y - 5.0,
            },
            max: Vec2 {
                x: self.at.x + 5.0,
                y: self.at.y + 5.0,
            },
        }
    }
}

impl Primitive for Line {
    fn draw(&self, ctx: &gtk::cairo::Context) {
        if self.dashed {
            ctx.set_dash(&vec![20.0, 10.0], 0.0);
        } else {
            ctx.set_dash(&vec![], 0.0);
        }
        ctx.set_line_width(self.width);
        ctx.set_source_rgb(self.color.r, self.color.g, self.color.b);
        ctx.move_to(self.from.x, self.from.y);
        ctx.line_to(self.to.x, self.to.y);
        ctx.stroke().unwrap();
    }

    fn bbox(&self) -> BBox {
        BBox {
            min: Vec2 {
                x: f64::min(self.from.x, self.to.x),
                y: f64::min(self.from.y, self.to.y),
            },
            max: Vec2 {
                x: f64::max(self.from.x, self.to.x),
                y: f64::max(self.from.y, self.to.y),
            },
        }
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
        if self.dashed {
            ctx.set_dash(&vec![20.0, 10.0], 0.0);
        } else {
            ctx.set_dash(&vec![], 0.0);
        }
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
        ctx.set_dash(&vec![], 0.0);
    }

    fn bbox(&self) -> BBox {
        BBox {
            min: Vec2 {
                x: self
                    .points
                    .iter()
                    .map(|p| p.x)
                    .fold(f64::INFINITY, |a, b| a.min(b)),
                y: self
                    .points
                    .iter()
                    .map(|p| p.y)
                    .fold(f64::INFINITY, |a, b| a.min(b)),
            },
            max: Vec2 {
                x: self
                    .points
                    .iter()
                    .map(|p| p.x)
                    .fold(f64::NEG_INFINITY, |a, b| a.max(b)),
                y: self
                    .points
                    .iter()
                    .map(|p| p.y)
                    .fold(f64::NEG_INFINITY, |a, b| a.max(b)),
            },
        }
    }
}

impl Primitive for Text {
    fn draw(&self, ctx: &gtk::cairo::Context) {
        ctx.set_source_rgb(self.color.r, self.color.g, self.color.b);
        ctx.set_font_size(self.size);
        ctx.set_line_width(1.0);
        let ext = ctx.text_extents(&self.text).unwrap();
        ctx.move_to(
            self.at.x - ext.width() / 2.0,
            self.at.y + ext.height() / 2.0,
        );
        ctx.text_path(&self.text);
        ctx.fill().unwrap();

        // debug circle
        // ctx.set_source_rgba(1.0, 0.2, 0.2, 0.6);
        // ctx.arc(self.at.x, self.at.y, 10.0, 0.0, 2.0 * std::f64::consts::PI);
        // ctx.close_path();
        // ctx.fill().unwrap();
    }

    fn bbox(&self) -> BBox {
        // TODO
        BBox {
            min: self.at,
            max: self.at,
        }
    }
}

impl Primitive for Circle {
    fn draw(&self, ctx: &gtk::cairo::Context) {
        ctx.set_line_width(self.width);
        ctx.set_source_rgb(self.color.r, self.color.g, self.color.b);
        if self.dashed {
            ctx.set_dash(&vec![20.0, 10.0], 0.0);
        } else {
            ctx.set_dash(&vec![], 0.0);
        }
        ctx.arc(
            self.at.x,
            self.at.y,
            self.radius,
            0.0,
            2.0 * std::f64::consts::PI,
        ); // full circle
        ctx.stroke().unwrap();
    }

    fn bbox(&self) -> BBox {
        BBox {
            min: self.at
                - Vec2 {
                    x: self.radius,
                    y: self.radius,
                },
            max: self.at
                + Vec2 {
                    x: self.radius,
                    y: self.radius,
                },
        }
    }
}
