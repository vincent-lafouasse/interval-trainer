#!/usr/bin/env bash

set -o xtrace

setup_globals() {
	MAIN="main"

	AUX_DIR="aux"
	TARGET_DIR="target"

	mkdir "${AUX_DIR}"
	mkdir "${TARGET_DIR}"

	TEMPLATE="template/template.ly"
	LILY_FILE="${AUX_DIR}/${MAIN}.ly"
	OUTPUT="${TARGET_DIR}/${MAIN}.pdf"
}

fill_template() {
	TIME_SIG="2/4"
	CLEF="treble"
	NOTES="c'2"
	export TIME_SIG CLEF NOTES

	envsubst < "${TEMPLATE}" > "${LILY_FILE}"
}

clean() {
	rm -rf "${AUX_DIR}"
}

mr_proper() {
	clean
	rm -rf "${TARGET_DIR}"
}

#################################################################################
setup_globals

if [[ "$1" == "--clean" ]]; then
	clean
	exit 0
fi
if [[ "$1" == "--mrproper" ]]; then
	mr_proper
	exit 0
fi

fill_template
lilypond --output="${TARGET_DIR}" "${LILY_FILE}"
open "${OUTPUT}"

