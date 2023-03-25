use rand::Rng;
use std::fmt;

#[allow(dead_code)]
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
    pub fn size(&self) -> i8 {
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

#[allow(dead_code)]
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
    pub fn size(&self) -> i8 {
        let distance = self.base_interval.size();
        let increment = {
            if self.base_interval == BaseInterval::Second
                || self.base_interval == BaseInterval::Third
                || self.base_interval == BaseInterval::Sixth
            {
                match &self.quality {
                    Quality::Major => 0,
                    Quality::Minor => -1,
                    Quality::Perfect => panic!("Interval can't be perfect"),
                    Quality::Diminished => -2,
                    Quality::Augmented => 1,
                    Quality::DoublyDiminished => -3,
                    Quality::DoublyAugmented => 2,
                }
            } else {
                match &self.quality {
                    Quality::Major => panic!("Interval can't be major"),
                    Quality::Minor => panic!("Interval can't be minor"),
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
        let rn_base_interval = rand::thread_rng().gen_range(1, 8);
        let base_interval = match rn_base_interval {
            1 => BaseInterval::Unison,
            2 => BaseInterval::Second,
            3 => BaseInterval::Third,
            4 => BaseInterval::Fourth,
            5 => BaseInterval::Fifth,
            6 => BaseInterval::Sixth,
            7 => BaseInterval::Seventh,
            _ => panic!(""),
        };

        let rn_alteration = rand::thread_rng().gen_range(0, 2);
        let quality = match base_interval {
            BaseInterval::Unison => Quality::Perfect,
            BaseInterval::Second => match rn_alteration {
                0 => Quality::Major,
                1 => Quality::Minor,
                _ => panic!(""),
            },
            BaseInterval::Third => match rn_alteration {
                0 => Quality::Major,
                1 => Quality::Minor,
                _ => panic!(""),
            },
            BaseInterval::Fourth => match rn_alteration {
                0 => Quality::Perfect,
                1 => Quality::Augmented,
                _ => panic!(""),
            },
            BaseInterval::Fifth => Quality::Perfect,
            BaseInterval::Sixth => match rn_alteration {
                0 => Quality::Major,
                1 => Quality::Minor,
                _ => panic!(""),
            },
            BaseInterval::Seventh => match rn_alteration {
                0 => Quality::Major,
                1 => Quality::Minor,
                _ => panic!(""),
            },
        };

        Interval {
            base_interval: base_interval,
            quality: quality,
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
            Quality::DoublyDiminished => "Doubly Diminished",
            Quality::DoublyAugmented => "Doubly Augmented",
        };
        write!(f, "{}", repr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PERFECT_UNISON: Interval = Interval {
        base_interval: BaseInterval::Unison,
        quality: Quality::Perfect,
    };

    #[test]
    fn test_interval_size() {
        assert_eq!(PERFECT_UNISON.size(), 0);
    }
}
