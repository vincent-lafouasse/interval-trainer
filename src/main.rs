#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

//! # A cool ear trainer

mod interval;
mod listen;
mod note_range;
mod notes;
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
};

use crate::{note_range::NoteRange, notes::Note, synth::play_notes_in_thread};

struct IntervalTrainer {
    scene: Scene,
    note_range: NoteRange,
    reference_note: Option<Note>,
    mystery_note: Option<Note>,
}

impl IntervalTrainer {
    fn init(note_range: NoteRange) -> Self {
        Self { scene: Scene::Idle, note_range, mystery_note: None, reference_note: None }
    }
}

#[derive(Default, Copy, Clone, Debug)]
enum Scene {
    #[default]
    Idle,
    PlayingSound,
    Listening,
    Concluding,
}

const WHITE: Color = Color::RGB(255, 255, 255);

const WINDOW_WIDTH: u32 = 1000;
const WINDOW_HEIGHT: u32 = 400;

const SAMPLE_RATE: u16 = 44_100;

fn main() -> Result<(), String> {
    let png_dir = Path::new("src/assets/png");
    let empty_treble_staff_path = png_dir.join("treble_staff.png");

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
    let empty_treble_staff = texture_creator.load_texture(empty_treble_staff_path)?;

    let (playback_tx, playback_rx): (Sender<()>, Receiver<()>) = mpsc::channel();
    let (pitch_detection_tx, pitch_detection_rx): (Sender<()>, Receiver<()>) = mpsc::channel();

    let trainer = IntervalTrainer::init(NoteRange::tenor_voice());

    'mainloop: loop {
        for event in sdl_context.event_pump()?.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown { keycode: Option::Some(Keycode::Escape), .. } => break 'mainloop,
                Event::KeyDown { keycode: Option::Some(Keycode::A), .. } => {
                    println!("here comes some notes");
                    let n1 = Note::parse_from_string("A4")?;
                    let n2 = Note::parse_from_string("E5")?;
                    let note_length = Duration::from_millis(1000);
                    crate::synth::play_notes_in_thread(
                        n1,
                        n2,
                        note_length,
                        SAMPLE_RATE,
                        playback_tx.clone(),
                    );
                }
                _ => {}
            }
        }

        match playback_rx.try_recv() {
            Ok(()) => println!("man those were some nice notes"),
            Err(_) => {}
        }

        canvas.set_draw_color(WHITE);
        canvas.clear();
        canvas.copy(&empty_treble_staff, None, None)?;
        canvas.present();
    }

    Ok(())
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
