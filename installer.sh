check_existing() {
	if test -f "/usr/bin/findex"; then
		return true
	fi
}

do_installation() {
	cargo build --release
	echo Copying files...
	sudo cp target/release/findex /usr/bin/findex
	sudo mkdir -p /opt/findex
	sudo cp css/style.css /opt/findex
	echo Installation done!
}

main() {
	existing_installation=$(check_existing)

	if $existing_installation; then
		while true; do
			read -p "Already found existing_installation. Do you want to continue? [y/N]" yn
			case $yn in
				[Yy]*) do_installation; return 0 ;;
				[Nn]*) return 0 ;;
			esac
		done;
	fi;
	do_installation;
}

main
