mod intervals;
mod notes;

use crate::intervals::{BaseInterval, Interval, IntervalQualifier};
use crate::notes::{Alteration, Note, NoteName, N_NOTES};

fn main() {
    let note_name = NoteName::A;
    println!(
        "{}{}{}{}{}{}{}{}",
        note_name,
        note_name.next(),
        note_name.next().next(),
        note_name.next().next().next(),
        note_name.next().next().next().next(),
        note_name.next().next().next().next().next(),
        note_name.next().next().next().next().next().next(),
        note_name.next().next().next().next().next().next().next(),
    );

    let fave_key = Note {
        name: NoteName::D,
        alteration: Alteration::Flat,
    };
    println!("My favorite key is {}", fave_key);
    println!("It's {} semitones from C\n", fave_key.distance_from_c());

    println!("Cool new way to print Notes: {}", fave_key);

    let other_key = Note::parse_from_string("Eb").expect("Huh oh that's not a good note");
    println!(
        "Another good one is {}, which is {} semitones from C\n",
        other_key,
        other_key.distance_from_c()
    );

    let random_note = Note::get_random();
    println!("Here's a random note: {}\n", random_note);

    let an_interval = Interval {
        base_interval: BaseInterval::Fifth,
        qualifier: IntervalQualifier::Perfect,
    };

    println!("Here's an interval : {}", an_interval);
    println!("Its size is {} semitones", an_interval.size());

    let quizing = false;
    if quizing {
        quiz();
    }
}

fn quiz() {
    let random_note = Note::get_random();
    println!("Whate note is \na perfect fifth above {}?", random_note);
    let user_note = Note::get_from_user();
    match (user_note.distance_from_c() - random_note.distance_from_c()).rem_euclid(N_NOTES) == 7 {
        true => println!("ding ding you win"),
        false => println!("[EXTREMELY LOUD INCORRECT BUZZER]"),
    }
}
