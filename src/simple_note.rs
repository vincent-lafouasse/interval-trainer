use std::fmt;

pub struct SimpleNote {
    pub data: u8,
}

impl SimpleNote {
    pub fn octave(&self) -> i8 {
        (self.data / 12) as i8 - 1
    }
}

impl fmt::Display for SimpleNote {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self.data / 12 {
            0 => "C",
            1 => "C#/Db",
            2 => "D",
            3 => "Eb/D#",
            4 => "E",
            5 => "F",
            6 => "F#/Gb",
            7 => "G",
            8 => "Ab/G#",
            9 => "A",
            10 => "Bb/A#",
            11 => "B",
            _ => "X",
        };

        write!(f, "{}{}", name, self.octave())
    }
}
