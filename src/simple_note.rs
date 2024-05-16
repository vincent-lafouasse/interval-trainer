use std::fmt;

use crate::notes::NoteName;
use crate::Note;
use rand::Rng;

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub struct SimpleNote {
    pub data: i8,
}

impl SimpleNote {
    pub fn new(data: i8) -> Self {
        SimpleNote { data }
    }

    pub fn octave(&self) -> i8 {
        (self.data / 12) - 1
    }

    pub fn shift(input: Self, size: i8) -> Self {
        SimpleNote { data: input.data + size }
    }

    pub fn to_note_rand(&self) -> Note {
        let mut rng = rand::thread_rng();
        let sharp: u8 = rng.gen::<u8>() % 2;

        match self.data % 12 {
            0 => Note { name: NoteName::C, alteration: 0, octave: self.octave() },
            1 => match sharp {
                1 => Note { name: NoteName::C, alteration: 1, octave: self.octave() },
                0 => Note { name: NoteName::D, alteration: -1, octave: self.octave() },
                _ => panic!("unreachable"),
            },
            2 => Note { name: NoteName::D, alteration: 0, octave: self.octave() },
            3 => match sharp {
                1 => Note { name: NoteName::D, alteration: 1, octave: self.octave() },
                0 => Note { name: NoteName::E, alteration: -1, octave: self.octave() },
                _ => panic!("unreachable"),
            },
            4 => Note { name: NoteName::E, alteration: 0, octave: self.octave() },
            5 => Note { name: NoteName::F, alteration: 0, octave: self.octave() },
            6 => match sharp {
                1 => Note { name: NoteName::F, alteration: 1, octave: self.octave() },
                0 => Note { name: NoteName::G, alteration: -1, octave: self.octave() },
                _ => panic!("unreachable"),
            },
            7 => Note { name: NoteName::G, alteration: 0, octave: self.octave() },
            8 => match sharp {
                1 => Note { name: NoteName::G, alteration: 1, octave: self.octave() },
                0 => Note { name: NoteName::A, alteration: -1, octave: self.octave() },
                _ => panic!("unreachable"),
            },
            9 => Note { name: NoteName::A, alteration: 0, octave: self.octave() },
            10 => match sharp {
                1 => Note { name: NoteName::A, alteration: 1, octave: self.octave() },
                0 => Note { name: NoteName::B, alteration: -1, octave: self.octave() },
                _ => panic!("unreachable"),
            },
            11 => Note { name: NoteName::B, alteration: 0, octave: self.octave() },
            _ => panic!("unreachable"),
        }
    }
}

impl fmt::Display for SimpleNote {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self.data % 12 {
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
