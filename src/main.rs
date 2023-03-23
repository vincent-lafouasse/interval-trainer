mod notes;

fn main() {
    let quizing = true;

    let fave_key = notes::AltNote {
        name: notes::NoteName::D,
        alteration: notes::Alteration::FLAT,
    };
    println!("My favorite key is {}\n", fave_key.repr());

    if quizing {
        let random_note = notes::Note::get_random();
        println!("Whate note is a fifth above {}?", random_note.to_string());
        let user_note = notes::Note::get_from_user();
        match user_note.is_a_fifth_above(&random_note) {
            true => println!("ding ding you win"),
            false => println!("[EXTREMELY LOUD INCORRECT BUZZER]"),
        }
    }
}
