use cairo::Context;
use pango::{ffi::PANGO_SCALE, FontDescription};

use crate::dungeon::{self, Dungeon};

const PAGE_W: f64 = 595.0;
const PAGE_H: f64 = 842.0;
const START_H: f64 = 20.0;
const END_H: f64 = 800.0;
const LEFT_SPACE: f64 = 20.0;
const RIGHT_END: f64 = PAGE_W - 20.0;
const TEXT_WIDTH: f64 = RIGHT_END - LEFT_SPACE;

fn text_font() -> pango::FontDescription {
    let mut font = pango::FontDescription::default();
    font.set_size(10 * PANGO_SCALE);
    font
}

fn headline_font() -> pango::FontDescription {
    let mut font = pango::FontDescription::default();
    font.set_size(12 * PANGO_SCALE);
    font.set_weight(pango::Weight::Bold);
    font
}

fn layout_text() -> (pango::Context, pango::Layout) {
    let p_ctx = pango::Context::new();
    p_ctx.set_font_map(Some(&pangocairo::FontMap::default()));
    let layout = pango::Layout::new(&p_ctx);
    layout.set_width(TEXT_WIDTH as i32 * PANGO_SCALE);
    let mut font = text_font();
    layout.set_font_description(Some(&font));

    (p_ctx, layout)
}

fn layout_headline() -> (pango::Context, pango::Layout) {
    let p_ctx = pango::Context::new();
    p_ctx.set_font_map(Some(&pangocairo::FontMap::default()));
    let layout = pango::Layout::new(&p_ctx);
    layout.set_width(TEXT_WIDTH as i32 * PANGO_SCALE);
    let mut font = headline_font();
    layout.set_font_description(Some(&font));

    (p_ctx, layout)
}

pub fn to_pdf(dungeon: &Dungeon) {
    let pdf = gtk::cairo::PdfSurface::new(PAGE_W, PAGE_H, "foo.pdf").unwrap();
    let ctx = Context::new(pdf).unwrap();

    let mut cur_h = START_H;
    for room in dungeon.rooms() {
        // add headline
        {
            let (_, mut hl) = layout_headline();
            hl.set_text(&room.name);
            ctx.move_to(LEFT_SPACE, cur_h);
            pangocairo::show_layout(&ctx, &hl);
            let (extent, _) = hl.extents();
            // move cursor down
            cur_h += extent.height() as f64 / PANGO_SCALE as f64;
        }

        // TODO: Render Image here
        {
            cur_h += 120.0;
        }

        // add notes
        {
            let (_, mut tl) = layout_text();
            tl.set_text(&room.notes);
            ctx.move_to(LEFT_SPACE, cur_h);
            pangocairo::show_layout(&ctx, &tl);
            let (extent, _) = tl.extents();
            // move cursor down
            cur_h += extent.height() as f64 / PANGO_SCALE as f64;
        }

        // add horizontal line
        {
            cur_h += 20.0;
            ctx.move_to(LEFT_SPACE, cur_h);
            ctx.set_line_width(5.0);
            ctx.line_to(RIGHT_END, cur_h);
            ctx.stroke().unwrap();
            cur_h += 20.0
        }
        // ctx.show_page().unwrap();
    }
}
