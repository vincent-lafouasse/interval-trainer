\version "2.22.2"
#(set-default-paper-size '(cons (* 125 pt) (* 50 pt)))
\header { tagline = " " }
\new Staff \with {
	\override TimeSignature.stencil = ##f
}{
	\time 100/2 % no bar lines (probably)
	\clef treble
	\key c \major
	| a'!1 a'!1 |
}
\version "2.22.2"
#(set-default-paper-size '(cons (* 125 pt) (* 50 pt)))
\header { tagline = " " }
\new Staff \with {
	\override TimeSignature.stencil = ##f
}{
	\time 100/2 % no bar lines (probably)
	\clef treble
	\key c \major
	| a'!1 a'!1 |
}
