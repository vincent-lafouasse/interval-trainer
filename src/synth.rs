use core::time::Duration;
use rodio::source::Source;

use crate::wavetables::SINE_256;

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
        return sample;
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

        return left_weight * self.at(left_index) + right_weight * self.at(right_index);
    }

    pub fn at(&self, index: usize) -> f32 {
        self.plot[index]
    }

    pub fn resolution(&self) -> usize {
        256
    }
}
