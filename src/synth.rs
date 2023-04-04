use core::time::Duration;
use rodio::source::Source;

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
        self.index_increment =
            frequency * (self.wavetable.resolution as f32) / (self.sample_rate as f32);
    }

    pub fn get_sample(&mut self) -> f32 {
        let sample = self.wavetable.interpolate(self.index);
        self.index += self.index_increment;
        self.index %= self.wavetable.resolution as f32;
        return sample;
    }
}

impl Iterator for Oscillator {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.get_sample())
    }
}

impl Source for Oscillator {
    fn channels(&self) -> u16 {
        1
    }
    fn sample_rate(&self) -> u32 {
        self.sample_rate as u32
    }
    fn current_frame_len(&self) -> Option<usize> {
        None
    }
    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

pub struct Wavetable {
    pub plot: [f32; 256],
    pub resolution: usize,
}

impl Wavetable {
    pub fn new() -> Self {
        use std::f32::consts::PI;
        let mut plot: [f32; 256] = [0.0; 256];

        for i in 0..256 {
            plot[i] = (2.0 * (i as f32) * PI / (256 as f32)).sin();
        }
        Wavetable { plot, resolution: 256 }
    }

    fn interpolate(&self, float_index: f32) -> f32 {
        let left_index = float_index as usize;
        let right_index = (left_index + 1) % self.resolution;
        let right_weight = float_index - (left_index as f32);
        let left_weight = 1.0 - right_weight;

        return left_weight * self.at(left_index) + right_weight * self.at(right_index);
    }

    pub fn at(&self, index: usize) -> f32 {
        self.plot[index]
    }
}
