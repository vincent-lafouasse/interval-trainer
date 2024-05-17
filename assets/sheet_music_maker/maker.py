from Alteration import NoAlteration, Flat, DoubleFlat, Sharp, DoubleSharp
from Note import Note
from Clef import TrebleClef, BassClef

TARGET_DIR = "./target"


class LilypondFile:
    def __init__(self, note, clef):
        pass


def main():
    note = Note("C", NoAlteration(), 3)
    print(note.str_repr())
    print(note.ly_repr())
    print()
    note = Note("B", Flat(), 7)
    print(note.str_repr())
    print(note.ly_repr())
    print()
    note = Note("E", DoubleSharp(), 4)
    print(note.str_repr())
    print(note.ly_repr())
    print()


if __name__ == "__main__":
    main()
