use crate::{notes::Note, simple_note::SimpleNote};

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

    pub fn alto_sax() -> Self {
        NoteRange::from_str("Db3", "F6").unwrap()
    }
}

impl fmt::Display for NoteRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.bottom, self.top)
    }
}
