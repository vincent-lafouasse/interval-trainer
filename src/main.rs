mod notes;

fn main() {
    let fave_key = notes::Note {
        name: notes::NoteName::D,
        alteration: notes::Alteration::FLAT,
    };
    println!("My favorite key is {}", fave_key.repr());
    println!("It's {} semitones from C\n", fave_key.distance_from_c());

    let other_key = notes::Note::parse_from_string("Eb").expect("Huh oh that's not a good note");
    println!(
        "Another good one is {}, which is {} semitones from C\n",
        other_key.repr(),
        other_key.distance_from_c()
    );

    let random_note = notes::Note::get_random();
    println!("Here's a random note: {}\n", random_note.repr());

    let quizing = true;
    if quizing {
        quiz();
    }
}

fn quiz() {
    let random_note = notes::Note::get_random();
    println!(
        "Whate note is \na perfect fifth above {}?",
        random_note.repr()
    );
    let user_note = notes::Note::get_from_user();
    match (user_note.distance_from_c() + 12 - random_note.distance_from_c()) % 12 == 7 {
        true => println!("ding ding you win"),
        false => println!("[EXTREMELY LOUD INCORRECT BUZZER]"),
    }
}
