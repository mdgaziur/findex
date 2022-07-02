#!/bin/bash

install_systemd_service() {
  sudo cp findex.service /etc/systemd/user/
  sudo cp findex-restarter.service /etc/systemd/user
  sudo cp findex-restarter.path /etc/systemd/user/
  systemctl --user daemon-reload
  systemctl --user enable findex
  systemctl --user start findex
  systemctl --user enable findex-restarter.path
  systemctl --user start findex-restarter.path
}

prompt_for_systemd_service_installation() {
  while true; do
    read -r -p "Install SystemD service file? [Y/N] " yn
    case $yn in
      [Yy]* ) install_systemd_service; break;;
      [Nn]* ) break;;
    esac
  done;
}

do_installation() {
  echo "Installing Findex..."
	if cargo build --release; then
    echo "Build complete"
  else
    echo "Building failed. Exiting"
    exit 1
  fi

	echo "Copying files..."
	sudo cp target/release/findex /usr/bin/findex
	sudo cp findex_toggle /usr/bin/findex_toggle
	sudo echo ""
	sudo mkdir -p /opt/findex
	sudo cp css/style.css /opt/findex
	echo "Installation done!"
  prompt_for_systemd_service_installation
}

uninstall_systemd_service() {
  echo "Stopping service..."
  systemctl --user stop findex
  systemctl --user stop findex-restarter.path

  echo "Disabling service..."
  systemctl --user disable findex
  systemctl --user disable findex-restarter.path

  echo "Removing service..."
  sudo rm -rf /etc/systemd/user/findex.service
  sudo rm -rf /etc/systemd/user/findex-restarter.service
  sudo rm -rf /etc/systemd/user/findex-restarter.path
}

prompt_for_systemd_service_removal() {
  while true; do
    read -r -p "Uninstall SystemD service file? [Y/N] " yn
    case $yn in
      [Yy]* ) uninstall_systemd_service; break;;
      [Nn]* ) break;;
    esac
  done;
}

do_removal() {
  prompt_for_systemd_service_removal
  echo "Removing files..."
  sudo rm /usr/bin/findex
  sudo rm -r /opt/findex
  echo "Removal done!"
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
	if test -f "/usr/bin/findex"; then
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
