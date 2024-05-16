use crate::{notes::Note, simple_note::SimpleNote};

use rand::Rng;
use std::fmt;

#[derive(Debug, Copy, Clone)]
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

    pub fn crop_bottom(&self, shift_amount: u8) -> Self {
        assert!(shift_amount < self.size());
        let shift_amount: i8 = shift_amount.try_into().unwrap();
        NoteRange::new(self.bottom.shift(shift_amount), self.top).unwrap()
    }

    pub fn crop_top(&self, shift_amount: u8) -> Self {
        assert!(shift_amount < self.size());
        let shift_amount: i8 = shift_amount.try_into().unwrap();
        NoteRange::new(self.bottom, self.top.shift(-shift_amount)).unwrap()
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

    pub fn size(&self) -> u8 {
        (self.top.data + 1 - self.bottom.data).try_into().unwrap()
    }

    pub fn alto_sax() -> Self {
        Self::from_str("Db3", "F6").unwrap()
    }
}

impl fmt::Display for NoteRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.bottom, self.top)
    }
}
