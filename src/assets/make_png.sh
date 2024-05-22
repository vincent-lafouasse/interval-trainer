#!/usr/bin/env bash

TARGET_DIR="./png"

# measured in a lilypond output file
# it's ugly and hardcoded but it should work

# in pts
NOTE_HEAD_HEIGHT=44
FLAT_SIGN_HEIGHT=100
LEDGER_LINE_WIDTH=117
NOTE_SPACING=124

die() {
	>&2 echo "$1"
	exit 1
}

make_png_with_w() {
	if [ $# -ne 2 ]; then
		die "Usage: $0 file.svg width"
	fi

	name="$(basename "$1")"

	if [[ $name != *svg ]] ;
	then
		die "Usage: $0 file.svg width"
	fi

	base_name="${name/\.svg/}"    

	inkscape -w "$2" "$1" -o "${TARGET_DIR}/${base_name}".png
}

make_png_with_h() {
	if [ $# -ne 2 ]; then
		die "Usage: $0 file.svg width"
	fi

	name="$(basename "$1")"

	if [[ $name != *svg ]] ;
	then
		die "Usage: $0 file.svg width"
	fi

	base_name="${name/\.svg/}"    

	inkscape -h "$2" "$1" -o "${TARGET_DIR}/${base_name}".png
}

# 1000 -> 274
# ?    -> 117 ?
# 117 * 1000 / 274 = 427

main() {
	# make_png_with_w "svg/from_lilypond/treble_staff.svg" 1000
	# make_png_with_h "svg/from_wikipedia/WholeNote.svg" "$NOTE_HEAD_HEIGHT"
	# make_png_with_h "svg/public_domain/Flat.svg" "$FLAT_SIGN_HEIGHT"
	# make_png_with_h "svg/public_domain/DoubleFlat.svg" "$FLAT_SIGN_HEIGHT"
	make_png_with_w "svg/from_lilypond/ledger_line.svg" 427
}

main
