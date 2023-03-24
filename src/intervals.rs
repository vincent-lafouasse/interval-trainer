use std::fmt;

#[allow(dead_code)]
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
pub enum IntervalQualifier {
    Major,
    Minor,
    Perfect,
    Augmented,
    Diminished,
    DoublyAugmented,
    DoublyDiminished,
}

pub struct Interval {
    pub base_interval: BaseInterval,
    pub qualifier: IntervalQualifier,
}

impl Interval {
    pub fn size(&self) -> i8 {
        let mut distance = self.base_interval.size();
        distance += 1;
        distance -= 1;
        distance
    }
}

impl fmt::Display for Interval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.qualifier, self.base_interval)
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

impl fmt::Display for IntervalQualifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let repr = match &self {
            IntervalQualifier::Major => "Major",
            IntervalQualifier::Minor => "Minor",
            IntervalQualifier::Perfect => "Perfect",
            IntervalQualifier::Diminished => "Diminished",
            IntervalQualifier::Augmented => "Augmented",
            IntervalQualifier::DoublyDiminished => "Doubly Diminished",
            IntervalQualifier::DoublyAugmented => "Doubly Augmented",
        };
        write!(f, "{}", repr)
    }
}
