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

use crate::note_range::NoteRange;
use crate::notes::Note;
use crate::simple_note::SimpleNote;
use crate::synth::{Oscillator, Wavetable, WavetableSynth};

const SAMPLE_RATE: usize = 44_100;
static SINE: Wavetable = Wavetable::new();

fn main() -> Result<()> {
    color_eyre::install()?;
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let quizing = false;
    let synth = true;

    if quizing {
        quiz();
    }

    if synth {
        let synth = WavetableSynth::new(SINE, SAMPLE_RATE);

        let f_a4: f32 = Note::parse_from_string("A4").unwrap().frequency();
        let f_e5: f32 = Note::parse_from_string("E5").unwrap().frequency();
        let note_length_ms = 3000;

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
    let range = NoteRange::alto_sax();
    println!("{range}");
    let note = range.rand();
    println!("{note}");
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
