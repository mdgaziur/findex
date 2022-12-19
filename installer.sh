#!/bin/bash

do_installation() {
    echo "Installing Findex..."

    if [[ -z $WAYLAND_DISPLAY ]]; then
        echo "Building for xorg..."
        if cargo build --release --features xorg; then
            echo "Build complete"
        else
            echo "Building failed. Exiting"
            exit 1
        fi
    else
        echo "Building for wayland..."
        if cargo build --release --features wayland; then
            echo "Build complete"
        else
            echo "Building failed. Exiting"
            exit 1
        fi
    fi

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
    if [[ $WAYLAND_DISPLAY ]]; then
        echo "Findex can't bind hotkey in wayland."
        echo "To bind hotkey, bind the following command to your desired hotkey in the desktop environment you are using"
        echo "echo 1 > ~/.config/findex/toggle_file"
    fi
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
            read -r -p "Already found existing installation. Do you want to remove findex? [Y/N] " yn
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
