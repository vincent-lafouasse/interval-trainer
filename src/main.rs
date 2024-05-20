#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

//! # A cool ear trainer

mod interval;
mod interval_trainer;
mod listen;
mod note_range;
mod notes;
mod simple_note;
mod synth;
mod wavetables;

use color_eyre::eyre::Result;
use eframe::egui;

use std::sync::Arc;
use std::thread;
use std::time::Duration;

use crate::interval_trainer::IntervalTrainer;

fn main() -> Result<()> {
    color_eyre::install()?;
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1000.0, 500.0]),
        ..Default::default()
    };

    // run gui in main thread
    let _ = eframe::run_native(
        "Interval Trainer",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::new(IntervalTrainer::new_from_egui_context(cc))
        }),
    );

    Ok(())
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

/*
backend usage:

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
