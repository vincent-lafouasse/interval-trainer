fn main() {
    println!("Hello, world!");
}

trait LilypondThing {
    fn lily_repr(&self) -> &str;
}

type NoteName = char;
impl LilypondThing for NoteName {
    fn lily_repr(&self) -> &str {
        match *self {
            'C' => "c",
            'D' => "d",
            'E' => "e",
            'F' => "f",
            'G' => "g",
            'A' => "a",
            'B' => "b",
            _ => panic!("invalid notename: {}", *self),
        }
    }
}

enum Clef {
    TrebleClef,
    BassClef,
}

impl LilypondThing for Clef {
    fn lily_repr(&self) -> &str {
        match *self {
            Clef::TrebleClef => "treble",
            Clef::BassClef => "bass",
        }
    }
}

type Octave = i8;
impl LilypondThing for Octave {
    fn lily_repr(&self) -> &str {
        match *self {
            0 => ",,,",
            1 => ",,",
            2 => ",",
            3 => "",
            4 => "'",
            5 => "''",
            6 => "'''",
            7 => "''''",
            8 => "'''''",
            _ => panic!("invalid octave: {}", *self),
        }
    }
}

enum Alteration {
    NoAlteration,
    Sharp,
    DoubleSharp,
    Flat,
    DoubleFlat,
}
impl LilypondThing for Alteration {
    fn lily_repr(&self) -> &str {
        match *self {
            Alteration::NoAlteration => "",
            Alteration::Flat => "es",
            Alteration::DoubleFlat => "eses",
            Alteration::Sharp => "is",
            Alteration::DoubleSharp => "isis",
        }
    }
}
