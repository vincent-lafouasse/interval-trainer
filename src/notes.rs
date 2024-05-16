use int_enum::IntEnum;
use std::fmt;

use crate::interval::*;
use crate::SimpleNote;

#[derive(Debug)]
pub struct Note {
    pub name: NoteName,
    pub alteration: i8,
    pub octave: i8,
}

impl Note {
    pub fn up(&self, interval: Interval) -> Note {
        let mut output = Note {name: self.name, alteration: 0, octave: self.octave};
        let notename_shift = u8::from(interval.base_interval);
        todo!()
    }

    pub fn distance_from(&self, other: Note) -> i8 {
        self.to_simple().data - other.to_simple().data
    }

    pub fn to_simple(&self) -> SimpleNote {
        SimpleNote {
            data: 12 * (self.octave + 1)
                + self.name.distance_from_c() as i8
                + self.alteration as i8,
        }
    }

    pub fn frequency(&self) -> f32 {
        let offset_from_a4: i8 = self.to_simple().data - 69;

        440.0 * 2.0_f32.powf(offset_from_a4 as f32 / 12.0)
    }

    pub fn parse_from_string(string: &str) -> Result<Note, &str> {
        let mut chars = string.chars();

        let name: NoteName = match chars.next() {
            None => return Err("no note given"),
            Some(note_name) => match note_name {
                'C' => NoteName::C,
                'D' => NoteName::D,
                'E' => NoteName::E,
                'F' => NoteName::F,
                'G' => NoteName::G,
                'A' => NoteName::A,
                'B' => NoteName::B,
                _ => return Err("invalid note name"),
            },
        };

        let mut alteration: i8 = match chars.next() {
            None => return Err("missing octave"),
            Some(c) => match c {
                '0'..='9' => return Ok(Note { name, alteration: 0, octave: c as i8 - '0' as i8 }),
                '#' => 1,
                'b' => -1,
                _ => return Err("invalid alteration"),
            },
        };

        alteration = match chars.next() {
            None => return Err("missing octave"),
            Some(c) => match c {
                '0'..='9' => return Ok(Note { name, alteration, octave: c as i8 - '0' as i8 }),
                '#' => match alteration {
                    1 => 2,
                    _ => return Err("invalid alterations"),
                },
                'b' => match alteration {
                    -1 => -2,
                    _ => return Err("invalid alterations"),
                },
                _ => return Err("invalid alteration"),
            },
        };

        match chars.next() {
            None => Err("missing octave"),
            Some(c) => match c {
                '0'..='9' => Ok(Note { name, alteration, octave: c as i8 - '0' as i8 }),
                _ => Err("invalid note"),
            },
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, IntEnum)]
#[repr(u8)]
pub enum NoteName {
    C = 0,
    D = 1,
    E = 2,
    F = 3,
    G = 4,
    A = 5,
    B = 6,
}

impl NoteName {
    pub fn distance_from_c(&self) -> u8 {
        match *self {
            NoteName::A => 9,
            NoteName::B => 11,
            NoteName::C => 0,
            NoteName::D => 2,
            NoteName::E => 4,
            NoteName::F => 5,
            NoteName::G => 7,
        }
    }

    pub fn shift_up(&self, shift: u8) -> Self {
        let mut out = *self;
         for _ in 0..(shift % 7) {
            out = out.next();
        }
        out
    }

    pub fn shift_down(&self, shift: u8) -> Self {
        let mut out = *self;
         for _ in 0..(shift % 7) {
            out = out.prev();
        }
        out
    }

    pub fn next(&self) -> Self {
        match self {
            NoteName::B => NoteName::C,
            _ => NoteName::try_from(u8::from(*self) + 1).unwrap(),
        }
    }

    pub fn prev(&self) -> Self {
        match self {
            NoteName::C => NoteName::B,
            _ => NoteName::try_from(u8::from(*self) - 1).unwrap(),
        }
    }
}

impl fmt::Display for NoteName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let repr = match *self {
            NoteName::A => "A",
            NoteName::B => "B",
            NoteName::C => "C",
            NoteName::D => "D",
            NoteName::E => "E",
            NoteName::F => "F",
            NoteName::G => "G",
        };
        write!(f, "{}", repr)
    }
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let alteration_repr = match self.alteration {
            -2 => "bb",
            -1 => "b",
            0 => "",
            1 => "#",
            2 => "##",
            _ => "X",
        };
        write!(f, "{}{}{}", self.name, alteration_repr, self.octave)
    }
}
