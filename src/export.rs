use cairo::Context;
use pango::ffi::PANGO_SCALE;

use crate::{
    chamber::{Chamber, ChamberDrawOptions},
    common::{BBox, Rgb, Vec2},
    door::{Door, DoorDrawOptions},
    dungeon::Dungeon,
    view::{grid::Grid, primitives::Primitive},
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

fn secondary_headline_font() -> pango::FontDescription {
    let mut font = pango::FontDescription::default();
    font.set_size(10 * PANGO_SCALE);
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

fn layout_secondary_headline() -> (pango::Context, pango::Layout) {
    let p_ctx = pango::Context::new();
    p_ctx.set_font_map(Some(&pangocairo::FontMap::default()));
    let layout = pango::Layout::new(&p_ctx);
    layout.set_width(TEXT_WIDTH as i32 * PANGO_SCALE);
    let font = secondary_headline_font();
    layout.set_font_description(Some(&font));

    (p_ctx, layout)
}

fn dungeon_to_primitives(dungeon: &Dungeon, include_hidden: bool) -> Vec<Box<dyn Primitive>> {
    let mut all_prims = vec![];
    for chamber in dungeon.chambers() {
        if include_hidden == false && chamber.hidden {
            continue;
        }

        let mut prims = chamber.draw(
            None,
            Some(ChamberDrawOptions {
                color: Some(Rgb {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                }),
                fill: Some(true),
            }),
        );
        all_prims.append(&mut prims)
    }

    // draw doors
    for door in dungeon.doors.iter() {
        if include_hidden == false && door.hidden {
            continue;
        }

        let mut prims = door.draw(
            dungeon
                .chamber(door.part_of)
                .unwrap()
                .wall(door.on_wall)
                .unwrap(),
            DoorDrawOptions {
                color: Some(Rgb {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                }),
            },
        );
        all_prims.append(&mut prims)
    }

    all_prims
}

fn prims_to_bbox(prims: &Vec<Box<dyn Primitive>>) -> BBox {
    // determine size of dungeon
    let mut bbox = BBox {
        min: Vec2 {
            x: f64::INFINITY,
            y: f64::INFINITY,
        },
        max: Vec2 {
            x: f64::NEG_INFINITY,
            y: f64::NEG_INFINITY,
        },
    };

    // determine bounding box
    for p in prims.iter() {
        bbox &= p.bbox()
    }
    bbox.min = bbox.min - Vec2 { x: 50.0, y: 50.0 };
    bbox.max = bbox.max + Vec2 { x: 50.0, y: 50.0 };

    bbox
}

fn draw_full_dungeon(dungeon: &Dungeon, ctx: &Context, include_hidden: bool) {
    let all_prims = dungeon_to_primitives(dungeon, include_hidden);
    let bbox = prims_to_bbox(&all_prims);

    let size = bbox.max - bbox.min;
    let max_scale_x = (RIGHT_END - LEFT_SPACE) / size.x;
    let max_scale_y = (END_H - START_H) / size.y;
    let scale = f64::min(max_scale_x, max_scale_y);
    ctx.translate(
        -bbox.min.x * scale + LEFT_SPACE,
        -bbox.min.y * scale + (((END_H - START_H) - (size.y * scale)) / 2.0),
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

    // draw chamber
    for prim in all_prims.iter() {
        prim.draw(&ctx)
    }

    ctx.reset_clip();
    ctx.identity_matrix();
    ctx.show_page().unwrap();
}

struct PdfElement {
    pub height: f64,
    pub draw: Box<dyn Fn(&Context, f64, &Dungeon, &Chamber)>,
}

fn chamber_headline(chamber: &Chamber) -> PdfElement {
    let (_, hl) = layout_headline();
    hl.set_text(&format!("{}: {}", chamber.id, &chamber.name));
    PdfElement {
        height: (hl.extents().0.height() as f64 / PANGO_SCALE as f64) + HEADLINE_IMAGE_SPACING,
        draw: Box::new(move |ctx, start_h, _, _| {
            ctx.set_source_rgba(HEADLINE_COLOR.r, HEADLINE_COLOR.g, HEADLINE_COLOR.b, 1.0);
            ctx.move_to(LEFT_SPACE, start_h);
            pangocairo::show_layout(&ctx, &hl);
        }),
    }
}

fn chamber_image(_chamber: &Chamber) -> PdfElement {
    let h = IMAGE_SIZE + IMAGE_NOTES_SPACEING;

    PdfElement {
        height: h,
        draw: Box::new(move |ctx, cur_h, dungeon, chamber| {
            let mut prims = chamber.draw(
                None,
                Some(ChamberDrawOptions {
                    color: Some(Rgb {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                    }),
                    fill: Some(true),
                }),
            );

            // draw doors
            for door in dungeon.chamber_doors(chamber.id).iter() {
                let mut door_prims = door.draw(
                    dungeon
                        .chamber(door.part_of)
                        .unwrap()
                        .wall(door.on_wall)
                        .unwrap(),
                    DoorDrawOptions {
                        color: Some(Rgb {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                        }),
                    },
                );
                prims.append(&mut door_prims)
            }

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

                // draw chamber
                for prim in prims.iter() {
                    prim.draw(&ctx)
                }

                ctx.reset_clip();
                ctx.identity_matrix();
            }
        }),
    }
}

fn chamber_notes(chamber: &Chamber) -> PdfElement {
    let (_, tl) = layout_text();
    tl.set_text(&chamber.notes);
    PdfElement {
        height: (tl.extents().0.height() as f64 / PANGO_SCALE as f64) + HEADLINE_IMAGE_SPACING,
        draw: Box::new(move |ctx, start_h, _, _| {
            ctx.set_source_rgba(NOTES_COLOR.r, NOTES_COLOR.g, NOTES_COLOR.b, 1.0);
            ctx.move_to(LEFT_SPACE, start_h);
            pangocairo::show_layout(&ctx, &tl);
        }),
    }
}

fn chamber_door(door: &Door) -> PdfElement {
    // pointless ot add empty doors to the pdf
    if door.name.is_empty() && door.notes.is_empty() {
        return PdfElement {
            height: 0.0,
            draw: Box::new(move |_, _, _, _| {}),
        };
    }

    let (_, hl) = layout_secondary_headline();
    match door.name.is_empty() {
        true => hl.set_text(&format!("Door: {}", door.id)),
        false => hl.set_text(&format!("Door: {}", door.name)),
    };
    let (_, tl) = layout_text();
    tl.set_text(&door.notes);
    PdfElement {
        height: ((hl.extents().0.height() as f64 / PANGO_SCALE as f64) * 1.5)
            + (tl.extents().0.height() as f64 / PANGO_SCALE as f64)
            + HEADLINE_IMAGE_SPACING,
        draw: Box::new(move |ctx, start_h, _, _| {
            let mut cur_h = start_h;
            ctx.set_source_rgba(HEADLINE_COLOR.r, HEADLINE_COLOR.g, HEADLINE_COLOR.b, 1.0);
            ctx.move_to(LEFT_SPACE, cur_h);
            pangocairo::show_layout(&ctx, &hl);

            cur_h += (hl.extents().0.height() as f64 / PANGO_SCALE as f64) * 1.5;

            ctx.set_source_rgba(NOTES_COLOR.r, NOTES_COLOR.g, NOTES_COLOR.b, 1.0);
            ctx.move_to(LEFT_SPACE, cur_h);
            pangocairo::show_layout(&ctx, &tl);
        }),
    }
}

fn chamber_separator(_chamber: &Chamber) -> PdfElement {
    PdfElement {
        height: 42.0,
        draw: Box::new(|ctx, start_h, _, _| {
            let mut cur_h = start_h;
            cur_h += 20.0;
            ctx.move_to(LEFT_SPACE, cur_h);
            ctx.set_line_width(2.0);
            ctx.line_to(RIGHT_END, cur_h);
            ctx.stroke().unwrap();
        }),
    }
}

fn chamber_elems(dungeon: &Dungeon, chamber: &Chamber) -> Vec<PdfElement> {
    let mut elems = vec![
        chamber_headline(chamber),
        chamber_image(chamber),
        chamber_notes(chamber),
    ];
    for e in dungeon
        .chamber_doors(chamber.id)
        .iter()
        .map(|d| chamber_door(d))
    {
        elems.push(e)
    }
    elems.push(chamber_separator(chamber));
    elems
}

pub fn to_pdf(dungeon: &Dungeon, path: String) {
    let pdf = gtk::cairo::PdfSurface::new(PAGE_W, PAGE_H, path).unwrap();
    let ctx = Context::new(pdf).unwrap();

    let mut cur_h = START_H;

    // Draw entire dungeon
    draw_full_dungeon(dungeon, &ctx, true);

    for chamber in dungeon.chambers() {
        // TODO: take care of large chambers and split over multiple pages.

        // for each chamber
        // emit list of unseparable elements
        // each element has an associated height
        // if element no longer fits on page: start new page
        let elems = chamber_elems(dungeon, chamber);
        for e in elems {
            let next_h = cur_h + (e.height);
            if next_h > END_H {
                ctx.show_page().unwrap();
                cur_h = START_H;
            }
            (e.draw)(&ctx, cur_h, dungeon, chamber);
            cur_h = cur_h + (e.height);
        }
    }
}

pub fn to_full_player_map_pdf(dungeon: &Dungeon, path: String) {
    // Draw entire dungeon
    let all_prims = dungeon_to_primitives(dungeon, false);
    let bbox = prims_to_bbox(&all_prims);

    let size = bbox.max - bbox.min;
    // determine if page should be horizontal or vertical
    let vertical = size.y > size.x;
    let (pdf, max_scale_x, max_scale_y) = if vertical {
        (
            gtk::cairo::PdfSurface::new(PAGE_W, PAGE_H, path).unwrap(),
            PAGE_W / size.x,
            PAGE_H / size.y,
        )
    } else {
        (
            gtk::cairo::PdfSurface::new(PAGE_H, PAGE_W, path).unwrap(),
            PAGE_H / size.x,
            PAGE_W / size.y,
        )
    };
    let scale = f64::min(max_scale_x, max_scale_y);

    let ctx = Context::new(pdf).unwrap();

    if vertical {
        ctx.translate(-bbox.min.x * scale, -bbox.min.y * scale);
    } else {
        ctx.translate(-bbox.min.x * scale, -bbox.min.y * scale);
    }
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

    // draw chamber
    for prim in all_prims.iter() {
        prim.draw(&ctx)
    }

    ctx.reset_clip();
    ctx.identity_matrix();
    ctx.show_page().unwrap();
}
