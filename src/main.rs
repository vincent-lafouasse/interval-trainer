#![allow(unused_imports)]
#![allow(dead_code)]

mod frequencies;
mod intervals;
mod notes;
mod synth;
mod wavetables;

use color_eyre::eyre::Result;
use rodio::{OutputStream, OutputStreamHandle, Sink};
use std::thread::sleep;
use std::time::{Duration, Instant};

use crate::frequencies::FREQUENCIES;
use crate::intervals::{BaseInterval, Interval, Quality};
use crate::notes::{Alteration, Note, NoteName, CHROMATIC_NOTES_PER_OCTAVE};
use crate::synth::{Oscillator, Wavetable, WavetableSynth};

const SAMPLE_RATE: usize = 44_100;
static SINE: Wavetable = Wavetable::new();

fn main() -> Result<()> {
    color_eyre::install()?;
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let quizing = false;
    let debugging = false;
    let synth = true;
    let bach = false;

    if debugging {
        debug();
    }

    if quizing {
        quiz();
    }

    if synth {
        let mut synth = WavetableSynth::new(SINE, SAMPLE_RATE);
        synth.set_volume(0.5);

        let f_a4: f32 = FREQUENCIES[4 * CHROMATIC_NOTES_PER_OCTAVE + 9];
        let f_e5: f32 = FREQUENCIES[4 * CHROMATIC_NOTES_PER_OCTAVE + 9 + 7];
        let note_length_ms = 3000;

        synth.play(f_a4, note_length_ms, &stream_handle);
        sleep(Duration::from_secs(1));
        synth.play(f_e5, note_length_ms, &stream_handle);
    }

    if bach {
        play_some_bach(&stream_handle);
    }

    Ok(())
}

fn play_some_bach(handle: &OutputStreamHandle) {
    let mut synth = WavetableSynth::new(SINE, SAMPLE_RATE);
    synth.set_volume(0.5);
    synth.set_fade_length_ms(100, 100);

    let f_a4: f32 = FREQUENCIES[4 * CHROMATIC_NOTES_PER_OCTAVE + 9];
    let f_b4: f32 = FREQUENCIES[4 * CHROMATIC_NOTES_PER_OCTAVE + 11];
    let f_c5: f32 = FREQUENCIES[5 * CHROMATIC_NOTES_PER_OCTAVE + 0];
    let f_d5: f32 = FREQUENCIES[5 * CHROMATIC_NOTES_PER_OCTAVE + 2];

    let time_unit_ms = 400;

    let notes_to_play = vec![
        (f_a4, 1),
        (f_d5, 1),
        (f_c5, 1),
        (f_b4, 1),
        (f_c5, 1),
        (f_a4, 1),
        (f_d5, 4),
        (f_c5, 1),
        (f_d5, 1),
        (f_b4, 4),
    ];

    for (frequency, note_length) in notes_to_play.iter() {
        synth.play(*frequency, *note_length * time_unit_ms, handle);
    }
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
