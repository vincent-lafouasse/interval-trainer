use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Interval {
    pub base_interval: BaseInterval,
    pub quality: Quality,
}

impl Interval {
    pub fn size(&self) -> isize {
        self.base_interval.size() + self.quality.delta()
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

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
}

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
}

impl Quality {
    pub fn delta(&self) -> isize {
        match &self {
            Quality::Major => 0,
            Quality::Minor => -1,
            Quality::Perfect => 0,
            Quality::Diminished => -1,
            Quality::Augmented => 1,
        }
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
        };
        write!(f, "{}", repr)
    }
}
