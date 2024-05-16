#![allow(unused_imports)]
#![allow(dead_code)]

mod interval;
mod note_range;
mod notes;
mod simple_note;
mod synth;
mod wavetables;

use color_eyre::eyre::Result;
use pitch_detection::detector::mcleod::McLeodDetector;
use pitch_detection::detector::PitchDetector;
use rodio::{OutputStream, OutputStreamHandle, Sink};
use std::thread::sleep;
use std::time::{Duration, Instant};

use crate::interval::{Direction, Interval};
use crate::note_range::NoteRange;
use crate::notes::Note;
use crate::simple_note::SimpleNote;
use crate::synth::{Oscillator, Wavetable, WavetableSynth};

const SAMPLE_RATE: usize = 44_100;
const SIZE: usize = 1024;
const PADDING: usize = SIZE / 2;
const POWER_THRESHOLD: f64 = 5.0;
const CLARITY_THRESHOLD: f64 = 0.7;
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

    let f0: f64 = reference.frequency();
    let f: f64 = mystery_note.frequency();
    let note_length_ms = 3000;
    synth.play(f0, note_length_ms, &stream_handle);
    sleep(Duration::from_secs(1));
    synth.play(f, note_length_ms, &stream_handle);

    println!("It was {}. Did you get it right?", mystery_note);
    println!("{} Hz to {} Hz = {} cents", f0, f, distance_cents(f0, f));

    listen_for_frequency(f);

    Ok(())
}

fn listen_for_frequency(_f: f64) {
    let dt = 1.0 / SAMPLE_RATE as f64;
    let freq = 300.0;
    let signal: Vec<f64> = (0..SIZE)
        .map(|x| (2.0 * std::f64::consts::PI * x as f64 * dt * freq).sin())
        .collect();

    let mut detector = McLeodDetector::new(SIZE, PADDING);

    let pitch = detector
        .get_pitch(&signal, SAMPLE_RATE, POWER_THRESHOLD, CLARITY_THRESHOLD)
        .unwrap();

    println!("Frequency: {}, Clarity: {}", pitch.frequency, pitch.clarity);
}

fn distance_cents(f0: f64, f: f64) -> i32 {
    (1200.0 * f64::log2(f / f0)) as i32
}
