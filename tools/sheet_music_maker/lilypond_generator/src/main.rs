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
