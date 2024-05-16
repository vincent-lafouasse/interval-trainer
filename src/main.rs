#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

mod interval;
mod note_range;
mod notes;
mod simple_note;
mod synth;
mod wavetables;

use cpal::traits::{DeviceTrait, HostTrait};
use cpal::{Device, Host, SupportedStreamConfig};
use pitch_detection::detector::mcleod::McLeodDetector;
use pitch_detection::detector::PitchDetector;
use rodio::OutputStream;

use color_eyre::eyre::Result;
use std::thread::sleep;
use std::time::Duration;

use crate::interval::{Direction, Interval};
use crate::note_range::NoteRange;
use crate::notes::Note;
use crate::simple_note::SimpleNote;
use crate::synth::{Wavetable, WavetableSynth};

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
    let (reference, mystery_note) = choose_notes(&range);

    let f0: f64 = reference.frequency();
    let f: f64 = mystery_note.frequency();

    let synth = WavetableSynth::new(SINE, SAMPLE_RATE);
    println!("This is {}", reference);
    synth.play(f0, 1000, &stream_handle);
    sleep(Duration::from_secs(1));
    synth.play(f, 1000, &stream_handle);

    listen_for_frequency(f);
    println!("It was {}. Did you get it right?", mystery_note);

    Ok(())
}

fn listen_for_frequency(_f: f64) {
    // call Detector<sample type>.get_pitch() on input callback ?
    let (_host, input_device, config) =
        setup_input_device().expect("Failed to find an input device");
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

fn setup_input_device() -> Result<(Host, Device, SupportedStreamConfig), &'static str> {
    let host: Host = cpal::default_host();
    let device: Device = match host.default_input_device() {
        Some(device) => device,
        None => return Err("no input device available"),
    };

    let stream_config: SupportedStreamConfig = match device.default_input_config() {
        Ok(config) => config,
        Err(_) => return Err("couldnt find default stream config"),
    };

    Ok((host, device, stream_config))
}

fn distance_cents(f0: f64, f: f64) -> i32 {
    (1200.0 * f64::log2(f / f0)) as i32
}
