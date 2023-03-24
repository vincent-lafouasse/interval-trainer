mod notes;

fn main() {
    let fave_key = notes::AltNote {
        name: notes::NoteName::D,
        alteration: notes::Alteration::FLAT,
    };
    println!("My favorite key is {}", fave_key.repr());
    println!("It's {} semitones from C\n", fave_key.distance_from_c());

    let other_key = notes::AltNote::parse_from_string("B#");
    println!(
        "Another good one is {}, which is {} semitones from C\n",
        other_key.repr(),
        other_key.distance_from_c()
    );

    let quizing = false;
    if quizing {
        quiz();
    }
}

fn quiz() {
    let random_note = notes::Note::get_random();
    println!(
        "Whate note is \na perfect fifth above {}?",
        random_note.to_string()
    );
    let user_note = notes::Note::get_from_user();
    match user_note.is_a_fifth_above(&random_note) {
        true => println!("ding ding you win"),
        false => println!("[EXTREMELY LOUD INCORRECT BUZZER]"),
    }
}
