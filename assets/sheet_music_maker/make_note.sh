#!/usr/bin/env sh

generate_lilypond_file () {
	cat > "$1.ly" <<- EOM
\version "2.22.2"

#(set-default-paper-size "a9landscape")

{
	\time 4/4
	\clef treble
	\key c \major
	| c'2 |
	}
	EOM
}

#################################################################################

MAIN=main

generate_lilypond_file "${MAIN}"
lilypond "${MAIN}.ly"
open "${MAIN}.pdf"
