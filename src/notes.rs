pub struct Note {
    pub name: u8,
    pub alteration: i8,
    pub octave: i8,
}

impl Note {
    pub fn parse_from_string(string: &str) -> Result<Note, &str> {
        let mut chars = string.chars();

        let name: u8 = match chars.next() {
            None => return Err("no note given"),
            Some(note_name) => match note_name {
                'C' => 0,
                'D' => 2,
                'E' => 4,
                'F' => 5,
                'G' => 7,
                'A' => 9,
                'B' => 11,
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
