#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

mod interval;
mod note_range;
mod notes;
mod simple_note;
mod synth;
mod wavetables;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, Host, StreamConfig};
use pitch_detection::detector::mcleod::McLeodDetector;
use pitch_detection::detector::PitchDetector;
use rodio::{OutputStream, OutputStreamHandle};

use color_eyre::eyre::Result;
use std::sync::atomic::AtomicU32;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

use crate::interval::{Direction, Interval};
use crate::note_range::NoteRange;
use crate::notes::Note;
use crate::simple_note::SimpleNote;
use crate::synth::{Wavetable, WavetableSynth};

const SAMPLE_RATE: usize = 44_100;
static SINE: Wavetable = Wavetable::new();

fn main() -> Result<()> {
    color_eyre::install()?;

    let range = NoteRange::from_str("C2", "C5").unwrap();
    let (reference_note, mystery_note) = choose_notes(&range);

    println!("This is {}", reference_note);
    //play_notes(reference_note, mystery_note);

    listen_for_frequency(mystery_note.frequency());
    println!("It was {}. Did you get it right?", mystery_note);

    Ok(())
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

fn play_notes(n1: Note, n2: Note) {
    let synth = WavetableSynth::new(SINE, SAMPLE_RATE);
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    const NOTE_LENGTH: u64 = 1000;

    // synth needs refactoring to take a Duration instead of a u64
    synth.play(n1.frequency(), NOTE_LENGTH, &stream_handle);
    sleep(Duration::from_secs(1));
    synth.play(n2.frequency(), NOTE_LENGTH, &stream_handle);
}

fn listen_for_frequency(_f: f64) {
    let (_host, input_device) = setup_input_device().unwrap();
    let config = StreamConfig {
        channels: 1,
        sample_rate: cpal::SampleRate(44_100),
        buffer_size: cpal::BufferSize::Default,
    };

    const DETECTION_BUFFER_SIZE: usize = 1024;
    const PADDING: usize = DETECTION_BUFFER_SIZE / 2;
    const POWER_THRESHOLD: f64 = 5.0;
    const CLARITY_THRESHOLD: f64 = 0.7;

    let freq = Arc::new(AtomicU32::new(0));
    let mut detection_buffer: Vec<f32> = Vec::new();

    let stream = input_device
        .build_input_stream::<f32, _, _>(
            &config,
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                if detection_buffer.len() >= DETECTION_BUFFER_SIZE {
                    // buffer is ready to try pitch detection
                    println!("detection attempt");
                    detection_buffer.clear();
                } else {
                    // detection buffer isn't full, use this callback to append a callback buffer
                    detection_buffer.extend_from_slice(data);
                }
            },
            |e| eprintln!("An error has occured on the audio thread: {e}"),
            None,
        )
        .unwrap();

    let detection_duration = Duration::from_millis(100);
    println!(
        "lauching an input stream for {} ms",
        detection_duration.as_millis()
    );
    stream.play().unwrap();
    std::thread::sleep(detection_duration);
    stream.pause().unwrap();
}

fn setup_input_device() -> Result<(Host, Device), &'static str> {
    let host: Host = cpal::default_host();
    let device: Device = match host.default_input_device() {
        Some(device) => device,
        None => return Err("no input device available"),
    };

    Ok((host, device))
}

fn distance_cents(f0: f64, f: f64) -> i32 {
    (1200.0 * f64::log2(f / f0)) as i32
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
