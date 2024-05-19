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

use std::thread;
use std::time::Duration;

use crate::interval_trainer::IntervalTrainer;

const SAMPLE_RATE: u16 = 44_100;

fn main() -> Result<()> {
    color_eyre::install()?;
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1000.0, 500.0]),
        ..Default::default()
    };

    std::thread::spawn(|| {
        println!("before");
        std::thread::sleep(Duration::from_secs(1));
        println!("after");
    });

    // run gui in main thread
    let _ = eframe::run_native(
        "Interval Trainer",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::new(IntervalTrainer::new(cc))
        }),
    );

    Ok(())
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
