#!/usr/bin/env bash

TARGET_DIR="./png"

die() {
	>&2 echo "$1"
	exit 1
}

main() {
	if [ $# -ne 1 ]; then
		die "Usage: $0 file.svg"
	fi

	name="$(basename "$1")"

	if [[ $name != *svg ]] ;
	then
		die "Usage: $0 file.svg"
	fi

	base_name="${name/\.svg/}"    

	inkscape -w 1000 "$1" -o "${TARGET_DIR}/${base_name}".png
}

main "$@"
