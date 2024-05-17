#!/bin/bash

main() {
	for f in target/*.ly; do
		lilypond --silent --output=target "$f"
	done
}

main
