use core::time::Duration;
use rodio::source::Source;

/// A wavetable oscillator that can play sound via the `rodio::source::Source` trait
#[derive(Copy, Clone)]
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
    pub plot: [f32; 256],
}

impl Wavetable {
    pub const fn new() -> Self {
        Wavetable { plot: SINE_256 }
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

pub const SINE_256: [f32; 256] = [
    0.0,
    0.024541229,
    0.049067676,
    0.07356457,
    0.09801714,
    0.12241068,
    0.14673047,
    0.1709619,
    0.19509032,
    0.21910124,
    0.2429802,
    0.26671278,
    0.29028466,
    0.31368175,
    0.33688986,
    0.35989505,
    0.38268346,
    0.40524134,
    0.42755508,
    0.44961134,
    0.47139674,
    0.49289823,
    0.51410276,
    0.53499764,
    0.55557024,
    0.5758082,
    0.5956993,
    0.61523163,
    0.63439333,
    0.65317285,
    0.671559,
    0.68954057,
    0.70710677,
    0.7242471,
    0.7409512,
    0.7572089,
    0.77301043,
    0.7883464,
    0.8032075,
    0.8175848,
    0.83146966,
    0.8448536,
    0.85772866,
    0.87008697,
    0.8819213,
    0.8932243,
    0.9039893,
    0.9142098,
    0.9238795,
    0.9329928,
    0.94154406,
    0.9495282,
    0.95694035,
    0.96377605,
    0.97003126,
    0.9757021,
    0.9807853,
    0.98527765,
    0.9891765,
    0.99247956,
    0.9951847,
    0.99729043,
    0.99879545,
    0.9996988,
    1.0,
    0.9996988,
    0.99879545,
    0.99729043,
    0.9951847,
    0.9924795,
    0.9891765,
    0.98527765,
    0.98078525,
    0.9757021,
    0.97003126,
    0.96377605,
    0.9569403,
    0.94952816,
    0.94154406,
    0.9329928,
    0.9238795,
    0.9142097,
    0.9039893,
    0.8932243,
    0.88192123,
    0.870087,
    0.8577286,
    0.8448535,
    0.83146954,
    0.8175848,
    0.8032075,
    0.78834635,
    0.7730105,
    0.7572088,
    0.74095106,
    0.724247,
    0.70710677,
    0.6895405,
    0.67155886,
    0.65317285,
    0.6343933,
    0.6152315,
    0.59569913,
    0.57580817,
    0.5555702,
    0.53499746,
    0.51410276,
    0.49289814,
    0.47139663,
    0.44961137,
    0.42755505,
    0.40524122,
    0.38268328,
    0.35989505,
    0.3368898,
    0.3136816,
    0.29028472,
    0.26671273,
    0.24298008,
    0.21910107,
    0.19509031,
    0.17096181,
    0.14673033,
    0.1224107,
    0.0980171,
    0.07356445,
    0.049067486,
    0.02454121,
    -0.00000008742278,
    -0.024541385,
    -0.04906766,
    -0.07356462,
    -0.09801727,
    -0.12241087,
    -0.1467305,
    -0.17096199,
    -0.19509049,
    -0.21910124,
    -0.24298024,
    -0.2667129,
    -0.29028487,
    -0.31368178,
    -0.33688995,
    -0.3598952,
    -0.38268343,
    -0.4052414,
    -0.42755523,
    -0.4496115,
    -0.47139677,
    -0.4928983,
    -0.5141029,
    -0.53499764,
    -0.5555703,
    -0.57580835,
    -0.5956993,
    -0.61523163,
    -0.6343934,
    -0.65317297,
    -0.671559,
    -0.6895406,
    -0.7071069,
    -0.7242471,
    -0.7409512,
    -0.75720876,
    -0.77301043,
    -0.78834647,
    -0.8032076,
    -0.81758493,
    -0.8314698,
    -0.84485376,
    -0.85772854,
    -0.87008697,
    -0.8819213,
    -0.89322436,
    -0.9039894,
    -0.91420984,
    -0.9238797,
    -0.93299276,
    -0.94154406,
    -0.9495282,
    -0.95694035,
    -0.9637761,
    -0.9700313,
    -0.9757022,
    -0.98078525,
    -0.98527765,
    -0.9891765,
    -0.99247956,
    -0.9951847,
    -0.9972905,
    -0.99879545,
    -0.9996988,
    -1.0,
    -0.9996988,
    -0.99879545,
    -0.99729043,
    -0.9951847,
    -0.9924795,
    -0.9891765,
    -0.98527765,
    -0.98078525,
    -0.9757021,
    -0.9700312,
    -0.963776,
    -0.95694023,
    -0.9495282,
    -0.94154406,
    -0.93299276,
    -0.92387944,
    -0.91420966,
    -0.90398914,
    -0.8932241,
    -0.8819213,
    -0.87008697,
    -0.85772854,
    -0.84485346,
    -0.8314695,
    -0.81758463,
    -0.8032076,
    -0.7883464,
    -0.77301043,
    -0.75720876,
    -0.740951,
    -0.7242469,
    -0.70710653,
    -0.6895406,
    -0.671559,
    -0.6531728,
    -0.63439316,
    -0.61523145,
    -0.5956991,
    -0.5758079,
    -0.5555703,
    -0.53499764,
    -0.5141027,
    -0.49289808,
    -0.47139654,
    -0.44961107,
    -0.4275548,
    -0.40524137,
    -0.38268343,
    -0.35989496,
    -0.3368897,
    -0.31368154,
    -0.2902844,
    -0.2667124,
    -0.24298023,
    -0.21910122,
    -0.19509023,
    -0.17096172,
    -0.14673024,
    -0.12241037,
    -0.09801677,
    -0.0735646,
    -0.04906764,
    -0.024541123,
];
