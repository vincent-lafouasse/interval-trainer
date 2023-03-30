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
        let sample = self.linear_interpolation();
        self.index += self.index_increment;
        self.index %= self.wavetable.resolution as f32;
        return sample;
    }

    fn linear_interpolation(&self) -> f32 {
        let left_index = self.index as usize;
        let right_index = (left_index + 1) % self.wavetable.resolution;
        let right_weight = self.index - (left_index as f32);
        let left_weight = 1.0 - right_weight;

        return left_weight * self.wavetable.at(left_index)
            + right_weight * self.wavetable.at(right_index);
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
    pub plot: Vec<f32>,
    pub resolution: usize,
}

pub enum WavetableType {
    Sine,
    Square,
}

impl Wavetable {
    pub fn new(resolution: usize, wavetable_type: WavetableType) -> Self {
        use std::f32::consts::PI;
        let plot: Vec<f32> = match wavetable_type {
            WavetableType::Sine => (0..resolution)
                .map(|n| 2.0 * (n as f32) * PI / (resolution as f32))
                .map(|n| n.sin())
                .collect(),
            WavetableType::Square => (0..resolution)
                .map(|n| match 2 * n >= resolution {
                    true => 1.,
                    false => -1.,
                })
                .collect(),
        };
        Wavetable { plot, resolution }
    }

    pub fn at(&self, index: usize) -> f32 {
        self.plot[index]
    }

    pub fn resolution(&self) -> usize {
        self.plot.len()
    }
}
