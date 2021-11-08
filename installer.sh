#!/bin/bash
check_existing() {
	if test -f "/usr/bin/findex"; then
		return 1
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

do_removal() {
    cargo clean
    echo Removing files...
    sudo rm /usr/bin/findex
    sudo rm -r /opt/findex
    echo Removal done! 
}

main() {
	existing_installation=$(check_existing)
	if $existing_installation; then
		while true; do
			read -p "Already found existing installation. Do you want to remove findex? [y/N]" yn
			case $yn in
		    [Yy]*)
                do_removal; return 0
                ;;
            *)
                read -p "Do you want to reinstall findex? [y/N]" yn
                case $yn in
                [Yy]*)
                    do_installation; return 0
                    ;;
                *)
                    return 0
                    ;;
			    esac
                ;;
            esac
		done;
	fi;
	do_installation;
}

main
