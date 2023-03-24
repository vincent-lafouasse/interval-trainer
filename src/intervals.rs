use std::fmt;

#[allow(dead_code)]
pub enum BaseInterval {
    UNISON,
    SECOND,
    THIRD,
    FOURTH,
    FIFTH,
    SIXTH,
    SEVENTH,
}

#[allow(dead_code)]
pub enum IntervalQualifier {
    MAJOR,
    MINOR,
    PERFECT,
    AUGMENTED,
    DIMINISHED,
    DOUBLY_AUGMENTED,
    DOUBLY_DIMINISHED,
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
            BaseInterval::UNISON => "Unison",
            BaseInterval::SECOND => "Second",
            BaseInterval::THIRD => "Third",
            BaseInterval::FOURTH => "Fourth",
            BaseInterval::FIFTH => "Fifth",
            BaseInterval::SIXTH => "Sixth",
            BaseInterval::SEVENTH => "Seventh",
        };
        write!(f, "{}", repr)
    }
}

impl fmt::Display for IntervalQualifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let repr = match &self {
            IntervalQualifier::MAJOR => "Major",
            IntervalQualifier::MINOR => "Minor",
            IntervalQualifier::PERFECT => "Perfect",
            IntervalQualifier::DIMINISHED => "Diminished",
            IntervalQualifier::AUGMENTED => "Augmented",
            IntervalQualifier::DOUBLY_DIMINISHED => "Doubly Diminished",
            IntervalQualifier::DOUBLY_AUGMENTED => "Doubly Augmented",
        };
        write!(f, "{}", repr)
    }
}
