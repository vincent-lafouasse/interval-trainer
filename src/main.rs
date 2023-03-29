#![allow(unused_imports)]
#![allow(dead_code)]

mod intervals;
mod notes;
mod synth;

use color_eyre::eyre::Result;

use crate::intervals::{BaseInterval, Interval, Quality};
use crate::notes::{Alteration, Note, NoteName, CHROMATIC_NOTES_PER_OCTAVE};
use crate::synth::{Oscillator, Wavetable, WavetableType};

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
        let sample_size: usize = 10;
        let wavetable_type = WavetableType::Square;
        let square_wave: Wavetable = Wavetable::new(sample_size, wavetable_type);
        println!("{:?}", square_wave.plot);
        let sine_wave = Wavetable::new(sample_size, WavetableType::Sine);
        println!("{:?}", sine_wave.plot);
        let mut sine_oscillator = Oscillator::new(44000, sine_wave);
        sine_oscillator.set_frequency(420.0);
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

    let user_note = Note::get_from_user();

    if user_note == computed_end_note {
        println!("ding ding you win");
    } else {
        println!("[EXTREMELY LOUD INCORRECT BUZZER]");
        println!(
            "{} to {} is not a {}, that is a {}\n",
            random_start_note,
            user_note,
            random_interval,
            Interval::between(random_start_note, user_note)
        );
        println!("The correct answer was {}", computed_end_note);
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
