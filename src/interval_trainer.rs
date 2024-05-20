use eframe::egui;

use std::{sync::mpsc, time::Duration};

use crate::{
    interval::{Direction, Interval},
    listen::listen_for_note,
    note_range::NoteRange,
    notes::Note,
    synth::play_notes,
};

enum TrainerState {
    SoundNotPlaying,
    PlayingSound(mpsc::Receiver<()>, Note),
    AwaitingUserGuess(Note),
}

pub struct IntervalTrainer {
    state: TrainerState,
    message: String,
    sample_rate: u16,
}

impl IntervalTrainer {
    pub fn run(&self) {
        println!("hi");
    }

    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let sample_rate: u16 = 44100;
        println!("hello from IntervalTrainer constructor");

        Self { state: TrainerState::SoundNotPlaying, message: "".to_string(), sample_rate }
    }
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

impl eframe::App for IntervalTrainer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Play sound").clicked() &&
                !matches!(self.state, TrainerState::PlayingSound(_, _)) {
                let range = NoteRange::tenor_voice();
                let (reference_note, mystery_note) = choose_notes(&range);
                let (sender, receiver) = mpsc::channel();
                std::thread::spawn({
                    let (r, m) = (reference_note, mystery_note);
                    let sample_rate = self.sample_rate;
                    move || play_notes(r, m, Duration::from_millis(1000), sample_rate, sender)
                });
                self.state = TrainerState::PlayingSound(receiver, mystery_note);
            }

            use TrainerState::*;
            match &self.state {
                SoundNotPlaying => {}
                PlayingSound(receiver, note) => {
                    ui.add(
                        egui::Image::new(egui::include_image!("assets/svg/A4_treble.svg"))
                            .fit_to_exact_size([1000.0, 500.0].into())
                            .bg_fill(egui::Color32::WHITE),
                    );
                    if let Ok(()) = receiver.try_recv() {
                        self.state = TrainerState::AwaitingUserGuess(*note);
                    }
                }
                TrainerState::AwaitingUserGuess(note) => {
                    match listen_for_note(
                        note.to_simple(),
                        Duration::from_millis(1500),
                        self.sample_rate,
                    ) {
                        Some(cent_deviation) => {
                            self.message = format!(
                                "you got it ! it was {}\nyou got it within a {} cent deviation",
                                note, cent_deviation
                            )
                        }
                        None => self.message = format!("womp womp it was {}", note),
                    }
                    self.state = TrainerState::SoundNotPlaying;
                }
            }
            ui.label(&self.message);
        });
    }
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
