use std::cmp::Ordering;

use sdl2::{
    event::Event,
    image::{InitFlag, LoadTexture},
    keyboard::Keycode,
    pixels::Color,
    render::RenderTarget,
};

use crate::notes::Note;
use crate::notes::NoteName;

const HALF_SPACE: i32 = 20;
const BOTTOM_LINE_Y: i32 = 249;
const N_MAX_LEDGER_LINES: u8 = 3;
const TREBLE_BOTTOM_NOTE: Note = Note { name: NoteName::E, alteration: 0, octave: 4 };

pub fn render_staff<T: RenderTarget>(
    staff: &sdl2::render::Texture,
    canvas: &mut sdl2::render::Canvas<T>,
) -> Result<(), String> {
    let pos = Position { x: 0, y: 0 };

    render_at(pos, staff, canvas)
}

pub fn render_note<T: RenderTarget>(
    note: Note,
    note_head: &sdl2::render::Texture,
    ledger_line: &sdl2::render::Texture,
    canvas: &mut sdl2::render::Canvas<T>,
) -> Result<(), String> {
    let staff_position: i32 = Note::diatonic_distance(TREBLE_BOTTOM_NOTE, note).into();
    let pos = Position { x: 420, y: BOTTOM_LINE_Y - staff_position * HALF_SPACE };

    render_ledger_line(pos.x, 1, ledger_line, canvas)?;
    render_ledger_line(pos.x, 2, ledger_line, canvas)?;
    render_at(pos, note_head, canvas)
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

    render_at(pos, ledger_line, canvas)
}

pub fn render_at<T: RenderTarget>(
    pos: Position,
    texture: &sdl2::render::Texture,
    canvas: &mut sdl2::render::Canvas<T>,
) -> Result<(), String> {
    let size = Dimension { w: texture.query().width, h: texture.query().height };
    let render_rect = sdl2::rect::Rect::new(pos.x, pos.y, size.w, size.h);
    canvas.copy(texture, None, Some(render_rect))?;
    Ok(())
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
