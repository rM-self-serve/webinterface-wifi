#!/usr/bin/env bash

# --- Values replaced in github actions ---
version='VERSION'
webinterface_wifi_sha256sum='WEBINTERFACE_WIFI_SHA256SUM'
service_file_sha256sum='SERVICE_FILE_SHA256SUM'
config_sha256sum='CONFIG_SHA256SUM'
# -----------------------------------------

pkgname='webinterface-wifi'
installfile='./install-webint-wf.sh'
localbin='/home/root/.local/bin'
binfile="${localbin}/${pkgname}"
aliasfile="${localbin}/webint-wifi"
servicefile="/lib/systemd/system/${pkgname}.service"
configdir="/home/root/.config/${pkgname}"
configfile="${configdir}/config.toml"
sharedir="/home/root/.local/share/${pkgname}"
ssldir="${sharedir}/ssl"
authdir="${sharedir}/auth"
assetsdir="${sharedir}/assets"
faviconfile="${assetsdir}/favicon.ico"

wget_path=/home/root/.local/share/rM-self-serve/wget
wget_remote=http://toltec-dev.org/thirdparty/bin/wget-v1.21.1-1
wget_checksum=c258140f059d16d24503c62c1fdf747ca843fe4ba8fcd464a6e6bda8c3bbb6b5


main() {
	case "$@" in
	'install' | '')
		install
		;;
	'remove')
		remove
		;;
	*)
		echo 'input not recognized'
		cli_info
		exit 0
		;;
	esac
}

cli_info() {
	echo "${pkgname} installer ${version}"
	echo -e "${CYAN}COMMANDS:${NC}"
	echo '  install'
	echo '  remove'
	echo ''
}


sha_fail() {
	echo "sha256sum did not pass, error downloading ${pkgname}"
	echo "Exiting installer and removing installed files"
	[[ -f $binfile ]] && rm $binfile
	[[ -f $installfile ]] && rm $installfile
	[[ -f $servicefile ]] && rm $servicefile
	exit
}

install() {
	printf "\n${pkgname}\n"
	printf "View the web interface over wifi\n"
	printf "This program will be installed in %s\n" "${localbin}"
	printf "%s will be added to the path in ~/.bashrc if necessary\n" "${localbin}"

	if [ -f "$wget_path" ] && ! sha256sum -c <(echo "$wget_checksum  $wget_path") > /dev/null 2>&1; then
	    rm "$wget_path"
	fi
	if ! [ -f "$wget_path" ]; then
	    echo "Fetching secure wget"
	    # Download and compare to hash
	    mkdir -p "$(dirname "$wget_path")"
	    if ! wget -q "$wget_remote" --output-document "$wget_path"; then
		echo "Error: Could not fetch wget, make sure you have a stable Wi-Fi connection"
		exit 1
	    fi
	fi
	if ! sha256sum -c <(echo "$wget_checksum  $wget_path") > /dev/null 2>&1; then
	    echo "Error: Invalid checksum for the local wget binary"
	    exit 1
	fi
	chmod 755 "$wget_path"

	mkdir -p $localbin

	case :$PATH: in
	*:$localbin:*) ;;
	*) echo "PATH=\"${localbin}:\$PATH\"" >>/home/root/.bashrc ;;
	esac


	[[ -f $binfile ]] && rm $binfile
	"$wget_path" https://github.com/rM-self-serve/${pkgname}/releases/download/${version}/${pkgname} \
		-P $localbin

	if ! sha256sum -c <(echo "$webinterface_wifi_sha256sum  $binfile") >/dev/null 2>&1; then
		sha_fail
	fi

	chmod +x $binfile
	ln -s $binfile $aliasfile

	[[ -f $servicefile ]] && rm $servicefile
	"$wget_path" https://github.com/rM-self-serve/${pkgname}/releases/download/${version}/${pkgname}.service \
		-P /lib/systemd/system

	if ! sha256sum -c <(echo "$service_file_sha256sum  $servicefile") >/dev/null 2>&1; then
		sha_fail
	fi

	if ! [ -f $configfile ]; then
		mkdir -p $configdir
		"$wget_path" https://github.com/rM-self-serve/${pkgname}/releases/download/${version}/config.default.toml \
			-O $configfile

		if ! sha256sum -c <(echo "$config_sha256sum  $configfile") >/dev/null 2>&1; then
			sha_fail
		fi
	fi

	mkdir -p $ssldir
	mkdir -p $authdir
	mkdir -p $assetsdir

	[[ -f $faviconfile ]] && rm $faviconfile
	"$wget_path" https://github.com/rM-self-serve/${pkgname}/releases/download/${version}/favicon.ico \
		-P $assetsdir

	systemctl daemon-reload

	printf "\nFinished installing ${pkgname}, removing install script\n\n"
	printf "Run the following command to use ${pkgname}\n"
	printf "systemctl enable ${pkgname} --now\n\n"

	[[ -f $installfile ]] && rm $installfile
}

remove() {
	printf "Remove ${pkgname}\n"
	echo "This will not remove the /home/root/.local/bin directory nor the path in .bashrc"

	[[ -f $binfile ]] && rm $binfile
	[[ -L $aliasfile ]] && rm $aliasfile

	if systemctl --quiet is-active "$pkgname" 2>/dev/null; then
		echo "Stopping $pkgname"
		systemctl stop "$pkgname"
	fi
	if systemctl --quiet is-enabled "$pkgname" 2>/dev/null; then
		echo "Disabling $pkgname"
		systemctl disable "$pkgname"
	fi

	[[ -f $servicefile ]] && rm $servicefile
	[[ -f $installfile ]] && rm $installfile
	rmdir "$sharedir"/*/* "$sharedir"/* "$sharedir" 2> /dev/null || true

	echo "Tried to remove ${sharedir}"
	echo "Did not remove ${configdir}"
	echo "Successfully removed webinterface-wifi"
}

main "$@"
