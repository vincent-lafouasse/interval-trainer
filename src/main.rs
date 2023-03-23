mod notes;

fn main() {
    let random_note = notes::Note::get_random();
    println!("Whate note is a fifth above {}?", random_note.to_string());
    let user_note = notes::Note::get_from_user();
    let distance = (user_note.distance + 12 - random_note.distance) % 12;
    match distance {
        7 => println!("ding ding you win"),
        _ => println!("[EXTREMELY LOUD INCORRECT BUZZER]"),
    }
}
