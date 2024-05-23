use crate::music::{note::Note, simple_note::SimpleNote};

use rand::Rng;
use std::fmt;

#[derive(Debug, Copy, Clone)]
pub struct NoteRange {
    pub bottom: SimpleNote,
    pub top: SimpleNote,
}

impl NoteRange {
    pub fn new(bottom: SimpleNote, top: SimpleNote) -> Result<Self, &'static str> {
        match top.get_u8() > bottom.get_u8() {
            true => Ok(NoteRange { bottom, top }),
            false => Err("Invalid note range"),
        }
    }

    pub fn crop_bottom(&self, shift_amount: i8) -> Self {
        assert!(shift_amount < self.size_i8());

        NoteRange::new(self.bottom.shift(shift_amount), self.top).unwrap()
    }

    pub fn crop_top(&self, shift_amount: i8) -> Self {
        assert!(shift_amount < self.size_i8());

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
        let rn: i8 = rng.gen_range(0..=self.size_i8());
        let note: i8 = self.bottom.get_i8() + rn;

        SimpleNote::new(note).to_note_rand()
    }

    pub fn size_u8(&self) -> u8 {
        self.top.get_u8() + 1 - self.bottom.get_u8()
    }

    pub fn size_i8(&self) -> i8 {
        self.top.get_i8() + 1 - self.bottom.get_i8()
    }

    pub fn alto_sax() -> Self {
        Self::from_str("Db3", "F6").unwrap()
    }

    pub fn tenor_voice() -> Self {
        Self::from_str("Bb2", "G4").unwrap()
    }

    pub fn treble_staff() -> Self {
        Self::from_str("E3", "A5").unwrap()
    }
}

impl fmt::Display for NoteRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.bottom, self.top)
    }
}
