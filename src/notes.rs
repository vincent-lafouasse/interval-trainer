use rand::Rng;
use std::io;

pub const N_NOTES: i8 = 12;

#[allow(dead_code)]
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
pub enum Alteration {
    NATURAL,
    FLAT,
    SHARP,
}

impl NoteName {
    pub fn repr(&self) -> &str {
        match &self {
            NoteName::C => "C",
            NoteName::D => "D",
            NoteName::E => "E",
            NoteName::F => "F",
            NoteName::G => "G",
            NoteName::A => "A",
            NoteName::B => "B",
        }
    }
}

impl Alteration {
    pub fn repr(&self) -> &str {
        match &self {
            Alteration::NATURAL => "",
            Alteration::FLAT => "b",
            Alteration::SHARP => "#",
        }
    }
}

pub struct Note {
    pub name: NoteName,
    pub alteration: Alteration,
}

impl Note {
    pub fn get_from_user() -> Note {
        println!("Please input a note:");
        let mut note: Note;
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
            0 => Alteration::NATURAL,
            1 => Alteration::FLAT,
            2 => Alteration::SHARP,
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
            Alteration::NATURAL => 0,
            Alteration::FLAT => -1,
            Alteration::SHARP => 1,
        };

        (base_distance + increment).rem_euclid(N_NOTES)
    }

    pub fn parse_from_string(string: &str) -> Result<Note, &str> {
        if string.len() == 0 || string.len() > 2 {
            return Err("Invalid note");
        }
        let note_name = match &string[0..1] {
            "A" => NoteName::A,
            "B" => NoteName::B,
            "C" => NoteName::C,
            "D" => NoteName::D,
            "E" => NoteName::E,
            "F" => NoteName::F,
            "G" => NoteName::G,
            _ => return Err("Invalid note"),
        };

        let mut alteration = Alteration::NATURAL;
        if string.len() == 2 {
            alteration = match &string[1..2] {
                "b" => Alteration::FLAT,
                "#" => Alteration::SHARP,
                _ => return Err("Invalid alteration"),
            };
        }
        Ok(Note {
            name: note_name,
            alteration: alteration,
        })
    }

    pub fn repr(&self) -> String {
        format!("{}{}", self.name.repr(), self.alteration.repr())
    }
}
