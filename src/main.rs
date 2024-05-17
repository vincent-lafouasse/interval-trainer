#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

mod interval;
mod listen;
mod note_range;
mod notes;
mod simple_note;
mod synth;
mod wavetables;

use color_eyre::eyre::Result;
use std::time::Duration;

use crate::interval::{Direction, Interval};
use crate::listen::listen_for_note;
use crate::note_range::NoteRange;
use crate::notes::Note;
use crate::simple_note::SimpleNote;
use crate::synth::play_notes;

const SAMPLE_RATE: u16 = 44_100;

fn main() -> Result<()> {
    color_eyre::install()?;

    let range = NoteRange::tenor_voice();
    let (reference_note, mystery_note) = choose_notes(&range);

    println!("This is {}", reference_note);
    play_notes(
        reference_note,
        mystery_note,
        Duration::from_millis(1000),
        SAMPLE_RATE,
    );

    match listen_for_note(
        mystery_note.to_simple(),
        Duration::from_millis(1500),
        SAMPLE_RATE,
    ) {
        Some(cent_deviation) => println!(
            "you got it ! it was {}\nyou got it within a {} cent deviation",
            mystery_note, cent_deviation
        ),
        None => println!("womp womp it was {}", mystery_note),
    }

    Ok(())
}

fn choose_notes(range: &NoteRange) -> (Note, Note) {
    let interval = Interval::get_random_diatonic();
    let direction = Direction::Up;

    let new_range = match direction {
        Direction::Up => range.crop_top(interval.size_i8()),
        Direction::Down => range.crop_bottom(interval.size_i8()),
    };

    let reference = new_range.rand();
    (reference, reference.up(interval))
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
