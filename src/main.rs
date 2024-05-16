#![allow(unused_imports)]
#![allow(dead_code)]

mod note_range;
mod notes;
mod synth;
mod wavetables;

use color_eyre::eyre::Result;
use rodio::{OutputStream, OutputStreamHandle, Sink};
use std::thread::sleep;
use std::time::{Duration, Instant};

use crate::note_range::NoteRange;
use crate::notes::Note;
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

        let f_a4: f32 = 440.0;
        let f_e5: f32 = f_a4 * 1.5;
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

fn log_note(note: &Note) {
    println!("-----------------------------------------------------------");
    println!("{}", note);
    println!("{:#?}", note);
    println!("{}", note.to_midi_style());
    println!("{}", note.frequency());
}

fn quiz() {
    println!("-----------------------------------------------------------");
    let note = Note::parse_from_string("A4").unwrap();
    log_note(&note);
    let note = Note::parse_from_string("A2").unwrap();
    log_note(&note);
    let note = Note::parse_from_string("G##2").unwrap();
    log_note(&note);

    let range = NoteRange::alto_sax();
    println!("{range}");
}
