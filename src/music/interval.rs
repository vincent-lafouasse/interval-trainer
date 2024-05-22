use int_enum::IntEnum;
use std::fmt;

use rand::thread_rng;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Interval {
    pub base_interval: BaseInterval,
    pub quality: Quality,
}

#[derive(Copy, Clone, Debug, PartialEq, IntEnum)]
#[repr(u8)]
pub enum BaseInterval {
    Unison = 0,
    Second = 1,
    Third = 2,
    Fourth = 3,
    Fifth = 4,
    Sixth = 5,
    Seventh = 6,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Quality {
    Major,
    Minor,
    Perfect,
    Augmented,
    Diminished,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
}

impl Interval {
    pub fn size_i8(&self) -> i8 {
        self.base_interval.size_i8() + self.quality.delta()
    }

    pub fn size_u8(&self) -> u8 {
        self.size_i8().try_into().unwrap()
    }

    pub fn get_random_diatonic() -> Interval {
        use rand::prelude::*;
        let mut rng = thread_rng();
        let diatonic_intervals = [
            //Interval { base_interval: BaseInterval::Unison, quality: Quality::Perfect },
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

impl Direction {
    pub fn rand() -> Self {
        use rand::Rng;
        let mut rng = thread_rng();
        let rn: u8 = rng.gen::<u8>() % 2;

        match rn {
            0 => Direction::Up,
            1 => Direction::Down,
            _ => panic!("unreachable"),
        }
    }
}

impl BaseInterval {
    pub fn size_i8(&self) -> i8 {
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

    pub fn size_u8(&self) -> u8 {
        self.size_i8().try_into().unwrap()
    }
}

impl Quality {
    pub fn delta(&self) -> i8 {
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
