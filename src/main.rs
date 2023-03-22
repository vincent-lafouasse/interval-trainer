use rand::Rng;
use std::io;

pub const NOTES: [&str; 12] = [
    "C", "Db", "D", "Eb", "E", "F", "F#", "G", "Ab", "A", "Bb", "B",
];

fn distance_from_note_name(name: &str) -> Result<usize, &'static str> {
    for i in 0..12 {
        if name == NOTES[i] {
            return Ok(i);
        }
    }
    return Err("Not a note");
}

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
        let distance: usize = match distance_from_note_name(input_note_name.trim()) {
            Ok(num) => num,
            Err(msg) => panic!("{}", msg),
        };
        Note { distance: distance }
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
