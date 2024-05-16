use crate::Note;

use std::fmt;

#[derive(Debug)]
pub struct NoteRange {
    pub start: Note,
    pub size: u8,
}

impl NoteRange {
    pub fn new(start: Note, size: u8) -> Self {
        NoteRange { start, size }
    }

    pub fn from_notes(start: Note, end: Note) -> Self {
        let size: u8 = (start.to_midi_style() - end.to_midi_style())
            .try_into()
            .expect("Invalid note range");
        NoteRange { start, size }
    }

    pub fn alto_sax() -> Self {
        NoteRange { start: Note::parse_from_string("Db3").unwrap(), size: 32 }
    }
}

impl fmt::Display for NoteRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.start, self.size)
    }
}
