#![allow(dead_code)]

mod note_repr;

use crate::note_repr::Alteration;
use crate::note_repr::Clef;
use crate::note_repr::LilypondThing;
use crate::note_repr::Note;

fn main() {
    let a4 = Note::new('A', Alteration::NoAlteration, 4);
    let lily_file = LilypondFile { note: a4, clef: Clef::TrebleClef };
    println!("{}", lily_file.filename());
}

struct LilypondFile {
    note: Note,
    clef: Clef,
}

impl LilypondFile {
    fn filename(&self) -> String {
        let alteration_repr: String = match self.note.alteration {
            Alteration::NoAlteration => "".to_string(),
            Alteration::Flat => "b".to_string(),
            Alteration::DoubleFlat => "bb".to_string(),
            Alteration::Sharp => "s".to_string(),
            Alteration::DoubleSharp => "ss".to_string(),
        };

        format!(
            "{}{}{}_{}.ly",
            self.note.name,
            alteration_repr,
            self.note.octave,
            self.clef.lily_repr()
        )
    }
}
