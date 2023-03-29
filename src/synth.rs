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

    pub fn len(&self) -> usize {
        self.plot.len()
    }
}
