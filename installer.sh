#!/bin/bash
existing_installation=false

check_existing() {
	if test -f "/usr/bin/findex"; then
	  existing_installation=true
	fi
}

do_installation() {
  echo Installing Findex
	cargo build --release
	echo Copying files...
	sudo cp target/release/findex /usr/bin/findex
	sudo mkdir -p /opt/findex
	sudo cp css/style.css /opt/findex
	echo Installation done!
}

do_removal() {
  echo Removing files...
  sudo rm /usr/bin/findex
  sudo rm -r /opt/findex
  echo Removal done!
}

prompt_for_installation() {
  while true; do
    read -r -p "Install findex? [Y/N] " yn
    case $yn in
      [Yy]* ) do_installation; break;;
      [Nn]* ) break;;
    esac
  done;
}

main() {
  check_existing
	if $existing_installation; then
		while true; do
			read -r -p "Already found existing installation. Do you want to remove findex? [Y/N] " yn
      case $yn in
        [Yy]* ) do_removal; prompt_for_installation; exit;;
        [Nn]* ) exit;;
      esac
		done;
	fi;
	do_installation;
}

main
