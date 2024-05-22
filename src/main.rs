#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

//! # A cool ear trainer

mod interval;
mod listen;
mod note_range;
mod notes;
mod render;
mod simple_note;
mod synth;
mod wavetables;

use std::{
    path::Path,
    sync::mpsc,
    sync::mpsc::{Receiver, Sender},
    thread,
    time::Duration,
};

use sdl2::{
    event::Event,
    image::{InitFlag, LoadTexture},
    keyboard::Keycode,
    pixels::Color,
    render::RenderTarget,
};

use crate::{
    interval::{Direction, Interval},
    listen::listen_for_note_in_thread,
    note_range::NoteRange,
    notes::Note,
    render::{render_note, render_staff},
    simple_note::SimpleNote,
    synth::play_notes_in_thread,
};

struct IntervalTrainer {
    scene: Scene,
    range: NoteRange,
}

impl IntervalTrainer {
    fn init(range: NoteRange) -> Self {
        Self { scene: Scene::Idle, range }
    }
}

#[derive(Default, Copy, Clone, Debug)]
enum Scene {
    #[default]
    Idle,
    PlayingSound(Note, Note),
    Listening(SimpleNote),
    Concluding,
}

const WHITE: Color = Color::RGB(255, 255, 255);

const WINDOW_WIDTH: u32 = 1000;
const WINDOW_HEIGHT: u32 = 400;

const SAMPLE_RATE: u16 = 44_100;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(InitFlag::PNG)?;

    let window = video_subsystem
        .window("Interval Trainer", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .software()
        .build()
        .map_err(|e| e.to_string())?;

    let texture_creator = canvas.texture_creator();
    let png_dir = Path::new("src/assets/png");
    let treble_staff_path = png_dir.join("treble_staff.png");
    let notehead_path = png_dir.join("WholeNote.png");
    let treble_staff = texture_creator.load_texture(&treble_staff_path)?;
    let note_head = texture_creator.load_texture(&notehead_path)?;

    let (playback_tx, playback_rx): (Sender<()>, Receiver<()>) = mpsc::channel();
    let (pitch_detection_tx, pitch_detection_rx): (Sender<bool>, Receiver<bool>) = mpsc::channel();

    let mut trainer = IntervalTrainer::init(NoteRange::tenor_voice());

    let cool_note = Note::parse_from_string("G5")?;

    'mainloop: loop {
        for event in sdl_context.event_pump()?.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown { keycode: Option::Some(Keycode::Escape), .. } => break 'mainloop,
                Event::KeyDown { keycode: Option::Some(Keycode::A), .. } => {
                    if matches!(trainer.scene, Scene::Idle) {
                        let (reference, mystery_note) = choose_notes(&trainer.range);
                        let note_length = Duration::from_millis(1000);
                        crate::synth::play_notes_in_thread(
                            reference,
                            mystery_note,
                            note_length,
                            SAMPLE_RATE,
                            playback_tx.clone(),
                        );
                        trainer.scene = Scene::PlayingSound(reference, mystery_note);
                    }
                }
                _ => {}
            }
        }

        if let Scene::PlayingSound(_, mystery_note) = trainer.scene {
            match playback_rx.try_recv() {
                Ok(()) => {
                    let detection_duration = Duration::from_millis(1500);
                    listen_for_note_in_thread(
                        mystery_note.to_simple(),
                        detection_duration,
                        SAMPLE_RATE,
                        pitch_detection_tx.clone(),
                    );
                    trainer.scene = Scene::Listening(mystery_note.to_simple());
                }
                Err(_) => {}
            }
        }

        if let Scene::Listening(mystery_note) = trainer.scene {
            match pitch_detection_rx.try_recv() {
                Ok(true) => {
                    println!("gg");
                    break 'mainloop;
                }
                Ok(false) => {
                    println!("womp womp");
                    break 'mainloop;
                }
                Err(_) => {}
            }
        }

        canvas.set_draw_color(WHITE);
        canvas.clear();
        render_staff(&treble_staff, &mut canvas)?;
        render_note(cool_note, &note_head, &mut canvas)?;
        canvas.present();
    }

    Ok(())
}

fn choose_notes(range: &NoteRange) -> (Note, Note) {
    let interval = Interval::get_random_diatonic();
    let direction = Direction::Up;

    let new_range = match direction {
        Direction::Up => range.crop_top(interval.size_i8()),
        Direction::Down => range.crop_bottom(interval.size_i8()),
    };

    let reference = new_range.rand();
    (reference, reference.up(interval))
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

/*
backend usage for reference:

    const SAMPLE_RATE: u16 = 44_100;

    let range = NoteRange::tenor_voice();
    let (reference_note, mystery_note) = choose_notes(&range);

    println!("This is {}", reference_note);
    play_notes(
        reference_note,
        mystery_note,
        Duration::from_millis(1000),
        SAMPLE_RATE,
    );

    match listen_for_note(
        mystery_note.to_simple(),
        Duration::from_millis(1500),
        SAMPLE_RATE,
    ) {
        Some(cent_deviation) => println!(
            "you got it ! it was {}\nyou got it within a {} cent deviation",
            mystery_note, cent_deviation
        ),
        None => println!("womp womp it was {}", mystery_note),
    }
*/
