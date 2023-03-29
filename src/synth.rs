pub struct Oscillator {
    sample_rate: f32,
    wavetable: Wavetable,
    index: f32,
    index_increment: f32,
}

impl Oscillator {
    pub fn new(sample_rate: f32, wavetable: Wavetable) -> Self {
        Oscillator { sample_rate, wavetable, index: 0., index_increment: 0. }
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.index_increment = frequency * (self.wavetable.len() as f32) / self.sample_rate;
    }

    pub fn get_sample(&mut self) -> f32 {
        let sample = self.linear_interpolation();
        self.index += self.index_increment;
        self.index %= self.wavetable.len() as f32;

        sample
    }

    fn linear_interpolation(&self) -> f32 {
        let truncated_index = self.index as usize;
        let next_index = (truncated_index + 1) % self.wavetable.len();
        let next_index_weight = self.index - (truncated_index as f32);
        let truncated_index_weight = 1.0 - next_index_weight;

        truncated_index_weight * self.wavetable.at(truncated_index)
            + next_index_weight * self.wavetable.at(next_index)
    }
}

pub struct Wavetable {
    pub plot: Vec<f32>,
}

pub enum WavetableType {
    Sine,
    Square,
}

impl Wavetable {
    pub fn new(size: usize, wavetable_type: WavetableType) -> Self {
        use std::f32::consts::PI;
        let plot: Vec<f32> = match wavetable_type {
            WavetableType::Sine => (0..size)
                .map(|n| 2.0 * (n as f32) * PI / (size as f32))
                .map(|n| n.sin())
                .collect(),
            WavetableType::Square => (0..size)
                .map(|n| match 2 * n >= size {
                    true => 1.,
                    false => -1.,
                })
                .collect(),
        };
        Wavetable { plot }
    }

    pub fn at(&self, index: usize) -> f32 {
        self.plot[index]
    }

    pub fn len(&self) -> usize {
        self.plot.len()
    }
}
