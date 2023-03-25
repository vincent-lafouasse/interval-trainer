use rand::Rng;
use std::fmt;
use std::io;

pub const NOTES_PER_OCTAVE: i8 = 12;

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum NoteName {
    C,
    D,
    E,
    F,
    G,
    A,
    B,
}

#[allow(dead_code)]
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
            let parsing_result = Note::parse_from_string(&input.trim());
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

    pub fn get_random() -> Note {
        let rn_note_name = rand::thread_rng().gen_range(0, 7) % 7;
        let rn_alteration = rand::thread_rng().gen_range(0, 3) % 3;

        let note_name = match rn_note_name {
            0 => NoteName::A,
            1 => NoteName::B,
            2 => NoteName::C,
            3 => NoteName::D,
            4 => NoteName::E,
            5 => NoteName::F,
            6 => NoteName::G,
            _ => panic!(""),
        };

        let alteration = match rn_alteration {
            0 => Alteration::Natural,
            1 => Alteration::Flat,
            2 => Alteration::Sharp,
            _ => panic!(""),
        };

        Note {
            name: note_name,
            alteration: alteration,
        }
    }

    pub fn distance_from_c(&self) -> i8 {
        let base_distance: i8 = match self.name {
            NoteName::C => 0,
            NoteName::D => 2,
            NoteName::E => 4,
            NoteName::F => 5,
            NoteName::G => 7,
            NoteName::A => 9,
            NoteName::B => 11,
        };

        let increment: i8 = match self.alteration {
            Alteration::Natural => 0,
            Alteration::Flat => -1,
            Alteration::Sharp => 1,
        };

        (base_distance + increment).rem_euclid(NOTES_PER_OCTAVE)
    }

    pub fn parse_from_string(string: &str) -> Result<Note, &str> {
        if string.len() == 0 || string.len() > 2 {
            return Err("Note is a notename (A, B, etc) and an optional alteration (b or #)");
        }
        let note_name = match &string[0..1] {
            "A" => NoteName::A,
            "B" => NoteName::B,
            "C" => NoteName::C,
            "D" => NoteName::D,
            "E" => NoteName::E,
            "F" => NoteName::F,
            "G" => NoteName::G,
            _ => return Err("Invalid note name"),
        };

        let mut alteration = Alteration::Natural;
        if string.len() == 2 {
            alteration = match &string[1..2] {
                "b" => Alteration::Flat,
                "#" => Alteration::Sharp,
                _ => return Err("Invalid alteration"),
            };
        }
        Ok(Note {
            name: note_name,
            alteration: alteration,
        })
    }
}

impl NoteName {
    #[allow(dead_code)]
    pub fn next(&self) -> NoteName {
        match &self {
            NoteName::A => NoteName::B,
            NoteName::B => NoteName::C,
            NoteName::C => NoteName::D,
            NoteName::D => NoteName::E,
            NoteName::E => NoteName::F,
            NoteName::F => NoteName::G,
            NoteName::G => NoteName::A,
        }
    }

    #[allow(dead_code)]
    pub fn previous(&self) -> NoteName {
        match &self {
            NoteName::A => NoteName::G,
            NoteName::B => NoteName::A,
            NoteName::C => NoteName::B,
            NoteName::D => NoteName::C,
            NoteName::E => NoteName::D,
            NoteName::F => NoteName::E,
            NoteName::G => NoteName::F,
        }
    }

    #[allow(dead_code)]
    pub fn shift(note_name: NoteName, distance: i8) -> NoteName {
        let mut new_note = note_name;
        let actual_distance = distance.rem_euclid(7);
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

    const C: Note = Note {
        name: NoteName::C,
        alteration: Alteration::Natural,
    };
    const A_FLAT: Note = Note {
        name: NoteName::A,
        alteration: Alteration::Flat,
    };
    const D_FLAT: Note = Note {
        name: NoteName::D,
        alteration: Alteration::Flat,
    };
    const F_SHARP: Note = Note {
        name: NoteName::F,
        alteration: Alteration::Sharp,
    };
    const C_SHARP: Note = Note {
        name: NoteName::C,
        alteration: Alteration::Sharp,
    };
    const F_FLAT: Note = Note {
        name: NoteName::F,
        alteration: Alteration::Flat,
    };
    const B_SHARP: Note = Note {
        name: NoteName::B,
        alteration: Alteration::Sharp,
    };

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
            Note {
                name: NoteName::B,
                alteration: Alteration::Flat
            }
        );

        let a_sharp = Note::parse_from_string("A#");
        assert!(a_sharp.is_ok());
        assert_eq!(
            a_sharp.unwrap(),
            Note {
                name: NoteName::A,
                alteration: Alteration::Sharp
            }
        );

        let g_sharp = Note::parse_from_string("G#");
        assert!(g_sharp.is_ok());
        assert_eq!(
            g_sharp.unwrap(),
            Note {
                name: NoteName::G,
                alteration: Alteration::Sharp
            }
        );

        let c_flat = Note::parse_from_string("Cb");
        assert!(c_flat.is_ok());
        assert_eq!(
            c_flat.unwrap(),
            Note {
                name: NoteName::C,
                alteration: Alteration::Flat
            }
        );

        let d_natural = Note::parse_from_string("D");
        assert!(d_natural.is_ok());
        assert_eq!(
            d_natural.unwrap(),
            Note {
                name: NoteName::D,
                alteration: Alteration::Natural
            }
        );
    }

    #[test]
    fn test_parse_note_from_str_err() {
        let invalid_length = Note::parse_from_string("420");
        match invalid_length {
            Err(msg) => assert_eq!(
                msg,
                "Note is a notename (A, B, etc) and an optional alteration (b or #)"
            ),
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
