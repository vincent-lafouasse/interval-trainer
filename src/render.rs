use sdl2::{
    event::Event,
    image::{InitFlag, LoadTexture},
    keyboard::Keycode,
    pixels::Color,
    render::RenderTarget,
};

use crate::notes::Note;

const HALF_SPACE: i32 = 20;
const BOTTOM_LINE_Y: i32 = 249;
const N_MAX_LEDGER_LINES: u8 = 3;

pub fn render_staff<T: RenderTarget>(
    staff: &sdl2::render::Texture,
    canvas: &mut sdl2::render::Canvas<T>,
) -> Result<(), String> {
    let pos = Position { x: 0, y: 0 };

    render_at(pos, staff, canvas)
}

type StaffPosition = i8;

pub fn render_note<T: RenderTarget>(
    note: Note,
    note_head: &sdl2::render::Texture,
    canvas: &mut sdl2::render::Canvas<T>,
) -> Result<(), String> {
    let staff_position = 2;
    let pos = Position { x: 420, y: BOTTOM_LINE_Y - staff_position * HALF_SPACE };

    render_at(pos, note_head, canvas)
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
