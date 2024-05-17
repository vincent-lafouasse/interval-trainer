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
