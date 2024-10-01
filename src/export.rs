use std::f64;

use cairo::Context;
use pango::ffi::PANGO_SCALE;
use pangocairo::functions::{show_layout, show_layout_line};

use crate::{
    chamber::{Chamber, ChamberDrawOptions},
    common::{BBox, Rgb, Vec2},
    door::{Door, DoorDrawOptions},
    dungeon::Dungeon,
    object::{Object, ObjectDrawOptions},
    view::{grid::Grid, primitives::Primitive},
};

const PAGE_W: f64 = 595.0;
const PAGE_H: f64 = 842.0;
const EDGE_SPACING: f64 = 15.0;
const SIZE_PAGE_NUMBER: f64 = 12.0;
const HEIGHT_PAGE_NUMBER: f64 = PAGE_H - EDGE_SPACING * 2.0;
const START_H: f64 = EDGE_SPACING * 2.0;
const END_H: f64 = PAGE_H - (EDGE_SPACING * 2.0) - SIZE_PAGE_NUMBER;
const LEFT_SPACE: f64 = EDGE_SPACING;
const RIGHT_END: f64 = PAGE_W - EDGE_SPACING;
const TEXT_WIDTH: f64 = RIGHT_END - LEFT_SPACE;
const HEADLINE_IMAGE_SPACING: f64 = 12.0;
const IMAGE_NOTES_SPACEING: f64 = 24.0;
const TITLE_SPACING: f64 = 16.0;
const TEXT_SPACING: f64 = 12.0;
const IMAGE_SIZE: f64 = 120.0;

const TEXT_FONT_SIZE: i32 = 10;
const TEXT_LINE_SPACING: f64 = 1.5;

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

fn title_font() -> pango::FontDescription {
    let mut font = pango::FontDescription::default();
    font.set_size(24 * PANGO_SCALE);
    font.set_weight(pango::Weight::Bold);
    font
}

fn text_font() -> pango::FontDescription {
    let mut font = pango::FontDescription::default();
    font.set_size(TEXT_FONT_SIZE * PANGO_SCALE);
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

fn page_number_font() -> pango::FontDescription {
    let mut font = pango::FontDescription::default();
    font.set_size(8 * PANGO_SCALE);
    font
}

fn layout_title() -> (pango::Context, pango::Layout) {
    let p_ctx = pango::Context::new();
    p_ctx.set_font_map(Some(&pangocairo::FontMap::default()));
    let layout = pango::Layout::new(&p_ctx);
    layout.set_width(TEXT_WIDTH as i32 * PANGO_SCALE);
    let font = title_font();
    layout.set_font_description(Some(&font));
    layout.set_alignment(pango::Alignment::Center);

    (p_ctx, layout)
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

fn layout_page_number() -> (pango::Context, pango::Layout) {
    let p_ctx = pango::Context::new();
    p_ctx.set_font_map(Some(&pangocairo::FontMap::default()));
    let layout = pango::Layout::new(&p_ctx);
    layout.set_width(TEXT_WIDTH as i32 * PANGO_SCALE);
    let font = page_number_font();
    layout.set_font_description(Some(&font));
    layout.set_alignment(pango::Alignment::Right);

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

    // draw objects
    for object in dungeon.objects.iter() {
        // hide if object is hidden
        if include_hidden == false && object.hidden {
            continue;
        }
        // hide if chamber of object is hidden
        if let Some(chamber_id) = object.part_of {
            if dungeon.chamber(chamber_id).unwrap().hidden {
                continue;
            }
        }

        let mut prims = object.draw(ObjectDrawOptions {
            color: Some(Rgb {
                r: 0.0,
                g: 0.0,
                b: 0.0,
            }),
        });
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

fn draw_chamber(dungeon: &Dungeon, chamber: &Chamber, cur_h: f64, max_size: Vec2<f64>, ctx: &Context, include_hidden: bool) -> f64 {
    if include_hidden == false && chamber.hidden {
        return 0.0;
    }
    // Draw Image
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
        if include_hidden == false && door.hidden {
            continue;
        }
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

    // draw objects
    for obj in dungeon.chamber_objects(chamber.id).iter() {
        if include_hidden == false && obj.hidden {
            continue;
        }
        let mut door_prims = obj.draw(ObjectDrawOptions {
            color: Some(Rgb {
                r: 0.0,
                g: 0.0,
                b: 0.0,
            }),
        });
        prims.append(&mut door_prims)
    }

    if !prims.is_empty() {
        let mut bbox = prims[0].bbox();
        for p in prims.iter() {
            bbox &= p.bbox()
        }
        bbox.min = bbox.min - Vec2 { x: 50.0, y: 50.0 };
        bbox.max = bbox.max + Vec2 { x: 50.0, y: 50.0 };

        if bbox.is_valid() {
            let size = bbox.max - bbox.min;
            let scale = f64::min(max_size.x/size.x, max_size.y/size.y);
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
            return bbox.size().y * scale
        } else {
            return 0.0;
        }
    }
    return 0.0;
}


fn draw_full_dungeon(dungeon: &Dungeon, ctx: &Context, include_hidden: bool) {
    let all_prims = dungeon_to_primitives(dungeon, include_hidden);
    let bbox = prims_to_bbox(&all_prims);
    // early abort on empty dungeon
    if !bbox.is_valid() {
        return;
    }

    let mut cur_h = START_H;
    let (_, tl) = layout_title();
    tl.set_text(&dungeon.name);
    ctx.set_source_rgba(HEADLINE_COLOR.r, HEADLINE_COLOR.g, HEADLINE_COLOR.b, 1.0);
    ctx.move_to(LEFT_SPACE, cur_h);
    show_layout(&ctx, &tl);
    cur_h += (tl.extents().0.height() as f64 / PANGO_SCALE as f64) + TITLE_SPACING;

    let size = bbox.max - bbox.min;
    let max_scale_x = (RIGHT_END - LEFT_SPACE) / size.x;
    let max_scale_y = (END_H - cur_h) / size.y;
    let scale = f64::min(max_scale_x, max_scale_y);
    ctx.translate(
        -bbox.min.x * scale + LEFT_SPACE,
        -bbox.min.y * scale + (((END_H - cur_h) - (size.y * scale)) / 2.0),
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

/**
 * Combination of chamber headline and image
 * This is combined to avoid a Headline add the end of the page without further info.
 */
fn chamber_headline(chamber: &Chamber) -> PdfElement {
    let (_, hl) = layout_headline();
    hl.set_text(&format!("{}: {}", chamber.id, &chamber.name));

    let headline_height =
        (hl.extents().0.height() as f64 / PANGO_SCALE as f64) + HEADLINE_IMAGE_SPACING;
    let image_height = IMAGE_SIZE + IMAGE_NOTES_SPACEING;

    PdfElement {
        height: headline_height + image_height,
        draw: Box::new(move |ctx, start_h, dungeon, chamber| {
            let mut cur_h = start_h;
            // Draw Headline
            ctx.set_source_rgba(HEADLINE_COLOR.r, HEADLINE_COLOR.g, HEADLINE_COLOR.b, 1.0);
            ctx.move_to(LEFT_SPACE, start_h);
            show_layout(&ctx, &hl);

            cur_h += headline_height;
            draw_chamber(dungeon, chamber, cur_h, Vec2{x: IMAGE_SIZE, y: IMAGE_SIZE}, ctx, true);
        }),
    }
}

fn str_to_pdf_elements(str: String) -> Vec<PdfElement> {
    let (_, tl) = layout_text();
    tl.set_text(&str);
    let lines = tl.lines();
    lines
        .into_iter()
        .map(move |l| PdfElement {
            height: (l.extents().0.height() as f64 / PANGO_SCALE as f64).max(TEXT_FONT_SIZE as f64)
                * TEXT_LINE_SPACING,
            draw: Box::new(move |ctx, start_h, _, _| {
                ctx.set_source_rgba(NOTES_COLOR.r, NOTES_COLOR.g, NOTES_COLOR.b, 1.0);
                ctx.move_to(LEFT_SPACE, start_h);
                show_layout_line(&ctx, &l);
            }),
        })
        .collect()
}

fn chamber_notes(chamber: &Chamber) -> Vec<PdfElement> {
    str_to_pdf_elements(chamber.notes.clone())
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
            show_layout(&ctx, &hl);

            cur_h += (hl.extents().0.height() as f64 / PANGO_SCALE as f64) * 1.5;

            ctx.set_source_rgba(NOTES_COLOR.r, NOTES_COLOR.g, NOTES_COLOR.b, 1.0);
            ctx.move_to(LEFT_SPACE, cur_h);
            show_layout(&ctx, &tl);
        }),
    }
}

fn chamber_object(object: &Object) -> PdfElement {
    // pointless ot add empty objects to the pdf
    if object.name.is_empty() && object.notes.is_empty() {
        return PdfElement {
            height: 0.0,
            draw: Box::new(move |_, _, _, _| {}),
        };
    }

    let (_, hl) = layout_secondary_headline();
    match object.name.is_empty() {
        true => hl.set_text(&format!("Object: {}", object.id)),
        false => hl.set_text(&format!("Object: {}", object.name)),
    };
    let (_, tl) = layout_text();
    tl.set_text(&object.notes);
    PdfElement {
        height: ((hl.extents().0.height() as f64 / PANGO_SCALE as f64) * 1.5)
            + (tl.extents().0.height() as f64 / PANGO_SCALE as f64)
            + HEADLINE_IMAGE_SPACING,
        draw: Box::new(move |ctx, start_h, _, _| {
            let mut cur_h = start_h;
            ctx.set_source_rgba(HEADLINE_COLOR.r, HEADLINE_COLOR.g, HEADLINE_COLOR.b, 1.0);
            ctx.move_to(LEFT_SPACE, cur_h);
            show_layout(&ctx, &hl);

            cur_h += (hl.extents().0.height() as f64 / PANGO_SCALE as f64) * 1.5;

            ctx.set_source_rgba(NOTES_COLOR.r, NOTES_COLOR.g, NOTES_COLOR.b, 1.0);
            ctx.move_to(LEFT_SPACE, cur_h);
            show_layout(&ctx, &tl);
        }),
    }
}

fn separator() -> PdfElement {
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
    let mut elems = vec![chamber_headline(chamber)];
    elems.append(&mut chamber_notes(chamber));
    for e in dungeon
        .chamber_doors(chamber.id)
        .iter()
        .map(|d| chamber_door(d))
    {
        elems.push(e)
    }
    for e in dungeon
        .chamber_objects(chamber.id)
        .iter()
        .map(|o| chamber_object(o))
    {
        elems.push(e)
    }

    elems.push(separator());
    elems
}

fn finalize_page(ctx: &Context, cur_page_number: i32) {
    // add page number to page
    let (_, pl) = layout_page_number();
    pl.set_text(&format!("{}", cur_page_number));
    ctx.set_source_rgba(NOTES_COLOR.r, NOTES_COLOR.g, NOTES_COLOR.b, 1.0);
    ctx.move_to(LEFT_SPACE, HEIGHT_PAGE_NUMBER);
    show_layout(&ctx, &pl);
}

pub fn to_pdf(dungeon: &Dungeon, path: String) {
    let pdf = gtk::cairo::PdfSurface::new(PAGE_W, PAGE_H, path).unwrap();
    let ctx = Context::new(pdf).unwrap();

    let mut cur_h = START_H;
    let mut cur_page_number = 1;

    // Draw entire dungeon
    draw_full_dungeon(dungeon, &ctx, true);

    let (_, hl) = layout_headline();
    hl.set_text(&dungeon.name);
    ctx.set_source_rgba(HEADLINE_COLOR.r, HEADLINE_COLOR.g, HEADLINE_COLOR.b, 1.0);
    ctx.move_to(LEFT_SPACE, cur_h);
    show_layout(&ctx, &hl);
    cur_h += (hl.extents().0.height() as f64 / PANGO_SCALE as f64) + TEXT_SPACING;

    let mut dungeon_elems = str_to_pdf_elements(dungeon.notes.clone());
    dungeon_elems.push(separator());

    for e in dungeon_elems {
        let next_h = cur_h + (e.height);
        if next_h > END_H {
            finalize_page(&ctx, cur_page_number);
            cur_page_number += 1;

            // start new page
            ctx.show_page().unwrap();
            cur_h = START_H;
        }
        (e.draw)(&ctx, cur_h, dungeon, &Chamber::new()); // TODO this is hacky. Chamber not needed
        cur_h = cur_h + (e.height);
    }

    for chamber in dungeon.chambers() {
        // for each chamber
        // emit list of unseparable elements
        // each element has an associated height
        // if element no longer fits on page: start new page
        let elems = chamber_elems(dungeon, chamber);
        for e in elems {
            let next_h = cur_h + (e.height);
            if next_h > END_H {
                finalize_page(&ctx, cur_page_number);
                cur_page_number += 1;

                // start new page
                ctx.show_page().unwrap();
                cur_h = START_H;
            }
            (e.draw)(&ctx, cur_h, dungeon, chamber);
            cur_h = cur_h + (e.height);
        }
    }
    // add page number to last page
    finalize_page(&ctx, cur_page_number);
}

pub fn to_player_cutout_pdf(dungeon: &Dungeon, path: String) {
    // find max bbox size
    let max_size = dungeon.chambers().iter().fold(Vec2{
        x: f64::MIN,
        y: f64::MIN,
    }, |prev, chamber| {
        let bbox = chamber.bbox();
        Vec2{
            x: prev.x.max(bbox.max.x-bbox.min.x),
            y: prev.y.max(bbox.max.y-bbox.min.y),
        }
    });
    let pdf = gtk::cairo::PdfSurface::new(PAGE_W, PAGE_H, path).unwrap();
    let scale = (PAGE_W - (2. * EDGE_SPACING)) / max_size.x;
    let ctx = Context::new(pdf).unwrap();

    let mut cur_h = START_H;
    let mut cur_page_number = 1;
    for chamber in dungeon.chambers() {
        let next_h = cur_h + chamber.bbox().size().y * scale + 12.0;
        if next_h > END_H {
            finalize_page(&ctx, cur_page_number);
            cur_page_number += 1;
            // start new page
            ctx.show_page().unwrap();
            cur_h = START_H;
        }
        cur_h += draw_chamber(dungeon, chamber, cur_h, scale * chamber.bbox().size(), &ctx, false) + 12.0;
    }
    // add page number to last page
    finalize_page(&ctx, cur_page_number);
}

pub fn to_full_player_map_pdf(dungeon: &Dungeon, path: String) {
    // Draw entire dungeon
    let all_prims = dungeon_to_primitives(dungeon, false);
    // early abort of dungeon is empty (nothing to draw)
    if all_prims.len() == 0 {
        return;
    }
    let bbox = prims_to_bbox(&all_prims);
    if !bbox.is_valid() {
        return;
    }

    let size = bbox.max - bbox.min;
    // determine if page should be horizontal or vertical
    let vertical = size.y > size.x;
    let (pdf, max_scale_x, max_scale_y) = if vertical {
        (
            gtk::cairo::PdfSurface::new(PAGE_W, PAGE_H, path).unwrap(),
            (PAGE_W - (2. * EDGE_SPACING)) / size.x,
            (PAGE_H - (2. * EDGE_SPACING)) / size.y,
        )
    } else {
        (
            gtk::cairo::PdfSurface::new(PAGE_H, PAGE_W, path).unwrap(),
            (PAGE_H - (2. * EDGE_SPACING)) / size.x,
            (PAGE_W - (2. * EDGE_SPACING)) / size.y,
        )
    };
    let scale = f64::min(max_scale_x, max_scale_y);

    let ctx = Context::new(pdf).unwrap();

    if vertical {
        ctx.translate(
            -bbox.min.x * scale + EDGE_SPACING,
            -bbox.min.y * scale + EDGE_SPACING,
        );
    } else {
        ctx.translate(
            -bbox.min.x * scale + EDGE_SPACING,
            -bbox.min.y * scale + EDGE_SPACING,
        );
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

#[cfg(test)]
mod test {
    use crate::{chamber::Chamber, dungeon::Dungeon};

    use super::{to_full_player_map_pdf, to_pdf, to_player_cutout_pdf};

    #[test]
    fn test_to_full_player_map_pdf_empty() {
        let dungeon = &Dungeon::new();
        to_full_player_map_pdf(
            dungeon,
            "/tmp/test_to_full_player_map_pdf_empty.pdf".to_string(),
        )
    }
    #[test]
    fn test_to_full_player_map_pdf_empty_chamber() {
        let mut dungeon = Dungeon::new();
        let chamber = Chamber::new();
        dungeon.add_chamber(chamber);
        to_full_player_map_pdf(
            &dungeon,
            "/tmp/test_to_full_player_map_pdf_empty_chamber.pdf".to_string(),
        )
    }

    #[test]
    fn test_to_pdf_empty() {
        let dungeon = &Dungeon::new();
        to_pdf(dungeon, "/tmp/test_to_pdf_empty.pdf".to_string())
    }
    #[test]
    fn test_to_pdf_empty_chamber() {
        let mut dungeon = Dungeon::new();
        let chamber = Chamber::new();
        dungeon.add_chamber(chamber);
        to_pdf(&dungeon, "/tmp/test_to_pdf_empty_chamber.pdf".to_string())
    }
    
    #[test]
    fn test_to_player_cutout_pdf_empty() {
        let dungeon = &Dungeon::new();
        to_player_cutout_pdf(dungeon, "/tmp/test_to_player_cutout_pdf_empty.pdf".to_string())
    }
    #[test]
    fn test_to_player_cutout_pdf_empty_chamber() {
        let mut dungeon = Dungeon::new();
        let chamber = Chamber::new();
        dungeon.add_chamber(chamber);
        to_player_cutout_pdf(&dungeon, "/tmp/test_to_player_cutout_pdf_empty_chamber.pdf".to_string())
    }
}
