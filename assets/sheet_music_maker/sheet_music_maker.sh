#!/usr/bin/env bash

# set -o xtrace

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
	if [[ $1 == "--clean" ]]; then
		clean
		exit 0
	fi
	if [[ $1 == "--mrproper" ]]; then
		mr_proper
		exit 0
	fi
}

parse_instructions() {
	JSON="$1"
	CLEF="$(jq ".clef" "${JSON}" | sed 's/"//g')"
	SUBDIVISION="$(jq ".subdivision" "${JSON}" | sed 's/"//g')"
	NOTES="$(jq ".notes" "${JSON}" | sed 's/"//g')"
	echo "${CLEF}"
	echo "${SUBDIVISION}"
	echo "${NOTES}"
}

parse_note() {
	# convert scientific note names like A4 G2 to Lilypond style notation a' g,
	note="$1"
	if ! [[ ${note} =~ ^[A-G][0-8]$ ]]; then
		echo "Error: invalid note $1" >&2
		return 1
	fi
	note_name=$(printf %.1s "${note}" | tr '[:upper:]' '[:lower:]')
	octave="${note:0-1}"
	case "${octave}" in
	0)
		echo "${note_name},,,"
		;;
	1)
		echo "${note_name},,"
		;;
	2)
		echo "${note_name},"
		;;
	3)
		echo "${note_name}"
		;;
	4)
		echo "${note_name}'"
		;;
	5)
		echo "${note_name}''"
		;;
	6)
		echo "${note_name}'''"
		;;
	7)
		echo "${note_name}''''"
		;;
	8)
		echo "${note_name}'''''"
		;;
	*)
		# unreacheable
		;;
	esac
}

fill_template() {
	CLEF="treble"
	SUBDIVISION='2'
	NOTES="$1${SUBDIVISION} $2"
	export CLEF NOTES

	envsubst <"${TEMPLATE}" >"${LILY_FILE}"
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

parse_cli_args "$@" || {
	echo "Error: invalid input" >&2
	return 1
}

parse_instructions "sheet_music_instructions.json"

fill_template "$(parse_note C4)" "$(parse_note G5)"

lilypond --silent --output="${TARGET_DIR}" "${LILY_FILE}"

open "${OUTPUT_FILE}"

clean
