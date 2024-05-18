#![allow(dead_code)]
#![allow(unused_variables)]

mod note_repr;

use std::fs::File;

use crate::note_repr::Alteration;
use crate::note_repr::Clef;
use crate::note_repr::LilypondThing;
use crate::note_repr::Note;

fn main() -> std::io::Result<()> {
    let a4 = Note::new('A', Alteration::NoAlteration, 4);
    let lily_file = LilypondFile { note: a4, clef: Clef::TrebleClef };
    println!("{}", lily_file.filename());

    Ok(())
}

struct LilypondFile {
    note: Note,
    clef: Clef,
}

impl LilypondFile {
    fn write(&self, output_dir: String) -> std::io::Result<()> {
        let mut file = File::options()
            .append(true)
            .open(output_dir + &self.filename())?;

        writeln!(&mut file, "\\version \"2.22.2\"")?;
        writeln!(&mut file, "#(set-default-paper-size '(cons (* 125 pt) (* 50 pt)))")?;
        writeln!(&mut file, "\\header { tagline = \" \" }")?;
        writeln!(&mut file, "\\new Staff \\with {")?;
        writeln!(&mut file, "	\\override TimeSignature.stencil = ##f")?;
        writeln!(&mut file, "}{")?;
        writeln!(&mut file, "	\\time 100/2 % no bar lines (probably)")?;
        writeln!(&mut file, "	\\clef {self.clef.get()}")?;
        writeln!(&mut file, format!("	\\clef {}", self.clef.lily_repr()))?;
        writeln!(&mut file, "	\\key c \\major")?;
        writeln!(&mut file, "	| {self.note.ly_repr()}!1 {self.note.ly_repr()}!1 | \n")?;
        writeln!(&mut file, "}")?;

        Ok(())
    }

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
