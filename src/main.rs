#![allow(unused_imports)]
#![allow(dead_code)]

mod frequencies;
mod intervals;
mod notes;
mod synth;

use color_eyre::eyre::Result;
use core::time::Duration;
use rodio::source::Source;
use rodio::{OutputStream, Sink};

use crate::frequencies::FREQUENCIES;
use crate::intervals::{BaseInterval, Interval, Quality};
use crate::notes::{Alteration, Note, NoteName, CHROMATIC_NOTES_PER_OCTAVE};
use crate::synth::{Oscillator, Wavetable};

fn main() -> Result<()> {
    color_eyre::install()?;

    let quizing = false;
    let debugging = false;
    let synth = true;

    if debugging {
        debug();
    }

    if quizing {
        quiz();
    }

    if synth {
        static SINE: Wavetable = Wavetable::new();
        const SAMPLE_RATE: usize = 44_100;
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();

        let volume = 0.5;

        let f_c4: f32 = FREQUENCIES[4 * CHROMATIC_NOTES_PER_OCTAVE];
        let f_d4: f32 = FREQUENCIES[4 * CHROMATIC_NOTES_PER_OCTAVE + 2];
        let f_e4: f32 = FREQUENCIES[4 * CHROMATIC_NOTES_PER_OCTAVE + 4];
        let f_f4: f32 = FREQUENCIES[4 * CHROMATIC_NOTES_PER_OCTAVE + 5];
        let f_g4: f32 = FREQUENCIES[4 * CHROMATIC_NOTES_PER_OCTAVE + 7];

        let time_unit_ms = 300;

        let notes_to_play = vec![
            (f_d4, 2 * time_unit_ms),
            (f_e4, 1 * time_unit_ms),
            (f_f4, 2 * time_unit_ms),
            (f_g4, 1 * time_unit_ms),
            (f_e4, 3 * time_unit_ms),
            (f_c4, 2 * time_unit_ms),
            (f_d4, 4 * time_unit_ms),
        ];

        for (frequency, duration) in notes_to_play.iter() {
            let sink = Sink::try_new(&stream_handle)?;
            sink.set_volume(volume);
            let mut sine_oscillator = Oscillator::new(SAMPLE_RATE, SINE);
            sine_oscillator.set_frequency(*frequency);
            sink.append(sine_oscillator);
            std::thread::sleep(std::time::Duration::from_millis(*duration));
            sink.stop();
        }
    }

    Ok(())
}

fn quiz() {
    println!("-----------------------------------------------------------");
    let random_interval = Interval::get_random_diatonic();
    let random_start_note = Note::get_random_biased();

    println!(
        "What note is a {} above {}?",
        random_interval, random_start_note
    );

    let computed_end_note = random_interval.note_up_from(random_start_note);

    loop {
        let user_note = Note::get_from_user();

        if user_note == computed_end_note {
            println!("ding ding you win");
            break;
        } else {
            let user_interval = Interval::between(random_start_note, user_note);
            println!("[EXTREMELY LOUD INCORRECT BUZZER]");
            println!("{random_start_note} to {user_note} is not a {random_interval}, that is a {user_interval}\n");
        }
    }
}

fn debug() {
    println!("-----------------------------------------------------------");
    let random_interval = Interval::get_random_diatonic();

    println!("Here's a random interval :\n\t{}", random_interval);
    println!("Its size is {} semitones\n", random_interval.size());

    let c = Note { name: NoteName::C, alteration: Alteration::Natural };

    let up_from_c: Note = random_interval.note_up_from(c);
    println!("{} is a {} up from C\n", up_from_c, random_interval);

    let note1 = Note::get_random_biased();
    let note2 = Note::get_random_biased();
    println!("Here are two random notes: {} and {}", note1, note2);
    println!("Between them is a {}", Interval::between(note1, note2));
}
