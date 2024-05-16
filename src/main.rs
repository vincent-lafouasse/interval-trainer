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

    let range = NoteRange::from_str("C2", "C5").unwrap();
    let interval = Interval::get_random_diatonic();
    let direction = Direction::Up;

    let new_range = match direction {
        Direction::Up => range.crop_top(interval.size_i8()),
        Direction::Down => range.crop_bottom(interval.size_i8()),
    };

    let reference: Note = new_range.rand();
    let mystery_note: Note = reference.up(interval);
    println!("This is {}", reference);

    let synth = WavetableSynth::new(SINE, SAMPLE_RATE);

    let note_length_ms = 3000;
    synth.play(reference.frequency(), note_length_ms, &stream_handle);
    sleep(Duration::from_secs(1));
    synth.play(mystery_note.frequency(), note_length_ms, &stream_handle);

    println!("It was {}. Did you get it right?", mystery_note);

    Ok(())
}
