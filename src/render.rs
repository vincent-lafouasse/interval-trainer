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
    let size = Dimension { w: staff.query().width, h: staff.query().height };
    let pos = Position { x: 0, y: 0 };
    let render_rect = sdl2::rect::Rect::new(pos.x, pos.y, size.w, size.h);
    canvas.copy(staff, None, Some(render_rect))?;
    Ok(())
}

pub fn render_note<T: RenderTarget>(
    note_head: &sdl2::render::Texture,
    canvas: &mut sdl2::render::Canvas<T>,
) -> Result<(), String> {
    let size = Dimension { w: note_head.query().width, h: note_head.query().height };
    let pos = Position { x: 420, y: 69 };
    let render_rect = sdl2::rect::Rect::new(pos.x, pos.y, size.w, size.h);
    canvas.copy(note_head, None, Some(render_rect))?;
    Ok(())
}

struct Position {
    x: i32,
    y: i32,
}

struct Dimension {
    w: u32,
    h: u32,
}
