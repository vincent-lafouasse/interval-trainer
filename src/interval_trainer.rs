use std::{sync::mpsc::Sender, time::Duration};

use crate::music::{Direction, Interval, Note, NoteRange};

use crate::audio;

pub struct IntervalTrainer {
    pub scene: Scene,
    range: NoteRange,
    sample_rate: u16,
}

impl IntervalTrainer {
    pub fn init(range: NoteRange) -> Self {
        const SAMPLE_RATE: u16 = 44_100;
        Self { scene: Scene::Idle, range, sample_rate: SAMPLE_RATE }
    }

    pub fn start_playback(&self, playback_tx: Sender<()>) -> (Note, Note) {
        let (reference, mystery_note) = self.choose_notes();
        let note_length = Duration::from_millis(1000);
        audio::synth::play_notes_in_thread(
            reference,
            mystery_note,
            note_length,
            self.sample_rate,
            playback_tx.clone(),
        );

        (reference, mystery_note)
    }

    pub fn listen_for(&self, mystery_note: Note, pitch_detection_tx: Sender<bool>) {
        let detection_duration = Duration::from_millis(1500);
        audio::listen::listen_for_note_in_thread(
            mystery_note.to_simple(),
            detection_duration,
            self.sample_rate,
            pitch_detection_tx.clone(),
        );
    }

    pub fn ding(&self) {
        audio::play_sample::play_ding_in_thread();
    }

    pub fn bad_ding(&self) {
        audio::play_sample::play_wrong_sound_effect_in_thread();
    }

    fn choose_notes(&self) -> (Note, Note) {
        let interval = Interval::get_random_diatonic();
        let direction = Direction::Up;

        let new_range = match direction {
            Direction::Up => self.range.crop_top(interval.size_i8()),
            Direction::Down => self.range.crop_bottom(interval.size_i8()),
        };

        let reference = new_range.rand();
        (reference, reference.up(interval))
    }
}

#[derive(Default, Copy, Clone, Debug)]
pub enum Scene {
    #[default]
    Idle,
    PlayingSound(Note, Note),
    Listening(Note, Note),
    Concluding(Note, Note),
}
