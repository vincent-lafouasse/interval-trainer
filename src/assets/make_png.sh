#!/usr/bin/env bash

TARGET_DIR="./png"

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

main() {
	make_png_with_w "svg/from_lilypond/treble_staff.svg" 1000
	make_png_with_h "svg/from_wikipedia/WholeNote.svg" 44
}

main