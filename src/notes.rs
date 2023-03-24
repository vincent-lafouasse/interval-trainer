use rand::Rng;
use std::io;

pub const NOTES: [&str; 12] = [
    "C", "Db", "D", "Eb", "E", "F", "F#", "G", "Ab", "A", "Bb", "B",
];

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

pub struct AltNote {
    pub name: NoteName,
    pub alteration: Alteration,
}

impl AltNote {
    pub fn repr(&self) -> String {
        format!("{}{}", self.name.repr(), self.alteration.repr())
    }

    pub fn distance_from_c(&self) -> i8 {
        let base_distance = match self.name {
            NoteName::C => 0,
            NoteName::D => 2,
            NoteName::E => 4,
            NoteName::F => 5,
            NoteName::G => 7,
            NoteName::A => 9,
            NoteName::B => 11,
        };

        let increment = match self.alteration {
            Alteration::NATURAL => 0,
            Alteration::FLAT => -1,
            Alteration::SHARP => 1,
        };

        (12 + base_distance + increment) % 12
    }

    pub fn parse_from_string(string: &str) -> AltNote {
        if string.len() == 0 || string.len() > 2 {
            panic!("Invalid note");
        }
        let note_name = match &string[0..1] {
            "A" => NoteName::A,
            "B" => NoteName::B,
            "C" => NoteName::C,
            "D" => NoteName::D,
            "E" => NoteName::E,
            "F" => NoteName::F,
            "G" => NoteName::G,
            _ => panic!("Invalid note"),
        };

        let mut alteration = Alteration::NATURAL;
        if string.len() == 2 {
            alteration = match &string[1..2] {
                "b" => Alteration::FLAT,
                "#" => Alteration::SHARP,
                _ => panic!("Invalid alteration"),
            };
        }
        AltNote {
            name: note_name,
            alteration: alteration,
        }
    }
}

pub struct Note {
    pub distance: usize,
}

impl Note {
    pub fn is_a_fifth_above(&self, other: &Note) -> bool {
        (self.distance + 12 - other.distance) % 12 == 7
    }

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
