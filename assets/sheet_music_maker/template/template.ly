\version "2.22.2"

#(set-default-paper-size "a9landscape")

\new Staff
\with
{
    \override TimeSignature.stencil = ##f % no time signature
}
{
\time 100/2 % no bar lines (probably)
\clef ${CLEF}
\key c \major
| ${NOTES} |
}
