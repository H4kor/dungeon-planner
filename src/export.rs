use cairo::Context;

use crate::dungeon::Dungeon;

pub fn to_pdf(_dungeon: &Dungeon) {
    let pdf = gtk::cairo::PdfSurface::new(595.0, 842.0, "foo.pdf").unwrap();
    let ctx = Context::new(pdf).unwrap();
    ctx.set_line_width(1.0);
    ctx.set_source_rgb(0.0, 0.0, 0.0);
    ctx.move_to(10.0, 10.0);
    ctx.set_font_size(8.0);
    ctx.show_text("Duneon Planner").unwrap();
}
