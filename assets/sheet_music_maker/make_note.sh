#!/usr/bin/env sh

MAIN=C4

lilypond "${MAIN}.ly"
open "${MAIN}.pdf"
