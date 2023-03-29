pub struct Wavetable {
    pub plot: Vec<f32>,
}

impl Wavetable {
    pub fn new(size: usize) -> Self {
        let mut plot: Vec<f32> = Vec::with_capacity(size);
        for n in 0..size {
            plot.push((2.0 * std::f32::consts::PI * (n as f32) / (size as f32)).sin());
        }
        Wavetable { plot }
    }
}
