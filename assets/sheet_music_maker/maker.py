TARGET_DIR = "./target"


class LilypondFile:
    def __init__(self, note, clef):
        pass


class Note:
    def __init__(self, name, alteration, octave):
        assert name in "ABCDEFG"
        assert octave in range(9)
        print(f"new note {name}{alteration.str_repr()}{octave}")

class Alteration:
    def __init__(self):
        pass

    @staticmethod
    def lilypond_repr(self):
        pass

    @staticmethod
    def str_repr(self):
        pass

class NoAlteration (Alteration):
    def __init__(self):
        pass

    def lilypond_repr(self):
        return ""

    def str_repr(self):
        return ""

def main():
    note = Note("C", NoAlteration(), 3)


if __name__ == "__main__":
    main()
