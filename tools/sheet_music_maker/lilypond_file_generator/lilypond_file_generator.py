from Alteration import NoAlteration, Flat, DoubleFlat, Sharp, DoubleSharp
from Note import Note
from Clef import TrebleClef, BassClef

TARGET_DIR = "./target/"


class LilypondFile:
    def __init__(self, note, clef):
        self.note = note
        self.clef = clef
        self.filename = note.str_repr() + "_" + clef.get()

    def write(self):
        full_filename = TARGET_DIR + self.filename + ".ly"
        with open(full_filename, "w") as output:
            output.write('\\version "2.22.2" \n')
            output.write("#(set-default-paper-size '(cons (* 80 pt) (* 30 pt)))\n")
            output.write("\\header { tagline = \" \" }")
            output.write("\\new Staff \\with { \n")
            output.write("	\\override TimeSignature.stencil = ##f \n")
            output.write("}{ \n")
            output.write("	\\time 100/2 % no bar lines (probably) \n")
            output.write(f"	\\clef {self.clef.get()} \n")
            output.write("	\\key c \\major \n")
            output.write(f"	| {self.note.ly_repr()}1 {self.note.ly_repr()}1 | \n")
            output.write("} \n")


def main():
    note = Note("E", Flat(), 4)

    ly_file = LilypondFile(note, TrebleClef())
    ly_file.write()


if __name__ == "__main__":
    main()
