use std::cmp::Ordering;
use std::path::Path;

use sdl2::{image::LoadTexture, pixels::Color, render::RenderTarget};

use crate::music::note::{Note, NoteName};

const HALF_SPACE: i32 = 20;
const BOTTOM_LINE_Y: i32 = 249;
const N_MAX_LEDGER_LINES: u8 = 3;
const TREBLE_BOTTOM_NOTE: Note = Note { name: NoteName::E, alteration: 0, octave: 4 };

const WHITE: Color = Color::RGB(255, 255, 255);

const LEFT_X: i32 = 400;
const RIGHT_X: i32 = 750;

pub fn render_staff<T: RenderTarget>(
    note1: Option<Note>,
    note2: Option<Note>,
    sprites: &Sprites,
    canvas: &mut sdl2::render::Canvas<T>,
) -> Result<(), String> {
    canvas.set_draw_color(WHITE);
    canvas.clear();
    render_empty_staff(&sprites.staff, canvas)?;

    if let Some(note) = note1 {
        render_note(note, LEFT_X, sprites, canvas)?;
    }

    if let Some(note) = note2 {
        render_note(note, RIGHT_X, sprites, canvas)?;
    }

    Ok(())
}

fn render_empty_staff<T: RenderTarget>(
    staff: &sdl2::render::Texture,
    canvas: &mut sdl2::render::Canvas<T>,
) -> Result<(), String> {
    let pos = Position { x: 0, y: 0 };

    render_texture_at(staff, pos, canvas)
}

fn render_note<T: RenderTarget>(
    note: Note,
    x: i32,
    sprites: &Sprites,
    canvas: &mut sdl2::render::Canvas<T>,
) -> Result<(), String> {
    let staff_position: i32 = Note::diatonic_distance(TREBLE_BOTTOM_NOTE, note).into();
    let pos = Position { x, y: BOTTOM_LINE_Y - staff_position * HALF_SPACE };

    let ledgers = match staff_position {
        i32::MIN..=-2 => staff_position / 2,
        10..=i32::MAX => (staff_position - 8) / 2,
        _ => 0,
    };

    match ledgers {
        i32::MIN..=-1 => {
            for i in 1..=ledgers.abs() {
                render_ledger_line(pos.x, -i, &sprites.ledger_line, canvas)?;
            }
        }
        1..=i32::MAX => {
            for i in 1..=ledgers {
                render_ledger_line(pos.x, i, &sprites.ledger_line, canvas)?;
            }
        }
        0 => {}
    }

    render_alteration(note.alteration, x, staff_position, sprites, canvas)?;
    render_texture_at(&sprites.note_head, pos, canvas)
}

fn render_alteration<T: RenderTarget>(
    alteration: i8,
    x: i32,
    staff_position: i32,
    sprites: &Sprites,
    canvas: &mut sdl2::render::Canvas<T>,
) -> Result<(), String> {
    let pos = Position { x, y: BOTTOM_LINE_Y - staff_position * HALF_SPACE };
    match alteration {
        1 => render_texture_at(
            &sprites.sharp,
            Position { x: pos.x - 60, y: pos.y - 40 },
            canvas,
        ),
        2 => render_texture_at(
            &sprites.double_sharp,
            Position { x: pos.x - 65, y: pos.y },
            canvas,
        ),
        -1 => render_texture_at(
            &sprites.flat,
            Position { x: pos.x - 65, y: pos.y - 50 },
            canvas,
        ),
        -2 => render_texture_at(
            &sprites.double_flat,
            Position { x: pos.x - 90, y: pos.y - 50 },
            canvas,
        ),
        0 => Ok(()),
        _ => Err(String::from("no triple alterations allowed")),
    }
}

fn render_ledger_line<T: RenderTarget>(
    x: i32,
    staff_position: i32,
    ledger_line: &sdl2::render::Texture,
    canvas: &mut sdl2::render::Canvas<T>,
) -> Result<(), String> {
    let staff_position: i32 = match staff_position.cmp(&0) {
        Ordering::Equal => panic!("invalid legder line index: 0"),
        Ordering::Greater => 8 + 2 * staff_position,
        Ordering::Less => 2 * staff_position,
    };
    let pos = Position { x, y: BOTTOM_LINE_Y - staff_position * HALF_SPACE };
    let pos = Position { x: pos.x - 22, y: pos.y + 17 };

    render_texture_at(ledger_line, pos, canvas)
}

fn render_texture_at<T: RenderTarget>(
    texture: &sdl2::render::Texture,
    pos: Position,
    canvas: &mut sdl2::render::Canvas<T>,
) -> Result<(), String> {
    let size = Dimension { w: texture.query().width, h: texture.query().height };
    let render_rect = sdl2::rect::Rect::new(pos.x, pos.y, size.w, size.h);
    canvas.copy(texture, None, Some(render_rect))?;
    Ok(())
}

pub struct Sprites<'a> {
    pub staff: sdl2::render::Texture<'a>,
    pub note_head: sdl2::render::Texture<'a>,
    pub ledger_line: sdl2::render::Texture<'a>,
    pub sharp: sdl2::render::Texture<'a>,
    pub flat: sdl2::render::Texture<'a>,
    pub double_sharp: sdl2::render::Texture<'a>,
    pub double_flat: sdl2::render::Texture<'a>,
}

const PNG_DIR: &str = "src/assets/png";
const TREBLE_STAFF_PATH: &str = "treble_staff.png";
const NOTEHEAD_PATH: &str = "WholeNote.png";
const LEDGER_LINE_PATH: &str = "ledger_line.png";
const SHARP_PATH: &str = "Sharp.png";
const FLAT_PATH: &str = "Flat.png";
const DOUBLESHARP_PATH: &str = "DoubleSharp.png";
const DOUBLEFLAT_PATH: &str = "DoubleFlat.png";

impl<'a> Sprites<'a> {
    pub fn init<T>(texture_creator: &'a sdl2::render::TextureCreator<T>) -> Result<Self, String> {
        let png_dir = Path::new(PNG_DIR);
        let treble_staff = texture_creator.load_texture(&png_dir.join(TREBLE_STAFF_PATH))?;
        let note_head = texture_creator.load_texture(&png_dir.join(NOTEHEAD_PATH))?;
        let ledger_line = texture_creator.load_texture(&png_dir.join(LEDGER_LINE_PATH))?;
        let sharp = texture_creator.load_texture(&png_dir.join(SHARP_PATH))?;
        let flat = texture_creator.load_texture(&png_dir.join(FLAT_PATH))?;
        let double_sharp = texture_creator.load_texture(&png_dir.join(DOUBLESHARP_PATH))?;
        let double_flat = texture_creator.load_texture(&png_dir.join(DOUBLEFLAT_PATH))?;

        Ok(Self {
            staff: treble_staff,
            note_head,
            ledger_line,
            sharp,
            flat,
            double_sharp,
            double_flat,
        })
    }
}

pub struct Position {
    x: i32,
    y: i32,
}

pub struct Dimension {
    w: u32,
    h: u32,
}
