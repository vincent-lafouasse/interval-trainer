pub struct SimpleNote {
    data: u8,
}

impl SimpleNote {
    pub fn octave(&self) -> i8 {
        (self.data / 12) as i8 - 1
    }
}
