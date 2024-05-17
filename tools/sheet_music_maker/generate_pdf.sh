#!/bin/bash

main() {
	for f in target/*.ly; do
		lilypond --silent --svg --output=svg "$f"
	done
}

main
