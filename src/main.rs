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

use color_eyre::eyre::Result;
use std::time::Duration;

use eframe::egui;
use egui::Color32;

use crate::interval::{Direction, Interval};
use crate::listen::listen_for_note;
use crate::note_range::NoteRange;
use crate::notes::Note;
use crate::simple_note::SimpleNote;
use crate::synth::play_notes;

const SAMPLE_RATE: u16 = 44_100;

#[derive(Default)]
struct IntervalTrainer {
    scene: Scene,
}

impl IntervalTrainer {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

    fn set_scene(&mut self, new_scene: Scene) {
        self.scene = new_scene;
    }
}

#[derive(Default, Copy, Clone, PartialEq)]
enum Scene {
    #[default]
    Hello,
    CoolSVG,
}

impl eframe::App for IntervalTrainer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Click me").clicked() {
                match self.scene {
                    Scene::Hello => self.set_scene(Scene::CoolSVG),
                    Scene::CoolSVG => self.set_scene(Scene::Hello),
                }
            }

            match self.scene {
                Scene::Hello => {
                    ui.label("Hello");
                }
                Scene::CoolSVG => {
                    ui.label("Cool SVG");
                    ui.add(
                        egui::Image::new(egui::include_image!("assets/svg/Eb4_treble.svg"))
                            .fit_to_exact_size([1000.0, 500.0].into())
                            .bg_fill(Color32::WHITE),
                    );
                }
            }
        });
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1000.0, 500.0]),
        ..Default::default()
    };
    let _ = eframe::run_native(
        "Interval Trainer",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::<IntervalTrainer>::default()
        }),
    );

    /*
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
