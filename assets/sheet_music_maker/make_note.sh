#!/usr/bin/env sh

MAIN=main

cat > "${MAIN}.ly" <<- EOM
\version "2.22.2"

\relative {
\time 4/4
\clef treble
\key c \major
| c'1 |
}
EOM

lilypond "${MAIN}.ly"
open "${MAIN}.pdf"
