use std::fmt;

use crate::notes::{Note, NoteName};
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

    pub fn shift(&self, size: i8) -> Self {
        SimpleNote { data: self.data + size }
    }

    pub fn get_u8(&self) -> u8 {
        self.data.try_into().unwrap()
    }

    pub fn get_i8(&self) -> i8 {
        self.data
    }

    // spell(&self, policy: NamingPolicy) -> SpelledNote ?
    // with to_note_rand == spell(NamingPolicy::Rand)
    pub fn to_note_rand(self) -> Note {
        let mut rng = rand::thread_rng();
        let sharp: bool = rng.gen::<bool>();

        match self.data % 12 {
            0 => Note { name: NoteName::C, alteration: 0, octave: self.octave() },
            1 => match sharp {
                true => Note { name: NoteName::C, alteration: 1, octave: self.octave() },
                false => Note { name: NoteName::D, alteration: -1, octave: self.octave() },
            },
            2 => Note { name: NoteName::D, alteration: 0, octave: self.octave() },
            3 => match sharp {
                true => Note { name: NoteName::D, alteration: 1, octave: self.octave() },
                false => Note { name: NoteName::E, alteration: -1, octave: self.octave() },
            },
            4 => Note { name: NoteName::E, alteration: 0, octave: self.octave() },
            5 => Note { name: NoteName::F, alteration: 0, octave: self.octave() },
            6 => match sharp {
                true => Note { name: NoteName::F, alteration: 1, octave: self.octave() },
                false => Note { name: NoteName::G, alteration: -1, octave: self.octave() },
            },
            7 => Note { name: NoteName::G, alteration: 0, octave: self.octave() },
            8 => match sharp {
                true => Note { name: NoteName::G, alteration: 1, octave: self.octave() },
                false => Note { name: NoteName::A, alteration: -1, octave: self.octave() },
            },
            9 => Note { name: NoteName::A, alteration: 0, octave: self.octave() },
            10 => match sharp {
                true => Note { name: NoteName::A, alteration: 1, octave: self.octave() },
                false => Note { name: NoteName::B, alteration: -1, octave: self.octave() },
            },
            11 => Note { name: NoteName::B, alteration: 0, octave: self.octave() },
            _ => panic!("unreachable"),
        }
    }
}

pub enum NamingPolicy {
    Flats,
    Sharps,
    Random,
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
