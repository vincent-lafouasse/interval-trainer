#!/usr/bin/env bash

# set -o xtrace

AUX_DIR="aux"
TARGET_DIR="target"

mkdir -p "${AUX_DIR}"
mkdir -p "${TARGET_DIR}"

TEMPLATE="template/template.ly"

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

generate() {
	# setup variables
	NOTE=$1
	OCTAVE=$2
	PARSED_NOTE=$(parse_note "$NOTE$OCTAVE")
	CLEF=$3

	OUTPUT_NAME="$NOTE${OCTAVE}_$CLEF"
	LILY_FILE="${AUX_DIR}/${OUTPUT_NAME}.ly"
	OUTPUT_FILE="${TARGET_DIR}/${OUTPUT_NAME}.pdf"

	# fill lilypond file
	fill_template "${TEMPLATE}" "${LILY_FILE}"
	lilypond --silent --output="${TARGET_DIR}" "${LILY_FILE}"
	open "${OUTPUT_FILE}"
}

fill_template() {
	NOTES="${PARSED_NOTE}1 ${PARSED_NOTE}1"
	export CLEF NOTES

	envsubst <"$1" >"$2"
}

#################################################################################
main() {
	generate "C" "4" "treble"
}

main "$@"
