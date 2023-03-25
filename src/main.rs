mod intervals;
mod notes;

use color_eyre::eyre::Result;

use crate::intervals::{BaseInterval, Interval, Quality};
#[allow(unused_imports)]
use crate::notes::{Alteration, Note, NoteName, NOTES_PER_OCTAVE};

fn main() -> Result<()> {
    color_eyre::install()?;

    let debugging = false;
    if debugging {
        debug();
    }

    let quizing = true;
    if quizing {
        quiz();
    }
    Ok(())
}

fn debug() {
    let other_key = Note::parse_from_string("Eb").expect("Huh oh that's not a good note");
    println!(
        "Another good one is {}, which is {} semitones from C\n",
        other_key,
        other_key.distance_from_c()
    );

    let an_interval = Interval {
        base_interval: BaseInterval::Third,
        quality: Quality::Diminished,
    };

    println!("Here's an interval : {}", an_interval);
    println!("Its size is {} semitones", an_interval.size());
}

fn quiz() {
    let random_note = Note::get_random();
    println!("Whate note is \na perfect fifth above {}?", random_note);
    let user_note = Note::get_from_user();
    match (user_note.distance_from_c() - random_note.distance_from_c()).rem_euclid(NOTES_PER_OCTAVE)
        == 7
    {
        true => println!("ding ding you win"),
        false => println!("[EXTREMELY LOUD INCORRECT BUZZER]"),
    }
}
