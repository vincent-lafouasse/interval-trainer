#![allow(unused_imports)]
#![allow(dead_code)]

mod interval;
mod note_range;
mod notes;
mod simple_note;
mod synth;
mod wavetables;

use color_eyre::eyre::Result;
use rodio::{OutputStream, OutputStreamHandle, Sink};
use std::thread::sleep;
use std::time::{Duration, Instant};

use crate::interval::{Direction, Interval};
use crate::note_range::NoteRange;
use crate::notes::Note;
use crate::simple_note::SimpleNote;
use crate::synth::{Oscillator, Wavetable, WavetableSynth};

const SAMPLE_RATE: usize = 44_100;
static SINE: Wavetable = Wavetable::new();

fn main() -> Result<()> {
    color_eyre::install()?;
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let quizing = true;
    let synth = false;

    if quizing {
        quiz();
    }

    if synth {
        let synth = WavetableSynth::new(SINE, SAMPLE_RATE);

        let f_a4: f32 = Note::parse_from_string("A4").unwrap().frequency();
        let f_e5: f32 = Note::parse_from_string("E5").unwrap().frequency();
        let note_length_ms = 3000;

        // needs refactoring to take a Duration instead of a usize
        synth.play(f_a4, note_length_ms, &stream_handle);
        sleep(Duration::from_secs(1));
        synth.play(f_e5, note_length_ms, &stream_handle);
    }

    Ok(())
}

/*
*   something like:
*
*   let range = Range::trombone();
*   let interval = Interval::get_random_common();
*   let direction = random_direction();
*   let reference, to_guess = random_notes(range, interval, direction);
*
*   synth.play(reference, duration);
*   synth.play(to_guess, duration);
*
*/

fn quiz() {
    println!("-----------------------------------------------------------");
    let range = NoteRange::from_str("C3", "C6").unwrap();
    println!("{range}");
    let interval = Interval::get_random_diatonic();
    println!("{interval}");
    let direction = Direction::Up;
    dbg!(direction);

    let size: u8 = interval.size_u8();

    let new_range = match direction {
        Direction::Up => range.crop_top(size),
        Direction::Down => range.crop_bottom(size),
    };
    println!("{new_range}");

    let reference: Note = new_range.rand();
    let to_guess: Note = reference.up(interval);
    println!("{interval}");
    println!("Reference: {}", reference);
    println!("To guess: {}", to_guess);
}

fn log_note(note: &Note) {
    println!("-----------------------------------------------------------");
    println!("{}", note);
    println!("{:#?}", note);
    println!("{}", note.to_simple());
    println!("{}", note.frequency());
}

fn log_simple(note: &SimpleNote) {
    println!("-----------------------------------------------------------");
    println!("{}: {}", note.data, note);
}
