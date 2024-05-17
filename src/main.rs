#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

mod interval;
mod note_range;
mod notes;
mod pitch_detector;
mod simple_note;
mod synth;
mod wavetables;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, Host, StreamConfig};
use pitch_detection::detector::mcleod::McLeodDetector;
use pitch_detection::detector::PitchDetector;
use rodio::{OutputStream, OutputStreamHandle};

use color_eyre::eyre::Result;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::thread::sleep;
use std::time::{Duration, Instant};

use crate::interval::{Direction, Interval};
use crate::note_range::NoteRange;
use crate::notes::Note;
use crate::pitch_detector::{MyPitchDetectorConfig, MyPitchDetectorContext};
use crate::simple_note::SimpleNote;
use crate::synth::{Wavetable, WavetableSynth};

const SAMPLE_RATE: u16 = 44_100;
static SINE: Wavetable = Wavetable::new();

fn main() -> Result<()> {
    color_eyre::install()?;

    let range = NoteRange::tenor_voice();
    let (reference_note, mystery_note) = choose_notes(&range);

    println!("This is {}", reference_note);
    play_notes(reference_note, mystery_note, Duration::from_millis(1000));

    listen_for_frequency(mystery_note.frequency(), Duration::from_millis(1500));
    println!(
        "It was {} at a frequency of {} Hz. Did you get it right?",
        mystery_note,
        mystery_note.frequency() as u32
    );

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

fn play_notes(n1: Note, n2: Note, note_length: Duration) {
    let synth = WavetableSynth::new(SINE, SAMPLE_RATE);
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    // synth needs refactoring to take a Duration instead of a u64
    synth.play(n1.frequency(), note_length, &stream_handle);
    sleep(Duration::from_secs(1));
    synth.play(n2.frequency(), note_length, &stream_handle);
}

fn listen_for_frequency(_f: f64, detection_duration: Duration) {
    let config = MyPitchDetectorConfig {
        n_channels: 1,
        sample_rate: SAMPLE_RATE,
        buffer_size: 1024,
        power_threshold: 5.0,
        clarity_threshold: 0.7,
        precision_threshold_cents: 20,
    };
    let context = MyPitchDetectorContext::new(config).unwrap();

    let audio_thread_freq = Arc::new(AtomicU64::new(0));
    let ui_thread_freq = audio_thread_freq.clone();
    let mut detection_buffer: Vec<f32> = Vec::new();

    let input_callback = move |data: &[f32], _: &cpal::InputCallbackInfo| {
        if detection_buffer.len() >= config.buffer_size {
            // buffer is ready to try pitch detection
            let mut detector = McLeodDetector::new(config.buffer_size, config.buffer_size / 2);
            if let Some(pitch) = detector.get_pitch(
                &detection_buffer[0..config.buffer_size],
                config.sample_rate.into(),
                config.power_threshold,
                config.clarity_threshold,
            ) {
                audio_thread_freq.store(
                    Into::<f64>::into(pitch.frequency).to_bits(),
                    Ordering::Relaxed,
                );
            }
            detection_buffer.clear();
        } else {
            // detection buffer isn't full, use this callback to append a callback buffer
            detection_buffer.extend_from_slice(data);
        }
    };

    let stream = context
        .input_device
        .build_input_stream::<f32, _, _>(
            &context.stream_config,
            input_callback,
            |e| eprintln!("An error has occured on the audio thread: {e}"),
            None,
        )
        .unwrap();

    println!(
        "lauching an input stream for {} ms",
        detection_duration.as_millis()
    );

    let update_fps = 10.0;
    let start = Instant::now();

    stream.play().unwrap();
    while Instant::now().duration_since(start) < detection_duration {
        let tick_start = Instant::now();

        let detected_pitch = f64::from_bits(ui_thread_freq.load(Ordering::Relaxed));
        if detected_pitch > 0.0 {
            let (note, error) = closest_note(detected_pitch);
            let cent_precision_threshold = 20;
            if error.abs() < cent_precision_threshold {
                println!("{}\t{} cents", note, error);
            }
        }

        regularize_fps(
            tick_start,
            Duration::from_millis((1000.0 / update_fps) as u64),
        );
    }

    stream.pause().unwrap();
}

fn regularize_fps(tick_start: Instant, target_tick_duration: Duration) {
    let actual_tick_duration = Instant::now().duration_since(tick_start);
    if actual_tick_duration < target_tick_duration {
        std::thread::sleep(target_tick_duration - actual_tick_duration);
    }
}

fn setup_input_device() -> Result<(Host, Device), &'static str> {
    let host: Host = cpal::default_host();
    let device: Device = match host.default_input_device() {
        Some(device) => device,
        None => return Err("no input device available"),
    };

    Ok((host, device))
}

fn closest_note(f: f64) -> (SimpleNote, i8) {
    let distance_from_a4 = distance_cents(440.0, f);
    let distance_from_c_min_1 = distance_from_a4 + 69 * 100;

    let simple_note: SimpleNote;
    let error: u8;
    let positive_error: i8 = distance_from_c_min_1.rem_euclid(100).try_into().unwrap();
    let floor_note = distance_from_c_min_1 / 100;
    let floor_note: i8 = floor_note.try_into().unwrap();
    let floor_note = SimpleNote::new(floor_note);

    match positive_error < 50 {
        true => (floor_note, positive_error),
        false => (floor_note.shift(1), 100 - positive_error),
    }
}

fn distance_cents(f0: f64, f: f64) -> i32 {
    (1200.0 * f64::log2(f / f0)) as i32
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
