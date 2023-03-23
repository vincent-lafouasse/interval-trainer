use rand::Rng;
use std::io;
#[allow(dead_code)]

pub const NOTES: [&str; 12] = [
    "C", "Db", "D", "Eb", "E", "F", "F#", "G", "Ab", "A", "Bb", "B",
];

enum NoteName {
    C,
    D,
    E,
    F,
    G,
    A,
    B,
}

enum IntervalQuality {
    MAJOR,
    MINOR,
    DIMINISHED,
    AUGMENTED,
}

enum Alteration {
    NATURAL,
    FLAT,
    SHARP,
}

struct AltNote {
    name: NoteName,
    alteration: Option<Alteration>,
}

impl AltNote {}

struct Note {
    distance: usize,
}

impl Note {
    fn to_string(&self) -> &str {
        match self.distance {
            0..=12 => return NOTES[self.distance % 12],
            _ => return "wtf",
        }
    }
    fn get_random() -> Note {
        let random_distance = rand::thread_rng().gen_range(0, 12);
        Note {
            distance: random_distance,
        }
    }

    fn get_from_user() -> Note {
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

fn main() {
    let random_note = Note::get_random();
    println!("Whate note is a fifth above {}?", random_note.to_string());
    let user_note = Note::get_from_user();
    let distance = (user_note.distance + 12 - random_note.distance) % 12;
    match distance {
        7 => println!("ding ding you win"),
        _ => println!("[EXTREMELY LOUD INCORRECT BUZZER]"),
    }
}
