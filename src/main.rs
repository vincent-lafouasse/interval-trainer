#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

//! # A cool ear trainer

mod interval_trainer;
mod listen;
mod music;
mod play_wav;
mod render;
mod synth;

use std::{
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
    interval_trainer::{IntervalTrainer, Scene},
    music::{
        interval::{Direction, Interval},
        note::Note,
        note_range::NoteRange,
        simple_note::SimpleNote,
    },
    render::Sprites,
};

const WHITE: Color = Color::RGB(255, 255, 255);

const WINDOW_WIDTH: u32 = 1000;
const WINDOW_HEIGHT: u32 = 400;

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
    let sprites = Sprites::init(&texture_creator)?;

    let (playback_tx, playback_rx): (Sender<()>, Receiver<()>) = mpsc::channel();
    let (pitch_detection_tx, pitch_detection_rx): (Sender<bool>, Receiver<bool>) = mpsc::channel();

    let mut trainer = IntervalTrainer::init(NoteRange::treble_staff());

    // let cool_note = Note::parse_from_string("F3")?;

    'mainloop: loop {
        for event in sdl_context.event_pump()?.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown { keycode: Option::Some(Keycode::Escape), .. } => break 'mainloop,
                Event::KeyDown { keycode: Option::Some(Keycode::Space), .. } => {
                    if let Scene::Concluding(_, _) = trainer.scene {
                        trainer.scene = Scene::Idle;
                    }
                    if let Scene::Idle = trainer.scene {
                        let (reference, mystery_note) = trainer.start_playback(playback_tx.clone());
                        trainer.scene = Scene::PlayingSound(reference, mystery_note);
                    }
                }
                _ => {}
            }
        }

        canvas.set_draw_color(WHITE);
        canvas.clear();
        crate::render::render_staff(&sprites.staff, &mut canvas)?;

        let left_x: i32 = 420;
        let right_x: i32 = 800;

        if let Scene::PlayingSound(reference, mystery_note) = trainer.scene {
            crate::render::render_note(reference, left_x, &sprites, &mut canvas)?;
            match playback_rx.try_recv() {
                Ok(()) => {
                    trainer.listen_for(mystery_note, pitch_detection_tx.clone());
                    trainer.scene = Scene::Listening(reference, mystery_note);
                }
                Err(_) => {}
            }
        }

        if let Scene::Listening(reference, mystery_note) = trainer.scene {
            crate::render::render_note(reference, left_x, &sprites, &mut canvas)?;
            match pitch_detection_rx.try_recv() {
                Ok(true) => {
                    trainer.ding();
                    trainer.scene = Scene::Concluding(reference, mystery_note);
                }
                Ok(false) => {
                    trainer.scene = Scene::Concluding(reference, mystery_note);
                }
                Err(_) => {}
            }
        }

        if let Scene::Concluding(reference, mystery_note) = trainer.scene {
            crate::render::render_note(reference, left_x, &sprites, &mut canvas)?;
            crate::render::render_note(mystery_note, right_x, &sprites, &mut canvas)?;
        }

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
