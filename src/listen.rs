use std::time::{Duration, Instant};

use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Device, Host, StreamConfig,
};

use crate::simple_note::SimpleNote;
use pitch_detection::detector::{mcleod::McLeodDetector, PitchDetector};
use std::sync::{atomic::AtomicU64, atomic::Ordering, Arc};

pub type CentDeviation = i8;

pub fn listen_for_note(
    target_note: SimpleNote,
    detection_duration: Duration,
    sample_rate: u16,
) -> Option<CentDeviation> {
    let (_host, input_device) = setup_input_device().unwrap();
    let config = StreamConfig {
        channels: 1,
        sample_rate: cpal::SampleRate(sample_rate.into()),
        buffer_size: cpal::BufferSize::Default,
    };

    const DETECTION_BUFFER_SIZE: usize = 1024;
    const PADDING: usize = DETECTION_BUFFER_SIZE / 2;
    const POWER_THRESHOLD: f32 = 5.0;
    const CLARITY_THRESHOLD: f32 = 0.7;
    const CENT_DEVIATION_THRESHOLD: CentDeviation = 20;

    let audio_thread_freq = Arc::new(AtomicU64::new(0));
    let ui_thread_freq = audio_thread_freq.clone();
    let mut detection_buffer: Vec<f32> = Vec::new();

    let input_callback = move |data: &[f32], _: &cpal::InputCallbackInfo| {
        if detection_buffer.len() >= DETECTION_BUFFER_SIZE {
            // buffer is ready to try pitch detection
            let mut detector = McLeodDetector::new(DETECTION_BUFFER_SIZE, PADDING);
            if let Some(pitch) = detector.get_pitch(
                &detection_buffer[0..DETECTION_BUFFER_SIZE],
                sample_rate.into(),
                POWER_THRESHOLD,
                CLARITY_THRESHOLD,
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

    let stream = input_device
        .build_input_stream::<f32, _, _>(
            &config,
            input_callback,
            |e| eprintln!("An error has occured on the audio thread: {e}"),
            None,
        )
        .unwrap();

    const UPDATE_FPS: u8 = 10;
    let start = Instant::now();

    stream.play().unwrap();
    while Instant::now().duration_since(start) < detection_duration {
        let tick_start = Instant::now();

        let detected_pitch = f64::from_bits(ui_thread_freq.load(Ordering::Relaxed));
        if let Some((note, deviation)) = get_note(detected_pitch, CENT_DEVIATION_THRESHOLD) {
            if are_octaves_away(note, target_note) {
                stream.pause().unwrap();
                return Some(deviation);
            }
        }

        regularize_fps(tick_start, UPDATE_FPS);
    }

    stream.pause().unwrap();
    None
}

fn get_note(f: f64, cent_threshold: CentDeviation) -> Option<(SimpleNote, CentDeviation)> {
    let (note, deviation) = match f > 0.0 {
        true => closest_note(f),
        false => return None,
    };
    match deviation.abs() < cent_threshold {
        true => Some((note, deviation)),
        false => None,
    }
}

fn closest_note(f: f64) -> (SimpleNote, CentDeviation) {
    let distance_from_a4 = distance_cents(440.0, f);
    let distance_from_c_min_1 = distance_from_a4 + 69 * 100;

    let positive_deviation: CentDeviation =
        distance_from_c_min_1.rem_euclid(100).try_into().unwrap();
    let floor_note = distance_from_c_min_1 / 100;
    let floor_note: i8 = floor_note.try_into().unwrap();
    let floor_note = SimpleNote::new(floor_note);

    match positive_deviation < 50 {
        true => (floor_note, positive_deviation),
        false => (floor_note.shift(1), 100 - positive_deviation),
    }
}

fn regularize_fps(tick_start: Instant, target_fps: u8) {
    let target_tick_duration = Duration::from_millis((1000.0 / target_fps as f64) as u64);
    let actual_tick_duration = Instant::now().duration_since(tick_start);
    let to_wait = target_tick_duration.saturating_sub(actual_tick_duration);
    std::thread::sleep(to_wait);
}

fn setup_input_device() -> Result<(Host, Device), &'static str> {
    let host: Host = cpal::default_host();
    let device: Device = match host.default_input_device() {
        Some(device) => device,
        None => return Err("no input device available"),
    };

    Ok((host, device))
}

fn are_octaves_away(n1: SimpleNote, n2: SimpleNote) -> bool {
    (n1.get_i8() - n2.get_i8()) % 12 == 0
}

fn distance_cents(f0: f64, f: f64) -> i32 {
    (1200.0 * f64::log2(f / f0)) as i32
}
