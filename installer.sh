#!/bin/bash

do_installation() {
    echo "Installing Findex..."

    echo "Building Findex..."
    cargo build --release

    echo "Copying files..."
    sudo cp target/release/findex /usr/bin/findex
    sudo cp target/release/findex-daemon /usr/bin/findex-daemon
    sudo echo ""
    sudo mkdir -p /opt/findex
    sudo cp css/style.css /opt/findex

    if [[ ! -f ~/.config/findex/settings.toml ]]; then
        touch ~/.config/findex/settings.toml
    fi

    if [[ ! -f ~/.config/findex/style.css ]]; then
        cp css/style.css ~/.config/findex/style.css
    fi

    echo "Installation done!"
    echo "Now add \"findex-daemon\" to autostart. You may follow your desktop environment's guide to do this."
    echo "I'm starting \"findex-daemon\" for now."
    findex-daemon
    echo "Findex can't bind hotkey in wayland."
    echo "To bind hotkey, bind the following command to your desired hotkey in the desktop environment you are using"
    echo "echo 1 > ~/.config/findex/toggle_file"
    echo "If you had Findex 0.6.0 installed, you may want to remove findex services from systemd."
}

do_removal() {
    echo "Removing files..."
    sudo rm /usr/bin/findex
    sudo rm /usr/bin/findex-daemon
    sudo rm -r /opt/findex
    killall findex-daemon findex
    echo "Removal done!"
    echo "If you added \"findex-daemon\" to autostart, you may remove it now."
}

prompt_for_installation() {
    while true; do
        read -r -p "Install findex? [Y/N] " yn
        case $yn in
        [Yy]*)
            do_installation
            break
            ;;
        [Nn]*) break ;;
        esac
    done
}

main() {
    if test -f "/usr/bin/findex"; then
        while true; do
            read -r -p "Found existing installation. Do you want to uninstall it? [Y/N] " yn
            case $yn in
            [Yy]*)
                do_removal
                prompt_for_installation
                exit
                ;;
            [Nn]*) exit ;;
            esac
        done
    fi
    do_installation
}

main
