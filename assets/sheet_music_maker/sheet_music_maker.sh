#!/usr/bin/env sh

set -o xtrace

#################################################################################

MAIN="main"
AUX_DIR="aux"
TARGET_DIR="target"
mkdir "${AUX_DIR}"
mkdir "${TARGET_DIR}"

TEMPLATE="template/template.ly"
LILY_FILE="${AUX_DIR}/${MAIN}.ly"
OUTPUT="${TARGET_DIR}/${MAIN}.pdf"

TIME_SIG="2/4"
CLEF="treble"
NOTES="c'2"
export TIME_SIG CLEF NOTES

envsubst < "${TEMPLATE}" > "${LILY_FILE}"
lilypond --output="${TARGET_DIR}" "${LILY_FILE}"
open "${OUTPUT}"

rm -rf "${AUX_DIR}"
