use cairo::Context;
use pango::ffi::PANGO_SCALE;

use crate::{
    common::{Rgb, Vec2},
    dungeon::Dungeon,
    room::RoomDrawOptions,
    view::grid::Grid,
};

const PAGE_W: f64 = 595.0;
const PAGE_H: f64 = 842.0;
const START_H: f64 = 20.0;
const END_H: f64 = PAGE_H - 20.0;
const LEFT_SPACE: f64 = 20.0;
const RIGHT_END: f64 = PAGE_W - 20.0;
const TEXT_WIDTH: f64 = RIGHT_END - LEFT_SPACE;
const HEADLINE_IMAGE_SPACING: f64 = 12.0;
const IMAGE_NOTES_SPACEING: f64 = 12.0;
const IMAGE_SIZE: f64 = 120.0;
const HEADLINE_COLOR: Rgb = Rgb {
    r: 0.0,
    g: 0.0,
    b: 0.0,
};
const NOTES_COLOR: Rgb = Rgb {
    r: 0.0,
    g: 0.0,
    b: 0.0,
};

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
    let font = text_font();
    layout.set_font_description(Some(&font));

    (p_ctx, layout)
}

fn layout_headline() -> (pango::Context, pango::Layout) {
    let p_ctx = pango::Context::new();
    p_ctx.set_font_map(Some(&pangocairo::FontMap::default()));
    let layout = pango::Layout::new(&p_ctx);
    layout.set_width(TEXT_WIDTH as i32 * PANGO_SCALE);
    let font = headline_font();
    layout.set_font_description(Some(&font));

    (p_ctx, layout)
}

pub fn to_pdf(dungeon: &Dungeon) {
    let pdf = gtk::cairo::PdfSurface::new(PAGE_W, PAGE_H, "foo.pdf").unwrap();
    let ctx = Context::new(pdf).unwrap();

    let mut cur_h = START_H;
    for room in dungeon.rooms() {
        // TODO: take care of large rooms and split over multiple pages.

        // prepare elements
        let (_, hl) = layout_headline();
        hl.set_text(&room.name);
        let (_, tl) = layout_text();
        tl.set_text(&room.notes);

        // calculate total height
        let (h_extent, _) = hl.extents();
        let (t_extent, _) = tl.extents();
        let total_h = h_extent.height() as f64 / PANGO_SCALE as f64
            + HEADLINE_IMAGE_SPACING
            + IMAGE_SIZE
            + IMAGE_NOTES_SPACEING
            + t_extent.height() as f64 / PANGO_SCALE as f64
            + 25.0;

        if total_h + cur_h > END_H {
            ctx.show_page().unwrap();
            cur_h = START_H;
        }

        // add headline
        {
            ctx.set_source_rgba(HEADLINE_COLOR.r, HEADLINE_COLOR.g, HEADLINE_COLOR.b, 1.0);
            ctx.move_to(LEFT_SPACE, cur_h);
            pangocairo::show_layout(&ctx, &hl);
            let (extent, _) = hl.extents();
            // move cursor down
            cur_h += extent.height() as f64 / PANGO_SCALE as f64;
        }

        // Render Image
        {
            cur_h += HEADLINE_IMAGE_SPACING;
            let prims = room.draw(
                None,
                false,
                Some(RoomDrawOptions {
                    color: Some(Rgb {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                    }),
                    fill: Some(true),
                }),
            );
            if !prims.is_empty() {
                let mut bbox = prims[0].bbox();
                for p in prims.iter() {
                    bbox &= p.bbox()
                }
                bbox.min = bbox.min - Vec2 { x: 50.0, y: 50.0 };
                bbox.max = bbox.max + Vec2 { x: 50.0, y: 50.0 };

                let size = bbox.max - bbox.min;
                let scale = IMAGE_SIZE / f64::max(size.x, size.y);
                ctx.translate(
                    -bbox.min.x * scale + LEFT_SPACE,
                    -bbox.min.y * scale + cur_h,
                );
                ctx.scale(scale, scale);

                let mut grid = Grid::new();
                grid.color = Rgb {
                    r: 0.5,
                    g: 0.5,
                    b: 0.5,
                };
                grid.width = 1.0;

                // set clipping
                ctx.rectangle(bbox.min.x, bbox.min.y, size.x, size.y);
                ctx.clip();
                ctx.new_path();

                // draw grid
                ctx.set_dash(&vec![10.0, 10.0], 0.0);
                for prim in grid.draw(bbox.min.into(), bbox.max.into()) {
                    prim.draw(&ctx)
                }
                ctx.set_dash(&vec![], 0.0);

                // draw room
                for prim in prims.iter() {
                    prim.draw(&ctx)
                }

                ctx.reset_clip();
                ctx.identity_matrix();
                cur_h += size.y * scale;
            }
            cur_h += IMAGE_NOTES_SPACEING;
        }

        // add notes
        {
            ctx.set_source_rgba(NOTES_COLOR.r, NOTES_COLOR.g, NOTES_COLOR.b, 1.0);
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
            ctx.set_line_width(2.0);
            ctx.line_to(RIGHT_END, cur_h);
            ctx.stroke().unwrap();
            cur_h += 20.0
        }
        // ctx.show_page().unwrap();
    }
}
