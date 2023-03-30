#![allow(unused_imports)]
#![allow(dead_code)]

mod intervals;
mod notes;
mod synth;

use color_eyre::eyre::Result;
use core::time::Duration;
use rodio::source::Source;
use rodio::OutputStream;

use crate::intervals::{BaseInterval, Interval, Quality};
use crate::notes::{Alteration, Note, NoteName, CHROMATIC_NOTES_PER_OCTAVE};
use crate::synth::{Oscillator, Wavetable, WavetableType};

fn main() -> Result<()> {
    color_eyre::install()?;

    let quizing = true;
    let debugging = false;
    let synth = false;

    if debugging {
        debug();
    }

    if quizing {
        quiz();
    }

    if synth {
        let wavetable_resolution: usize = 64;
        let wavetable = Wavetable::new(wavetable_resolution, WavetableType::Sine);

        let sample_rate = 44_000;
        let mut sine_oscillator = Oscillator::new(sample_rate, wavetable);

        sine_oscillator.set_frequency(420.0);
        let note_length = std::time::Duration::from_secs(2);

        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let _result = stream_handle.play_raw(sine_oscillator.convert_samples());

        std::thread::sleep(note_length);
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
            println!("[EXTREMELY LOUD INCORRECT BUZZER]");
            println!(
                "{} to {} is not a {}, that is a {}\n",
                random_start_note,
                user_note,
                random_interval,
                Interval::between(random_start_note, user_note)
            );
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
