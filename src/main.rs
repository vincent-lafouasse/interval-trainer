mod intervals;
mod notes;

use color_eyre::eyre::Result;

#[allow(unused_imports)]
use crate::intervals::{BaseInterval, Interval, Quality};
#[allow(unused_imports)]
use crate::notes::{Alteration, Note, NoteName, CHROMATIC_NOTES_PER_OCTAVE};

fn main() -> Result<()> {
    color_eyre::install()?;

    let debugging = true;
    if debugging {
        debug();
    }

    let quizing = false;
    if quizing {
        quiz();
    }
    Ok(())
}

fn quiz() {
    let random_note = Note::get_random();
    println!("What note is \na perfect fifth above {}?", random_note);
    let user_note = Note::get_from_user();
    match (user_note.distance_from_c() - random_note.distance_from_c())
        .rem_euclid(CHROMATIC_NOTES_PER_OCTAVE)
        == 7
    {
        true => println!("ding ding you win"),
        false => println!("[EXTREMELY LOUD INCORRECT BUZZER]"),
    }
}

fn debug() {
    let random_interval = Interval::get_random_diatonic();

    println!("Here's a random interval :\n\t{}", random_interval);
    println!("Its size is {} semitones\n", random_interval.size());

    let c = Note {
        name: NoteName::C,
        alteration: Alteration::Natural,
    };

    let up_from_c: Note = random_interval.note_up_from(c);
    println!("{} is a {} up from C\n", up_from_c, random_interval);

    let note1 = Note::get_random();
    let note2 = Note::get_random();
    println!("Here are two random notes: {} and {}", note1, note2);
    println!("Between them is a {}", Interval::between(note1, note2));
}
