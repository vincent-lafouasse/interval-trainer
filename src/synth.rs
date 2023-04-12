use std::thread::sleep;
use std::time::{Duration, Instant};

use rodio::source::Source;
use rodio::{OutputStream, OutputStreamHandle, Sink};

use crate::wavetables::*;

pub struct WavetableSynth {
    wavetable: Wavetable,
    sample_rate: usize,
    fade_in_ms: u64,
    fade_out_ms: u64,
    sustain_volume: f32,
    update_period_ms: u64,
}

impl WavetableSynth {
    pub fn new(wavetable: Wavetable, sample_rate: usize) -> Self {
        let fade_in_ms: u64 = 50;
        let fade_out_ms: u64 = 50;
        let sustain_volume: f32 = 1.0;
        let update_period_ms: u64 = 5;
        WavetableSynth {
            wavetable,
            sample_rate,
            fade_in_ms,
            fade_out_ms,
            sustain_volume,
            update_period_ms,
        }
    }

    pub fn play(&self, frequency: f32, note_length_ms: u64, handle: &OutputStreamHandle) {
        let sink = Sink::try_new(handle).expect("Failed to create a new sink for audio playback");
        sink.set_volume(0.0);

        let mut oscillator = Oscillator::new(self.sample_rate, self.wavetable);
        oscillator.set_frequency(frequency);
        sink.append(oscillator);

        let fade_in_increment: f32 =
            self.sustain_volume / (self.fade_in_ms as f32 / self.update_period_ms as f32);
        let fade_out_increment: f32 =
            self.sustain_volume / (self.fade_out_ms as f32 / self.update_period_ms as f32);

        let update_period = Duration::from_millis(self.update_period_ms);
        let fade_in_duration = Duration::from_millis(self.fade_in_ms);
        let fade_out_duration = Duration::from_millis(self.fade_out_ms);

        let note_length = Duration::from_millis(note_length_ms);
        let note_start = Instant::now();
        let note_end = note_start + note_length;

        while Instant::now() <= note_end {
            let start_tick = Instant::now();
            if Instant::now() <= note_start + fade_in_duration {
                let volume = sink.volume() + fade_in_increment;
                sink.set_volume(volume);
            }
            if Instant::now() >= note_end - fade_out_duration {
                let volume = sink.volume() - fade_out_increment;
                sink.set_volume(volume);
            }
            let end_tick = Instant::now();
            sleep(update_period - end_tick.duration_since(start_tick));
        }

        sink.stop();
    }

    pub fn set_fade_length_ms(&mut self, _fade_in_ms: u64, _fade_out_ms: u64) {
        self.fade_in_ms = _fade_in_ms;
        self.fade_out_ms = _fade_out_ms;
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.sustain_volume = volume;
    }
}

/// A wavetable oscillator that can play sound via the `rodio::source::Source` trait
pub struct Oscillator {
    sample_rate: usize,
    wavetable: Wavetable,
    index: f32,
    index_increment: f32,
}

impl Oscillator {
    pub fn new(sample_rate: usize, wavetable: Wavetable) -> Self {
        Oscillator { sample_rate, wavetable, index: 0., index_increment: 0. }
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        // how much to move in the wavetable per tick
        // linear in self.wavetable.resolution()
        // linear in frequency (higher f => bigger increment to get more periods per units of time)
        // inverse in sample_rate because "per tick"
        self.index_increment =
            (self.wavetable.resolution() as f32) * frequency / (self.sample_rate as f32);
    }

    pub fn get_sample(&mut self) -> f32 {
        // the wavetable is discrete so non-integer values must be estimated
        // here by linear interpolation
        let sample = self.wavetable.interpolate(self.index);
        self.index += self.index_increment;
        self.index %= self.wavetable.resolution() as f32;
        sample
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

// required for the `Source` trait
impl Iterator for Oscillator {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.get_sample())
    }
}

#[derive(Copy, Clone)]
pub struct Wavetable {
    pub plot: &'static [f32; 256],
}

impl Wavetable {
    pub const fn new() -> Self {
        Wavetable { plot: &SINE_256 }
    }

    fn interpolate(&self, float_index: f32) -> f32 {
        let left_index = float_index as usize;
        let right_index = (left_index + 1) % self.resolution();
        let right_weight = float_index - (left_index as f32);
        let left_weight = 1.0 - right_weight;

        left_weight * self.at(left_index) + right_weight * self.at(right_index)
    }

    pub fn at(&self, index: usize) -> f32 {
        self.plot[index]
    }

    pub fn resolution(&self) -> usize {
        256
    }
}
