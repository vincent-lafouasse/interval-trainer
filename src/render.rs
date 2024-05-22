use sdl2::{
    event::Event,
    image::{InitFlag, LoadTexture},
    keyboard::Keycode,
    pixels::Color,
    render::RenderTarget,
};

pub fn render_staff<T: RenderTarget>(
    staff: &sdl2::render::Texture,
    canvas: &mut sdl2::render::Canvas<T>,
) -> Result<(), String> {
    let pos = Position { x: 0, y: 0 };

    render_at(pos, staff, canvas)
}

pub fn render_note<T: RenderTarget>(
    note_head: &sdl2::render::Texture,
    canvas: &mut sdl2::render::Canvas<T>,
) -> Result<(), String> {
    let pos = Position { x: 420, y: 69 };

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

pub struct Dimension {
    w: u32,
    h: u32,
}
