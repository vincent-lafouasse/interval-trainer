from Alteration import NoAlteration, Flat, DoubleFlat, Sharp, DoubleSharp

TARGET_DIR = "./target"


class LilypondFile:
    def __init__(self, note, clef):
        pass


def octave_ly_repr(octave):
    assert octave in range(9)
    if octave == 0:
		return """,,,"""
    if octave == 1:
		return """,,"""
    if octave == 2:
		return ""","""
    if octave == 3:
		return ""
    if octave == 4:
		return """'"""
    if octave == 5:
		return """''"""
    if octave == 6:
		return """'''"""
    if octave == 7:
		return """''''"""
    if octave == 8:
		return """'''''"""

class Note:
    def __init__(self, name, alteration, octave):
        assert name in "ABCDEFG"
        assert octave in range(9)
        self.name = name
        self.alteration = alteration
        self.octave = octave

    def str_repr(self):
        return f"{name.to_upper()}{self.alteration.str_repr()}{self.octave}"

    def ly_repr(self):
        return f"{name.to_lower()}{self.alteration.ly_repr()}{self.octave}"


def main():
    note = Note("C", NoAlteration(), 3)
    note = Note("B", Flat(), 7)
    note = Note("E", DoubleSharp(), 4)


if __name__ == "__main__":
    main()
