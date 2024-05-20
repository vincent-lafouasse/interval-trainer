use std::sync::mpsc::Sender;
use std::thread::sleep;
use std::time::{Duration, Instant};

use rodio::source::Source;
use rodio::{OutputStream, OutputStreamHandle, Sink};

use crate::notes::Note;
use crate::wavetables::*;

static SQUARE8: Wavetable = Wavetable::square8();

pub fn play_notes(n1: Note, n2: Note, note_length: Duration, sample_rate: u16, signal: Sender<()>) {
    let synth = WavetableSynth::new(SQUARE8, sample_rate);
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    // synth needs refactoring to take a Duration instead of a u64
    synth.play(n1.frequency(), note_length, &stream_handle);
    sleep(Duration::from_secs(1));
    synth.play(n2.frequency(), note_length, &stream_handle);
    signal.send(()).ok();
}

pub struct WavetableSynth {
    wavetable: Wavetable,
    sample_rate: u16,
    vca: Vca,
}

impl WavetableSynth {
    pub fn play(&self, frequency: f64, note_length: Duration, handle: &OutputStreamHandle) {
        let sink = Sink::try_new(handle).expect("Failed to create a new sink for audio playback");
        sink.set_volume(0.0);

        let mut oscillator = Oscillator::new(self.sample_rate, self.wavetable);
        oscillator.set_frequency(frequency);
        sink.append(oscillator);

        let note_start = Instant::now();
        let update_period = Duration::from_millis(5);

        while Instant::now().duration_since(note_start) <= note_length + self.vca.release {
            let start_tick = Instant::now();

            sink.set_volume(
                self.vca
                    .get(Instant::now().duration_since(note_start), note_length),
            );

            sleep(update_period.saturating_sub(Instant::now().duration_since(start_tick)));
        }

        sink.stop();
    }

    pub fn new(wavetable: Wavetable, sample_rate: u16) -> Self {
        WavetableSynth {
            wavetable,
            sample_rate,
            vca: Vca {
                attack: Duration::from_millis(500),
                sustain: 1.0,
                release: Duration::from_millis(500),
            },
        }
    }
}

pub struct Vca {
    attack: Duration,
    sustain: f32,
    release: Duration,
}

impl Vca {
    pub fn new(attack: Duration, sustain: f32, release: Duration) -> Self {
        Vca { attack, sustain, release }
    }

    pub fn get(&self, from_start: Duration, length: Duration) -> f32 {
        if from_start < self.attack {
            return interpolate(
                duration_to_millis(from_start),
                0.0,
                duration_to_millis(self.attack),
                0.0,
                self.sustain,
            );
        }

        if from_start < length {
            return self.sustain;
        }

        if from_start - length < self.release {
            return interpolate(
                duration_to_millis(from_start - length),
                0.0,
                duration_to_millis(self.release),
                self.sustain,
                0.0,
            );
        }

        0.0
    }
}

/// A wavetable oscillator that can play sound via the `rodio::source::Source` trait
pub struct Oscillator {
    sample_rate: u16,
    wavetable: Wavetable,
    index: f32,
    index_increment: f32,
}

impl Oscillator {
    pub fn new(sample_rate: u16, wavetable: Wavetable) -> Self {
        Oscillator { sample_rate, wavetable, index: 0., index_increment: 0. }
    }

    pub fn set_frequency(&mut self, frequency: f64) {
        // how much to move in the wavetable per tick
        // linear in self.wavetable.resolution()
        // linear in frequency (higher f => bigger increment to get more periods per units of time)
        // inverse in sample_rate because "per tick"
        self.index_increment =
            ((self.wavetable.resolution() as f64) * frequency / (self.sample_rate as f64)) as f32;
    }

    pub fn get_sample(&mut self) -> f32 {
        // the wavetable is discrete so non-integer values must be estimated
        // here by linear interpolation
        let sample = self.wavetable.at(self.index);
        self.index += self.index_increment;
        self.index %= self.wavetable.resolution() as f32;
        sample
    }
}

// required for the `Source` trait
impl Iterator for Oscillator {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.get_sample())
    }
}

#[derive(Copy, Clone)]
pub struct Wavetable {
    pub plot: &'static [f32; 1024],
}

impl Wavetable {
    pub const fn square8() -> Self {
        Wavetable { plot: &SQUARE_8 }
    }

    pub fn at(&self, index: f32) -> f32 {
        let left_index = index as usize;
        let right_index = (left_index + 1) % self.resolution();
        let right_weight = index - (left_index as f32);
        let left_weight = 1.0 - right_weight;

        left_weight * self.plot[left_index] + right_weight * self.plot[right_index]
    }

    pub fn resolution(&self) -> usize {
        1024
    }
}

impl Source for Oscillator {
    fn channels(&self) -> u16 {
        // number of channels
        // this is a monophonic synth
        1
    }
    fn sample_rate(&self) -> u32 {
        self.sample_rate as u32
    }
    fn current_frame_len(&self) -> Option<usize> {
        // > Returns the number of samples before the current frame ends. None means “infinite” or
        // > “until the sound ends”. Should never return 0 unless there’s no more data.
        None
    }
    fn total_duration(&self) -> Option<Duration> {
        // > Returns the total duration of this source, if known.
        // > None indicates at the same time “infinite” or “unknown”.
        None
    }
}

fn duration_to_millis(duration: Duration) -> f32 {
    (duration.as_secs() as f32) * 1000.0 + (duration.subsec_millis() as f32)
}

fn interpolate(current: f32, start: f32, end: f32, start_value: f32, end_value: f32) -> f32 {
    let high_contribution = (current - start) / (end - start);
    let low_contribution = (end - current) / (end - start);

    high_contribution * end_value + low_contribution * start_value
}
