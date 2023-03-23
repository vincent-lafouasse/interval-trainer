mod notes;

fn is_a_fifth_above(start: &notes::Note, end: &notes::Note) -> bool {
    (end.distance + 12 - start.distance) % 12 == 7
}

fn main() {
    let random_note = notes::Note::get_random();
    println!("Whate note is a fifth above {}?", random_note.to_string());
    let user_note = notes::Note::get_from_user();
    match is_a_fifth_above(&random_note, &user_note) {
        true => println!("ding ding you win"),
        false => println!("[EXTREMELY LOUD INCORRECT BUZZER]"),
    }
}
