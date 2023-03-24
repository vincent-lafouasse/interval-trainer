use std::fmt;

#[allow(dead_code)]
#[derive(PartialEq)]
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
        let distance = self.base_interval.size();
        let increment = {
            if self.base_interval == BaseInterval::Second
                || self.base_interval == BaseInterval::Third
                || self.base_interval == BaseInterval::Sixth
            {
                match &self.qualifier {
                    IntervalQualifier::Major => 0,
                    IntervalQualifier::Minor => -1,
                    IntervalQualifier::Perfect => panic!("Interval can't be perfect"),
                    IntervalQualifier::Diminished => -2,
                    IntervalQualifier::Augmented => 1,
                    IntervalQualifier::DoublyDiminished => -3,
                    IntervalQualifier::DoublyAugmented => 2,
                }
            } else {
                match &self.qualifier {
                    IntervalQualifier::Major => panic!("Interval can't be major"),
                    IntervalQualifier::Minor => panic!("Interval can't be minor"),
                    IntervalQualifier::Perfect => 0,
                    IntervalQualifier::Diminished => -1,
                    IntervalQualifier::Augmented => 1,
                    IntervalQualifier::DoublyDiminished => -2,
                    IntervalQualifier::DoublyAugmented => 2,
                }
            }
        };
        distance + increment
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
