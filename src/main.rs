use rand::Rng;

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
}

fn main() {
    let random_note = Note::get_random();
    println!("You can have {}", random_note.to_string());
    println!("Your note is {} semitones from C", random_note.distance);
    let note = "Ab";
    let distance_result = distance_from_note_name(note);
    let distance = match distance_result {
        Ok(distance) => distance,
        Err(msg) => panic!("{}", msg),
    };
    println!("The note {} is {} semitones from C", note, distance);
}
