use rand::Rng;
use std::io;

pub const NOTES: [&str; 12] = [
    "C", "Db", "D", "Eb", "E", "F", "F#", "G", "Ab", "A", "Bb", "B",
];

pub enum NoteName {
    C,
    D,
    E,
    F,
    G,
    A,
    B,
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

enum IntervalQuality {
    MAJOR,
    MINOR,
    DIMINISHED,
    AUGMENTED,
}

pub enum Alteration {
    NATURAL,
    FLAT,
    SHARP,
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

pub struct AltNote {
    pub name: NoteName,
    pub alteration: Alteration,
}

impl AltNote {
    pub fn repr(&self) -> String {
        format!("{}{}", self.name.repr(), self.alteration.repr())
    }
}

pub struct Note {
    pub distance: usize,
}

impl Note {
    pub fn to_string(&self) -> &str {
        match self.distance {
            0..=12 => return NOTES[self.distance % 12],
            _ => return "wtf",
        }
    }
    pub fn get_random() -> Note {
        let random_distance = rand::thread_rng().gen_range(0, 12);
        Note {
            distance: random_distance,
        }
    }

    pub fn get_from_user() -> Note {
        let mut input_note_name = String::new();
        io::stdin()
            .read_line(&mut input_note_name)
            .expect("Failed to read line");

        Note::parse_from_name(&input_note_name.trim())
    }

    fn parse_from_name(name: &str) -> Note {
        if name.len() == 0 || name.len() > 3 {
            panic!("Invalid note");
        }

        let mut distance = 0;

        for (index, token) in name.chars().enumerate() {
            if index == 0 {
                distance = match token {
                    'C' => 0,
                    'D' => 2,
                    'E' => 4,
                    'F' => 5,
                    'G' => 7,
                    'A' => 9,
                    'B' => 11,
                    _ => panic!("Invalid note"),
                };
            } else {
                match token {
                    '#' => distance += 1,
                    'b' => distance -= 1,
                    _ => panic!("Invalid alteration"),
                }
            }
        }
        return Note {
            distance: (distance + 12) % 12,
        };
    }
}
