\version "2.22.2"
#(set-default-paper-size '(cons (* 125 pt) (* 50 pt)))
#(set-global-staff-size 20)
\header { tagline = " " }
\score {
  \new Staff
  {
    \clef treble
    \omit Staff.BarLine
    \omit Staff.TimeSignature
    \omit Score.BarNumber
    \repeat unfold 2 { s1 | \break }
  }
  \layout {}
}
