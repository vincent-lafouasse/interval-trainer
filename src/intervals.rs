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
impl BaseInterval {
    pub fn repr(&self) -> &str {
        match &self {
            BaseInterval::UNISON => "Unison",
            BaseInterval::SECOND => "Second",
            BaseInterval::THIRD => "Third",
            BaseInterval::FOURTH => "Fourth",
            BaseInterval::FIFTH => "Fifth",
            BaseInterval::SIXTH => "Sixth",
            BaseInterval::SEVENTH => "Seventh",
        }
    }
}

#[allow(dead_code)]
pub enum IntervalQualifier {
    MAJOR,
    MINOR,
    PERFECT,
    AUGMENTED,
    DIMINISHED,
}

pub struct Interval {
    pub base_interval: BaseInterval,
    pub qualifier: IntervalQualifier,
}
