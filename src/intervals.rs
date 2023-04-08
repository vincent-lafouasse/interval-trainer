use std::fmt;

use crate::notes::{Alteration, Note, NoteName};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BaseInterval {
    Unison,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
}

impl BaseInterval {
    pub fn size(&self) -> isize {
        match &self {
            BaseInterval::Unison => 0,
            BaseInterval::Second => 2,
            BaseInterval::Third => 4,
            BaseInterval::Fourth => 5,
            BaseInterval::Fifth => 7,
            BaseInterval::Sixth => 9,
            BaseInterval::Seventh => 11,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Quality {
    Major,
    Minor,
    Perfect,
    Augmented,
    Diminished,
    DoublyAugmented,
    DoublyDiminished,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Interval {
    pub base_interval: BaseInterval,
    pub quality: Quality,
}

impl Interval {
    pub fn note_up_from(&self, start: Note) -> Note {
        let note_name_distance = match self.base_interval {
            BaseInterval::Unison => 0,
            BaseInterval::Second => 1,
            BaseInterval::Third => 2,
            BaseInterval::Fourth => 3,
            BaseInterval::Fifth => 4,
            BaseInterval::Sixth => 5,
            BaseInterval::Seventh => 6,
        };

        let note_name: NoteName = NoteName::shift(start.name, note_name_distance);
        let natural_note: Note = Note { name: note_name, alteration: Alteration::Natural };
        let rough_distance =
            (natural_note.distance_from_c() - start.distance_from_c()).rem_euclid(12);

        let alteration: Alteration = match self.size() - rough_distance {
            0 => Alteration::Natural,
            1 => Alteration::Sharp,
            2 => Alteration::DoubleSharp,
            -1 => Alteration::Flat,
            -2 => Alteration::DoubleFlat,
            _ => panic!("Can only handle 2 alteration"),
        };
        Note { name: note_name, alteration: alteration }
    }

    pub fn between(start: Note, end: Note) -> Interval {
        let mut interval_index = 0;
        while NoteName::shift(start.name, interval_index) != end.name {
            interval_index += 1;
        }
        let base_interval = match interval_index {
            0 => BaseInterval::Unison,
            1 => BaseInterval::Second,
            2 => BaseInterval::Third,
            3 => BaseInterval::Fourth,
            4 => BaseInterval::Fifth,
            5 => BaseInterval::Sixth,
            6 => BaseInterval::Seventh,
            _ => panic!(""),
        };

        let interval_size = (end.distance_from_c() - start.distance_from_c()).rem_euclid(12);
        let base_size = base_interval.size();
        let distance_from_diatonic = (interval_size - base_size).rem_euclid(12);

        let quality = {
            if base_interval == BaseInterval::Second
                || base_interval == BaseInterval::Third
                || base_interval == BaseInterval::Sixth
                || base_interval == BaseInterval::Seventh
            {
                match distance_from_diatonic {
                    2 => Quality::DoublyAugmented,
                    1 => Quality::Augmented,
                    0 => Quality::Major,
                    11 => Quality::Minor,
                    10 => Quality::Diminished,
                    19 => Quality::DoublyDiminished,
                    _ => panic!("Interval too diminished or augmented to handle for now"),
                }
            } else {
                match distance_from_diatonic {
                    2 => Quality::DoublyAugmented,
                    1 => Quality::Augmented,
                    0 => Quality::Perfect,
                    11 => Quality::Diminished,
                    10 => Quality::DoublyDiminished,
                    _ => panic!("Interval too diminished or augmented to handle for now"),
                }
            }
        };

        Interval { base_interval: base_interval, quality: quality }
    }

    pub fn size(&self) -> isize {
        let distance = self.base_interval.size();
        let increment = {
            if self.base_interval == BaseInterval::Second
                || self.base_interval == BaseInterval::Third
                || self.base_interval == BaseInterval::Sixth
                || self.base_interval == BaseInterval::Seventh
            {
                match &self.quality {
                    Quality::Major => 0,
                    Quality::Minor => -1,
                    Quality::Perfect => panic!("Interval {} can't be perfect", self.base_interval),
                    Quality::Diminished => -2,
                    Quality::Augmented => 1,
                    Quality::DoublyDiminished => -3,
                    Quality::DoublyAugmented => 2,
                }
            } else {
                match &self.quality {
                    Quality::Major => panic!("Interval {} can't be major", self.base_interval),
                    Quality::Minor => panic!("Interval {} can't be minor", self.base_interval),
                    Quality::Perfect => 0,
                    Quality::Diminished => -1,
                    Quality::Augmented => 1,
                    Quality::DoublyDiminished => -2,
                    Quality::DoublyAugmented => 2,
                }
            }
        };
        distance + increment
    }

    pub fn get_random_diatonic() -> Interval {
        use rand::prelude::*;
        let mut rng = thread_rng();
        let diatonic_intervals = [
            Interval { base_interval: BaseInterval::Unison, quality: Quality::Perfect },
            Interval { base_interval: BaseInterval::Second, quality: Quality::Major },
            Interval { base_interval: BaseInterval::Second, quality: Quality::Minor },
            Interval { base_interval: BaseInterval::Third, quality: Quality::Major },
            Interval { base_interval: BaseInterval::Third, quality: Quality::Minor },
            Interval { base_interval: BaseInterval::Fourth, quality: Quality::Perfect },
            Interval { base_interval: BaseInterval::Fourth, quality: Quality::Augmented },
            Interval { base_interval: BaseInterval::Fifth, quality: Quality::Perfect },
            Interval { base_interval: BaseInterval::Fifth, quality: Quality::Diminished },
            Interval { base_interval: BaseInterval::Sixth, quality: Quality::Major },
            Interval { base_interval: BaseInterval::Sixth, quality: Quality::Minor },
            Interval { base_interval: BaseInterval::Seventh, quality: Quality::Major },
            Interval { base_interval: BaseInterval::Seventh, quality: Quality::Minor },
        ];

        *diatonic_intervals.choose(&mut rng).unwrap()
    }
}

impl fmt::Display for Interval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.quality, self.base_interval)
    }
}
impl fmt::Display for BaseInterval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let repr = match &self {
            BaseInterval::Unison => "Unison",
            BaseInterval::Second => "Second",
            BaseInterval::Third => "Third",
            BaseInterval::Fourth => "Fourth",
            BaseInterval::Fifth => "Fifth",
            BaseInterval::Sixth => "Sixth",
            BaseInterval::Seventh => "Seventh",
        };
        write!(f, "{}", repr)
    }
}

impl fmt::Display for Quality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let repr = match &self {
            Quality::Major => "Major",
            Quality::Minor => "Minor",
            Quality::Perfect => "Perfect",
            Quality::Diminished => "Diminished",
            Quality::Augmented => "Augmented",
            Quality::DoublyDiminished => "Doubly Diminished",
            Quality::DoublyAugmented => "Doubly Augmented",
        };
        write!(f, "{}", repr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Intervals
    const PERFECT_UNISON: Interval =
        Interval { base_interval: BaseInterval::Unison, quality: Quality::Perfect };
    const MINOR_SECOND: Interval =
        Interval { base_interval: BaseInterval::Second, quality: Quality::Minor };
    const MAJOR_SECOND: Interval =
        Interval { base_interval: BaseInterval::Second, quality: Quality::Major };
    const MINOR_THIRD: Interval =
        Interval { base_interval: BaseInterval::Third, quality: Quality::Minor };
    const MAJOR_THIRD: Interval =
        Interval { base_interval: BaseInterval::Third, quality: Quality::Major };
    const PERFECT_FOURTH: Interval =
        Interval { base_interval: BaseInterval::Fourth, quality: Quality::Perfect };
    const AUGMENTED_FOURTH: Interval =
        Interval { base_interval: BaseInterval::Fourth, quality: Quality::Augmented };
    const DIMINISHED_FIFTH: Interval =
        Interval { base_interval: BaseInterval::Fifth, quality: Quality::Diminished };
    const PERFECT_FIFTH: Interval =
        Interval { base_interval: BaseInterval::Fifth, quality: Quality::Perfect };
    const MINOR_SIXTH: Interval =
        Interval { base_interval: BaseInterval::Sixth, quality: Quality::Minor };
    const MAJOR_SIXTH: Interval =
        Interval { base_interval: BaseInterval::Sixth, quality: Quality::Major };
    const MINOR_SEVENTH: Interval =
        Interval { base_interval: BaseInterval::Seventh, quality: Quality::Minor };
    const MAJOR_SEVENTH: Interval =
        Interval { base_interval: BaseInterval::Seventh, quality: Quality::Major };

    // Notes
    const C: Note = Note { name: NoteName::C, alteration: Alteration::Natural };
    const E_SHARP: Note = Note { name: NoteName::E, alteration: Alteration::Sharp };
    const A_FLAT: Note = Note { name: NoteName::A, alteration: Alteration::Flat };
    const D_FLAT: Note = Note { name: NoteName::D, alteration: Alteration::Flat };
    const F_SHARP: Note = Note { name: NoteName::F, alteration: Alteration::Sharp };
    const C_SHARP: Note = Note { name: NoteName::C, alteration: Alteration::Sharp };
    const F_FLAT: Note = Note { name: NoteName::F, alteration: Alteration::Flat };
    const B_SHARP: Note = Note { name: NoteName::B, alteration: Alteration::Sharp };
    #[test]
    fn test_interval_size() {
        assert_eq!(0, PERFECT_UNISON.size());
        assert_eq!(1, MINOR_SECOND.size());
        assert_eq!(2, MAJOR_SECOND.size());
        assert_eq!(3, MINOR_THIRD.size());
        assert_eq!(4, MAJOR_THIRD.size());
        assert_eq!(5, PERFECT_FOURTH.size());
        assert_eq!(6, AUGMENTED_FOURTH.size());
        assert_eq!(6, DIMINISHED_FIFTH.size());
        assert_eq!(7, PERFECT_FIFTH.size());
        assert_eq!(8, MINOR_SIXTH.size());
        assert_eq!(9, MAJOR_SIXTH.size());
        assert_eq!(10, MINOR_SEVENTH.size());
        assert_eq!(11, MAJOR_SEVENTH.size());
    }

    #[test]
    fn test_interval_between_common_intervals() {
        assert_eq!(Interval::between(C, D_FLAT), MINOR_SECOND);
        assert_eq!(Interval::between(C, A_FLAT), MINOR_SIXTH);
        assert_eq!(Interval::between(C, F_SHARP), AUGMENTED_FOURTH);
        assert_eq!(Interval::between(D_FLAT, C), MAJOR_SEVENTH);
        assert_eq!(Interval::between(A_FLAT, C), MAJOR_THIRD);
        assert_eq!(Interval::between(F_SHARP, C), DIMINISHED_FIFTH);
        assert_eq!(Interval::between(A_FLAT, D_FLAT), PERFECT_FOURTH);
        assert_eq!(Interval::between(D_FLAT, A_FLAT), PERFECT_FIFTH);
    }

    #[test]
    fn test_interval_between_intense_intervals() {
        assert_eq!(
            Interval::between(C, B_SHARP),
            Interval { base_interval: BaseInterval::Seventh, quality: Quality::Augmented }
        );
        assert_eq!(
            Interval::between(B_SHARP, C),
            Interval { base_interval: BaseInterval::Second, quality: Quality::Diminished }
        );

        assert_eq!(
            Interval::between(C_SHARP, C),
            Interval { base_interval: BaseInterval::Unison, quality: Quality::Diminished }
        );
        assert_eq!(
            Interval::between(C, C_SHARP),
            Interval { base_interval: BaseInterval::Unison, quality: Quality::Augmented }
        );

        // B# -> Fb is a triply diminished fifth (eq to major third), implem of bbb intervals todo
        // Fb -> B# is triply augmented fourth (eq to minor sixth)
    }

    #[test]
    fn test_note_up_from() {
        assert_eq!(
            MAJOR_SECOND.note_up_from(D_FLAT),
            Note { name: NoteName::E, alteration: Alteration::Flat }
        );
        assert_eq!(
            MAJOR_SIXTH.note_up_from(E_SHARP),
            Note { name: NoteName::C, alteration: Alteration::DoubleSharp }
        );
        assert_eq!(
            MAJOR_SEVENTH.note_up_from(E_SHARP),
            Note { name: NoteName::D, alteration: Alteration::DoubleSharp }
        );
    }
}
