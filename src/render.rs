use std::cmp::Ordering;
use std::path::Path;

use sdl2::{
    event::Event,
    image::{InitFlag, LoadTexture},
    keyboard::Keycode,
    pixels::Color,
    render::RenderTarget,
};

use crate::music::note::{Note, NoteName};

const HALF_SPACE: i32 = 20;
const BOTTOM_LINE_Y: i32 = 249;
const N_MAX_LEDGER_LINES: u8 = 3;
const TREBLE_BOTTOM_NOTE: Note = Note { name: NoteName::E, alteration: 0, octave: 4 };

pub fn render_staff<T: RenderTarget>(
    staff: &sdl2::render::Texture,
    canvas: &mut sdl2::render::Canvas<T>,
) -> Result<(), String> {
    let pos = Position { x: 0, y: 0 };

    render_texture_at(staff, pos, canvas)
}

pub fn render_note<T: RenderTarget>(
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

    render_texture_at(&sprites.note_head, pos, canvas)
}

pub fn render_ledger_line<T: RenderTarget>(
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

pub fn render_texture_at<T: RenderTarget>(
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
}

impl<'a> Sprites<'a> {
    pub fn init<T>(texture_creator: &'a sdl2::render::TextureCreator<T>) -> Result<Self, String> {
        let png_dir = Path::new("src/assets/png");
        let treble_staff = texture_creator.load_texture(&png_dir.join("treble_staff.png"))?;
        let note_head = texture_creator.load_texture(&png_dir.join("WholeNote.png"))?;
        let ledger_line = texture_creator.load_texture(&png_dir.join("ledger_line.png"))?;

        Ok(Self { staff: treble_staff, note_head, ledger_line })
    }
}

pub enum Side {
    Right,
    Left,
}

pub struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn shift_x(&self, amount: i32) -> Self {
        Self { x: self.x + amount, y: self.y }
    }

    fn shift_y(&self, amount: i32) -> Self {
        Self { x: self.x, y: self.y + amount }
    }
}

pub struct Dimension {
    w: u32,
    h: u32,
}
