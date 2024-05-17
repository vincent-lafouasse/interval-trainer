class Alteration:
    def __init__(self):
        pass

    def lilypond_repr(self):
        pass

    def str_repr(self):
        pass

class NoAlteration (Alteration):
    def __init__(self):
        pass

    def lilypond_repr(self):
        return ""

    def str_repr(self):
        return ""

class Sharp (Alteration):
    def __init__(self):
        pass

    def lilypond_repr(self):
        return "is"

    def str_repr(self):
        return "#"

class DoubleSharp (Alteration):
    def __init__(self):
        pass

    def lilypond_repr(self):
        return "isis"

    def str_repr(self):
        return "##"

class Flat (Alteration):
    def __init__(self):
        pass

    def lilypond_repr(self):
        return "es"

    def str_repr(self):
        return "b"

class DoubleFlat (Alteration):
    def __init__(self):
        pass

    def lilypond_repr(self):
        return "eses"

    def str_repr(self):
        return "bb"
