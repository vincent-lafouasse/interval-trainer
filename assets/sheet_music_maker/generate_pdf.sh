#!/bin/bash

main() {
	for f in target/*.ly; do
		lilypond --silent --svg --output=target "$f"
	done
}

main
