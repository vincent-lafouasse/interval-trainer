main() {
	for f in target/*.ly; do
		lilypond --silent "$f"
	done
}

main
