#!/usr/bin/env bash

webinterface_wifi_sha256sum='dd66455dab70ad13f5258a8fa1e624c90e691fb96cf3fac39821f09d83ef5353'
service_file_sha256sum='82e19e82fb860c20e15d5b50fde3e06fe5ffe648f3c5f9aaf3b02edc4b81e196'

installfile='./install-webint-wf.sh'
localbin='/home/root/.local/bin'
binfile="${localbin}/webinterface-wifi"
servicefile='/lib/systemd/system/webinterface-wifi.service'

printf "\nwebinterface-wifi\n"
printf "View the web interface over wifi\n"
printf "This program will be installed in %s\n" "${localbin}"
printf "%s will be added to the path in ~/.bashrc if necessary\n" "${localbin}"

read -r -p "Would you like to continue with installation? [y/N] " response
case "$response" in
[yY][eE][sS] | [yY])
	printf "Installing webinterface-wifi\n"
	;;
*)
	printf "Exiting installer and removing install script\n"
	[[ -f $installfile ]] && rm $installfile
	exit
	;;
esac

mkdir -p $localbin

case :$PATH: in
*:$localbin:*) ;;
*) echo "PATH=\"${localbin}:\$PATH\"" >>/home/root/.bashrc ;;
esac

function sha_fail() {
	echo "sha256sum did not pass, error downloading webinterface_wifi"
	echo "Exiting installer and removing installed files"
	[[ -f $binfile ]] && rm $binfile
	[[ -f $installfile ]] && rm $installfile
	[[ -f $servicefile ]] && rm $servicefile
	exit
}

[[ -f $binfile ]] && rm $binfile
wget https://github.com/rM-self-serve/webinterface-wifi/releases/download/v1.0.2/webinterface-wifi \
	-P $localbin

if ! sha256sum -c <(echo "$webinterface_wifi_sha256sum  $binfile") >/dev/null 2>&1; then
	sha_fail
fi

chmod +x $localbin/webinterface-wifi

[[ -f $servicefile ]] && rm $servicefile
wget https://raw.githubusercontent.com/rM-self-serve/webinterface-wifi/master/webinterface-wifi.service \
	-P /lib/systemd/system

if ! sha256sum -c <(echo "$service_file_sha256sum  $servicefile") >/dev/null 2>&1; then
	sha_fail
fi

systemctl daemon-reload

printf '\nFinished installing webinterface-wifi, removing install script\n\n'
printf 'Run the following command to use webinterface-wifi\n'
printf 'systemctl start webinterface-wifi\n\n'
printf 'To automatically start the application after restarting, run:\n'
printf 'systemctl enable webinterface-wifi\n\n'

[[ -f $installfile ]] && rm $installfile
