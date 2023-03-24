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
