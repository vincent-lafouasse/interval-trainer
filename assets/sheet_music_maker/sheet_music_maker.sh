#!/usr/bin/env sh

#################################################################################

MAIN="main"
AUX_DIR="./aux"
TARGET_DIR="./target"
mkdir "${AUX_DIR}"
mkdir "${TARGET_DIR}"

TEMPLATE="template/template.ly"
LILY_FILE="${AUX_DIR}/${MAIN}.ly"
OUTPUT="${TARGET_DIR}/${MAIN}.ly"

TIME_SIG="2/4"
CLEF="treble"
NOTES="c'2"
export TIME_SIG CLEF NOTES

cat "${TEMPLATE}" | envsubst > "${LILY_FILE}"
lilypond "${LILY_FILE}" --output="${TARGET_DIR}"
open "${OUTPUT}"

rm -rf "${AUX_DIR}"
