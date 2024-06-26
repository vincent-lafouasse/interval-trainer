use int_enum::IntEnum;
use std::cmp::Ordering;
use std::fmt;

use crate::music::{interval::Interval, simple_note::SimpleNote};

#[derive(Debug, Copy, Clone)]
pub struct Note {
    pub name: NoteName,
    pub alteration: i8,
    pub octave: i8,
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

impl Note {
    pub fn up(&self, interval: Interval) -> Note {
        let new_notename: u8 = u8::from(self.name) + u8::from(interval.base_interval);
        let octave_shift: i8 = new_notename.try_into().unwrap();
        let octave_shift: i8 = octave_shift / 7;

        let name: NoteName = NoteName::try_from(new_notename % 7).unwrap();
        let octave: i8 = self.octave + octave_shift;
        let alteration: i8 = interval.size_i8()
            - Note { name, alteration: 0, octave }.chromatic_distance_up_from(*self);

        Note { name, alteration, octave }
    }

    pub fn chromatic_distance_up_from(&self, other: Note) -> i8 {
        self.to_simple().get_i8() - other.to_simple().get_i8()
    }

    pub fn diatonic_distance(from: Note, to: Note) -> i8 {
        match from.octave.cmp(&to.octave) {
            Ordering::Equal => NoteName::diatonic_distance(from.name, to.name),
            Ordering::Greater => {
                let octave_difference = to.octave - from.octave;
                7 * (octave_difference - 1)
                    + 1
                    + NoteName::diatonic_distance(from.name, NoteName::B)
                    + NoteName::diatonic_distance(NoteName::C, to.name)
            }
            Ordering::Less => -Note::diatonic_distance(to, from),
        }
    }

    pub fn to_simple(self) -> SimpleNote {
        SimpleNote {
            data: 12 * (self.octave + 1) + self.name.semitones_from_c() as i8 + self.alteration,
        }
    }

    pub fn frequency(&self) -> f64 {
        let offset_from_a4: i8 = self.to_simple().get_i8() - 69;

        440.0 * 2.0_f64.powf(offset_from_a4 as f64 / 12.0)
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

impl NoteName {
    pub fn semitones_from_c(&self) -> u8 {
        match *self {
            NoteName::C => 0,
            NoteName::D => 2,
            NoteName::E => 4,
            NoteName::F => 5,
            NoteName::G => 7,
            NoteName::A => 9,
            NoteName::B => 11,
        }
    }

    // disgusting i know but there is no risk of panic
    pub fn diatonic_distance(from: Self, to: Self) -> i8 {
        <u8 as TryInto<i8>>::try_into(u8::from(to)).unwrap()
            - <u8 as TryInto<i8>>::try_into(u8::from(from)).unwrap()
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

#[cfg(test)]
mod tests {
    use super::*;

    const C4: Note = Note { name: NoteName::C, alteration: 0, octave: 4 };
    const D4: Note = Note { name: NoteName::D, alteration: 0, octave: 4 };
    const A4: Note = Note { name: NoteName::A, alteration: 0, octave: 4 };
    const B4: Note = Note { name: NoteName::B, alteration: 0, octave: 4 };

    const C5: Note = Note { name: NoteName::C, alteration: 0, octave: 5 };
    const D5: Note = Note { name: NoteName::D, alteration: 0, octave: 5 };
    const A5: Note = Note { name: NoteName::A, alteration: 0, octave: 5 };
    const B5: Note = Note { name: NoteName::B, alteration: 0, octave: 5 };

    const C6: Note = Note { name: NoteName::C, alteration: 0, octave: 6 };
    const D6: Note = Note { name: NoteName::D, alteration: 0, octave: 6 };
    const A6: Note = Note { name: NoteName::A, alteration: 0, octave: 6 };
    const B6: Note = Note { name: NoteName::B, alteration: 0, octave: 6 };

    #[test]
    fn diatonic_distance_same_octave() {
        assert_eq!(Note::diatonic_distance(A4, C4), -5);
        assert_eq!(Note::diatonic_distance(C4, A4), 5);
        assert_eq!(Note::diatonic_distance(C4, D4), 1);
        assert_eq!(Note::diatonic_distance(B4, D4), -5);
    }

    #[test]
    fn diatonic_distance_octave_up() {
        assert_eq!(Note::diatonic_distance(A4, C5), 2);
        assert_eq!(Note::diatonic_distance(C4, A5), 5 + 7);
        assert_eq!(Note::diatonic_distance(C4, A6), 5 + 7 + 7);
        assert_eq!(Note::diatonic_distance(C4, D5), 1 + 7);
        assert_eq!(Note::diatonic_distance(C4, D6), 1 + 7 + 7);
        assert_eq!(Note::diatonic_distance(C4, B6), 6 + 7 + 7);
    }

    #[test]
    fn diatonic_distance_octave_down() {
        assert_eq!(Note::diatonic_distance(C5, A4), -2);
        assert_eq!(Note::diatonic_distance(C6, A5), -2);
        assert_eq!(Note::diatonic_distance(C6, A4), -(2 + 7));
        assert_eq!(Note::diatonic_distance(B6, D5), -(5 + 7));
        assert_eq!(Note::diatonic_distance(C6, D4), -(6 + 7));
        assert_eq!(Note::diatonic_distance(B6, C4), -(6 + 14));
    }
}
