pub trait LilypondThing {
    fn lily_repr(&self) -> String;
}

#[derive(Copy, Clone)]
pub struct Note {
    pub name: NoteName,
    pub alteration: Alteration,
    pub octave: Octave,
}

impl Note {
    pub fn new(name: NoteName, alteration: Alteration, octave: Octave) -> Self {
        Note { name, alteration, octave }
    }
}

impl LilypondThing for Note {
    fn lily_repr(&self) -> String {
        self.name.lily_repr() + &self.alteration.lily_repr() + &self.octave.lily_repr()
    }
}

type NoteName = char;
impl LilypondThing for NoteName {
    fn lily_repr(&self) -> String {
        match *self {
            'C' => "c".to_string(),
            'D' => "d".to_string(),
            'E' => "e".to_string(),
            'F' => "f".to_string(),
            'G' => "g".to_string(),
            'A' => "a".to_string(),
            'B' => "b".to_string(),
            _ => panic!("invalid notename: {}", *self),
        }
    }
}

#[derive(Copy, Clone)]
pub enum Clef {
    TrebleClef,
    BassClef,
    TrebleClefSubOctave,
}

impl LilypondThing for Clef {
    fn lily_repr(&self) -> String {
        match *self {
            Clef::TrebleClef => "treble".to_string(),
            Clef::BassClef => "bass".to_string(),
            Clef::TrebleClefSubOctave => "treble_8".to_string(),
        }
    }
}

pub type Octave = i8;
impl LilypondThing for Octave {
    fn lily_repr(&self) -> String {
        match *self {
            0 => ",,,".to_string(),
            1 => ",,".to_string(),
            2 => ",".to_string(),
            3 => "".to_string(),
            4 => "'".to_string(),
            5 => "''".to_string(),
            6 => "'''".to_string(),
            7 => "''''".to_string(),
            8 => "'''''".to_string(),
            _ => panic!("invalid octave: {}", *self),
        }
    }
}

#[derive(Copy, Clone)]
pub enum Alteration {
    NoAlteration,
    Sharp,
    DoubleSharp,
    Flat,
    DoubleFlat,
}
impl LilypondThing for Alteration {
    fn lily_repr(&self) -> String {
        match *self {
            Alteration::NoAlteration => "".to_string(),
            Alteration::Flat => "es".to_string(),
            Alteration::DoubleFlat => "eses".to_string(),
            Alteration::Sharp => "is".to_string(),
            Alteration::DoubleSharp => "isis".to_string(),
        }
    }
}
