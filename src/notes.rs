pub struct Note {
    pub name: u8,
    pub alteration: i8,
    pub octave: u8,
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

        let mut note = Note { name, alteration: 0, octave: 0 };

        match chars.next() {
            None => return Err("missing octave"),
            Some(c) => match c {
                '0'..='9' => return Ok(Note { name, alteration: 0, octave: c as u8 - '0' as u8 }),
                '#' => note.alteration = 1,
                'b' => note.alteration = -1,
                _ => return Err("error"),
            },
        }

        Ok(note)
    }
}
