pub struct Wavetable {
    pub plot: Vec<f32>,
}

pub enum WavetableType {
    Sin,
}

impl Wavetable {
    pub fn new(size: usize, wavetable_type: WavetableType) -> Self {
        use std::f32::consts::PI;
        let plot: Vec<f32> = match wavetable_type {
            WavetableType::Sin => (0..size)
                .map(|n| 2.0 * (n as f32) * PI / (size as f32))
                .map(|n| n.sin())
                .collect(),
        };
        Wavetable { plot }
    }
}
