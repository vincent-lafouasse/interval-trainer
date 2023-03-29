use int_enum::IntEnum;
use std::fmt;
use std::io;

pub const CHROMATIC_NOTES_PER_OCTAVE: usize = 12;
pub const DIATONIC_NOTES_PER_OCTAVE: usize = 7;

#[derive(Copy, Clone, Debug, PartialEq, IntEnum)]
#[repr(usize)]
pub enum NoteName {
    C = 0,
    D = 1,
    E = 2,
    F = 3,
    G = 4,
    A = 5,
    B = 6,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Alteration {
    Natural,
    Flat,
    Sharp,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Note {
    pub name: NoteName,
    pub alteration: Alteration,
}

impl Note {
    pub fn get_from_user() -> Note {
        println!("Please input a note:");
        let note: Note;
        loop {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            let parsing_result = Note::parse_from_string(input.trim());
            match parsing_result {
                Ok(parsed_note) => {
                    note = parsed_note;
                    break;
                }
                Err(message) => {
                    println!("{}", message);
                    continue;
                }
            }
        }
        note
    }

    pub fn get_random_biased() -> Note {
        use rand::prelude::*;
        let mut rng = thread_rng();
        let nat_bias = 2;
        let notes = [
            // All 0 or 1 alteration notes except Cb Fb E# and B#
            (
                Note { name: NoteName::C, alteration: Alteration::Natural },
                nat_bias,
            ),
            (
                Note { name: NoteName::D, alteration: Alteration::Natural },
                nat_bias,
            ),
            (
                Note { name: NoteName::E, alteration: Alteration::Natural },
                nat_bias,
            ),
            (
                Note { name: NoteName::F, alteration: Alteration::Natural },
                nat_bias,
            ),
            (
                Note { name: NoteName::G, alteration: Alteration::Natural },
                nat_bias,
            ),
            (
                Note { name: NoteName::A, alteration: Alteration::Natural },
                nat_bias,
            ),
            (
                Note { name: NoteName::B, alteration: Alteration::Natural },
                nat_bias,
            ),
            (Note { name: NoteName::D, alteration: Alteration::Flat }, 1),
            (Note { name: NoteName::E, alteration: Alteration::Flat }, 1),
            (Note { name: NoteName::G, alteration: Alteration::Flat }, 1),
            (Note { name: NoteName::A, alteration: Alteration::Flat }, 1),
            (Note { name: NoteName::B, alteration: Alteration::Flat }, 1),
            (Note { name: NoteName::C, alteration: Alteration::Sharp }, 1),
            (Note { name: NoteName::D, alteration: Alteration::Sharp }, 1),
            (Note { name: NoteName::F, alteration: Alteration::Sharp }, 1),
            (Note { name: NoteName::G, alteration: Alteration::Sharp }, 1),
            (Note { name: NoteName::A, alteration: Alteration::Sharp }, 1),
        ];

        notes.choose_weighted(&mut rng, |item| item.1).unwrap().0
    }

    pub fn distance_from_c(&self) -> isize {
        let base_distance: isize = match self.name {
            NoteName::C => 0,
            NoteName::D => 2,
            NoteName::E => 4,
            NoteName::F => 5,
            NoteName::G => 7,
            NoteName::A => 9,
            NoteName::B => 11,
        };

        let increment: isize = match self.alteration {
            Alteration::Natural => 0,
            Alteration::Flat => -1,
            Alteration::Sharp => 1,
        };

        (base_distance + increment).rem_euclid(CHROMATIC_NOTES_PER_OCTAVE as isize)
    }

    pub fn parse_from_string(string: &str) -> Result<Note, &str> {
        let mut chars = string.chars();

        let name = match chars.next() {
            None => return Err("No note has been inputed"),
            Some(note_name) => match note_name {
                'A' => NoteName::A,
                'B' => NoteName::B,
                'C' => NoteName::C,
                'D' => NoteName::D,
                'E' => NoteName::E,
                'F' => NoteName::F,
                'G' => NoteName::G,
                _ => return Err("Invalid note name"),
            },
        };

        let alteration = match chars.next() {
            None => return Ok(Note { name, alteration: Alteration::Natural }),
            Some(_alteration) => match _alteration {
                '#' => Alteration::Sharp,
                'b' => Alteration::Flat,
                _ => return Err("Invalid alteration"),
            },
        };

        Ok(Note { name, alteration })
    }
}

impl NoteName {
    pub fn next(&self) -> Self {
        let new_int_value = (self.int_value() + 1).rem_euclid(DIATONIC_NOTES_PER_OCTAVE);
        NoteName::from_int(new_int_value).expect("couldn't make new NoteName from int")
    }

    pub fn shift(note_name: NoteName, distance: isize) -> NoteName {
        let mut new_note = note_name;
        let actual_distance = distance.rem_euclid(DIATONIC_NOTES_PER_OCTAVE as isize);
        for _ in 0..actual_distance {
            new_note = new_note.next();
        }
        new_note
    }
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.name, self.alteration)
    }
}

impl fmt::Display for NoteName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let repr = match &self {
            NoteName::C => "C",
            NoteName::D => "D",
            NoteName::E => "E",
            NoteName::F => "F",
            NoteName::G => "G",
            NoteName::A => "A",
            NoteName::B => "B",
        };
        write!(f, "{}", repr)
    }
}

impl fmt::Display for Alteration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let repr = match &self {
            Alteration::Natural => "",
            Alteration::Flat => "b",
            Alteration::Sharp => "#",
        };
        write!(f, "{}", repr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const C: Note = Note { name: NoteName::C, alteration: Alteration::Natural };
    const A_FLAT: Note = Note { name: NoteName::A, alteration: Alteration::Flat };
    const D_FLAT: Note = Note { name: NoteName::D, alteration: Alteration::Flat };
    const F_SHARP: Note = Note { name: NoteName::F, alteration: Alteration::Sharp };
    const C_SHARP: Note = Note { name: NoteName::C, alteration: Alteration::Sharp };
    const F_FLAT: Note = Note { name: NoteName::F, alteration: Alteration::Flat };
    const B_SHARP: Note = Note { name: NoteName::B, alteration: Alteration::Sharp };

    #[test]
    fn test_distance_from_c() {
        assert_eq!(0, C.distance_from_c());
        assert_eq!(8, A_FLAT.distance_from_c());
        assert_eq!(1, D_FLAT.distance_from_c());
        assert_eq!(6, F_SHARP.distance_from_c());
        assert_eq!(1, C_SHARP.distance_from_c());
        assert_eq!(4, F_FLAT.distance_from_c());
        assert_eq!(0, B_SHARP.distance_from_c());
    }

    #[test]
    fn test_note_display() {
        assert_eq!(format!("{}", C), "C");
        assert_eq!(format!("{}", A_FLAT), "Ab");
        assert_eq!(format!("{}", D_FLAT), "Db");
        assert_eq!(format!("{}", F_SHARP), "F#");
        assert_eq!(format!("{}", C_SHARP), "C#");
        assert_eq!(format!("{}", F_FLAT), "Fb");
        assert_eq!(format!("{}", B_SHARP), "B#");
    }

    #[test]
    fn test_note_name_shift() {
        let note_name = NoteName::A;
        assert_eq!(NoteName::shift(note_name, 0), NoteName::A);
        assert_eq!(NoteName::shift(note_name, 7 * 3), NoteName::A);
        assert_eq!(NoteName::shift(note_name, 7 * 7), NoteName::A);
        assert_eq!(NoteName::shift(note_name, 7 * 12), NoteName::A);

        assert_eq!(NoteName::shift(note_name, 0), NoteName::A);
        assert_eq!(NoteName::shift(note_name, 2), NoteName::C);
        assert_eq!(NoteName::shift(note_name, 4), NoteName::E);
        assert_eq!(NoteName::shift(note_name, 6), NoteName::G);
        assert_eq!(NoteName::shift(note_name, 8), NoteName::B);

        assert_eq!(NoteName::shift(note_name, -2), NoteName::F);
        assert_eq!(NoteName::shift(note_name, -4), NoteName::D);
        assert_eq!(NoteName::shift(note_name, -6), NoteName::B);
        assert_eq!(NoteName::shift(note_name, -8), NoteName::G);
    }

    #[test]
    fn test_parse_note_from_str_ok() {
        let b_flat = Note::parse_from_string("Bb");
        assert!(b_flat.is_ok());
        assert_eq!(
            b_flat.unwrap(),
            Note { name: NoteName::B, alteration: Alteration::Flat }
        );

        let a_sharp = Note::parse_from_string("A#");
        assert!(a_sharp.is_ok());
        assert_eq!(
            a_sharp.unwrap(),
            Note { name: NoteName::A, alteration: Alteration::Sharp }
        );

        let g_sharp = Note::parse_from_string("G#");
        assert!(g_sharp.is_ok());
        assert_eq!(
            g_sharp.unwrap(),
            Note { name: NoteName::G, alteration: Alteration::Sharp }
        );

        let c_flat = Note::parse_from_string("Cb");
        assert!(c_flat.is_ok());
        assert_eq!(
            c_flat.unwrap(),
            Note { name: NoteName::C, alteration: Alteration::Flat }
        );

        let d_natural = Note::parse_from_string("D");
        assert!(d_natural.is_ok());
        assert_eq!(
            d_natural.unwrap(),
            Note { name: NoteName::D, alteration: Alteration::Natural }
        );
    }

    #[test]
    fn test_parse_note_from_str_err() {
        let invalid_length = Note::parse_from_string("");
        match invalid_length {
            Err(msg) => assert_eq!(msg, "No note has been inputed"),
            Ok(_) => panic!("Poorly written tests"),
        }

        let invalid_note_name = Note::parse_from_string("H");
        match invalid_note_name {
            Err(msg) => assert_eq!(msg, "Invalid note name"),
            Ok(_) => panic!("Poorly written tests"),
        }

        let invalid_alteration = Note::parse_from_string("C+");
        match invalid_alteration {
            Err(msg) => assert_eq!(msg, "Invalid alteration"),
            Ok(_) => panic!("Poorly written tests"),
        }
    }
}
