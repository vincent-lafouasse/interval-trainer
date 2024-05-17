from Alteration import NoAlteration, Flat, DoubleFlat, Sharp, DoubleSharp

TARGET_DIR = "./target"

class LilypondFile:
    def __init__(self, note, clef):
        pass


class Note:
    def __init__(self, name, alteration, octave):
        assert name in "ABCDEFG"
        assert octave in range(9)
        print(f"new note {name}{alteration.str_repr()}{octave}")

def main():
    note = Note("C", NoAlteration(), 3)
    note = Note("B", Flat(), 7)
    note = Note("E", DoubleSharp(), 4)


if __name__ == "__main__":
    main()
