#!/usr/bin/env bash

webinterface_wifi_sha256sum='eb2f52742225e5dfa9d44edf4e9a6e90c56affb0c59efc2ddf4a51931c245b55'
service_file_sha256sum='ba0472927c1ed0f7c201973c32ebcb3dcb6a7186db9a0e9a1466d3308c9f6621'

installfile='./install-webint-wf.sh'
localbin='/home/root/.local/bin'
binfile="${localbin}/webinterface-wifi"
aliasfile="${localbin}/webint-wifi"
servicefile='/lib/systemd/system/webinterface-wifi.service'
configdir='/home/root/.config/webinterface-wifi'
configfile="${configdir}/config.toml"
sharedir='/home/root/.local/share/webinterface-wifi'
ssldir="${sharedir}/ssl"
authdir="${sharedir}/auth"
assetsdir="${sharedir}/assets"

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
	echo "sha256sum did not pass, error downloading webinterface-wifi"
	echo "Exiting installer and removing installed files"
	[[ -f $binfile ]] && rm $binfile
	[[ -f $installfile ]] && rm $installfile
	[[ -f $servicefile ]] && rm $servicefile
	exit
}

[[ -f $binfile ]] && rm $binfile
wget https://github.com/rM-self-serve/webinterface-wifi/releases/download/v2.0/webinterface-wifi \
	-P $localbin

if ! sha256sum -c <(echo "$webinterface_wifi_sha256sum  $binfile") >/dev/null 2>&1; then
	sha_fail
fi

chmod +x $binfile
ln -s $binfile $aliasfile

[[ -f $servicefile ]] && rm $servicefile
wget https://raw.githubusercontent.com/rM-self-serve/webinterface-wifi/master/webinterface-wifi.service \
	-P /lib/systemd/system

if ! sha256sum -c <(echo "$service_file_sha256sum  $servicefile") >/dev/null 2>&1; then
	sha_fail
fi

mkdir -p $configdir
if ! [ -f $configfile ]; then
	wget https://raw.githubusercontent.com/rM-self-serve/webinterface-wifi/master/config/config.default.toml \
		-O $configfile
fi

mkdir -p $ssldir
mkdir -p $authdir
mkdir -p $assetsdir
wget https://raw.githubusercontent.com/rM-self-serve/webinterface-wifi/master/assets/favicon.ico \
	-P $assetsdir

systemctl daemon-reload

printf '\nFinished installing webinterface-wifi, removing install script\n\n'
printf 'Run the following command to use webinterface-wifi\n'
printf 'systemctl start webinterface-wifi\n\n'
printf 'To automatically start the application after restarting, run:\n'
printf 'systemctl enable webinterface-wifi\n\n'

[[ -f $installfile ]] && rm $installfile
