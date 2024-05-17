mod note_repr;

use crate::note_repr::Alteration;
use crate::note_repr::LilypondThing;
use crate::note_repr::Note;

fn main() {
    let a4 = Note::new('A', Alteration::NoAlteration, 4);
    println!("{}", a4.lily_repr());

    let cbb7 = Note::new('C', Alteration::DoubleFlat, 7);
    println!("{}", cbb7.lily_repr());

    let f_dub_sharp2 = Note::new('F', Alteration::DoubleSharp, 1);
    println!("{}", f_dub_sharp2.lily_repr());
}
