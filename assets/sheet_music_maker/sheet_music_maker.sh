#!/usr/bin/env bash

set -o xtrace

setup_globals() {
	MAIN="main"

	AUX_DIR="aux"
	TARGET_DIR="target"

	mkdir -p "${AUX_DIR}"
	mkdir -p "${TARGET_DIR}"

	TEMPLATE="template/template.ly"
	LILY_FILE="${AUX_DIR}/${MAIN}.ly"
	OUTPUT_FILE="${TARGET_DIR}/${MAIN}.pdf"
}

parse_cli_args() {
	if [[ "$1" == "--clean" ]]; then
		clean
		exit 0
	fi
	if [[ "$1" == "--mrproper" ]]; then
		mr_proper
		exit 0
	fi
}

fill_template() {
	CLEF="treble"
	NOTES="a'2"
	export CLEF NOTES

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
parse_cli_args "$@" || { echo "Error: invalid input" >&2; return 1; }
fill_template
lilypond --silent --output="${TARGET_DIR}" "${LILY_FILE}"
open "${OUTPUT_FILE}"
clean

