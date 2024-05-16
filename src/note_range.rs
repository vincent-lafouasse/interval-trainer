use crate::{notes::Note, simple_note::SimpleNote};

use rand::Rng;
use std::fmt;

#[derive(Debug)]
pub struct NoteRange {
    pub bottom: SimpleNote,
    pub top: SimpleNote,
}

impl NoteRange {
    pub fn new(bottom: SimpleNote, top: SimpleNote) -> Result<Self, &'static str> {
        match top > bottom {
            true => Ok(NoteRange { bottom, top }),
            false => Err("Invalid note range"),
        }
    }

    pub fn from_str(bottom_str: &str, top_str: &str) -> Result<Self, &'static str> {
        let bottom_note = Note::parse_from_string(bottom_str).unwrap();
        let top_note = Note::parse_from_string(top_str).unwrap();
        let bottom = bottom_note.to_simple();
        let top = top_note.to_simple();

        NoteRange::new(bottom, top)
    }

    pub fn rand(&self) -> Note {
        let mut rng = rand::thread_rng();
        let top = self.top.data;
        let bottom = self.bottom.data;
        let size: u8 = (top + 1 - bottom).try_into().unwrap();

        let rn: i8 = (rng.gen::<u8>() % size).try_into().unwrap();

        let note: i8 = bottom + rn;
        println!("{}", note);

        SimpleNote::new(note).to_note_rand()
    }

    pub fn alto_sax() -> Self {
        NoteRange::from_str("Db3", "F6").unwrap()
    }
}

impl fmt::Display for NoteRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.bottom, self.top)
    }
}
