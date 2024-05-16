use std::fmt;

#[derive(Debug)]
pub struct Note {
    pub name: u8,
    pub alteration: i8,
    pub octave: i8,
}

impl Note {
    pub fn to_midi_style(&self) -> i8 {
        12 * (self.octave + 1) + self.name as i8 + self.alteration as i8
    }

    pub fn frequency(&self) -> f32 {
        let offset_from_a4: i8 = self.to_midi_style() - 69;

        440.0 * 2.0_f32.powf(offset_from_a4 as f32 / 12.0)
    }

    pub fn parse_from_string(string: &str) -> Result<Note, &str> {
        let mut chars = string.chars();

        let name: u8 = match chars.next() {
            None => return Err("no note given"),
            Some(note_name) => match note_name {
                'C' => 0,
                'D' => 1,
                'E' => 2,
                'F' => 3,
                'G' => 4,
                'A' => 5,
                'B' => 6,
                _ => return Err("invalid note name"),
            },
        };

        let mut alteration: i8 = match chars.next() {
            None => return Err("missing octave"),
            Some(c) => match c {
                '0'..='9' => return Ok(Note { name, alteration: 0, octave: c as i8 - '0' as i8 }),
                '#' => 1,
                'b' => -1,
                _ => return Err("invalid alteration"),
            },
        };

        alteration = match chars.next() {
            None => return Err("missing octave"),
            Some(c) => match c {
                '0'..='9' => return Ok(Note { name, alteration, octave: c as i8 - '0' as i8 }),
                '#' => match alteration {
                    1 => 2,
                    _ => return Err("invalid alterations"),
                },
                'b' => match alteration {
                    -1 => -2,
                    _ => return Err("invalid alterations"),
                },
                _ => return Err("invalid alteration"),
            },
        };

        match chars.next() {
            None => Err("missing octave"),
            Some(c) => match c {
                '0'..='9' => Ok(Note { name, alteration, octave: c as i8 - '0' as i8 }),
                _ => Err("invalid note"),
            },
        }
    }
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let notename_repr = match self.name {
            0 => "C",
            1 => "D",
            2 => "E",
            3 => "F",
            4 => "G",
            5 => "A",
            6 => "B",
            _ => "X",
        };
        let alteration_repr = match self.alteration {
            -2 => "bb",
            -1 => "b",
            0 => "",
            1 => "#",
            2 => "##",
            _ => "X",
        };
        write!(f, "{}{}{}", notename_repr, alteration_repr, self.octave)
    }
}
