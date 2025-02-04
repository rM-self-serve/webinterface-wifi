#!/usr/bin/env bash

# --- Values replaced in github actions ---
version='VERSION'
webinterface_wifi64_sha256sum='WEBINTERFACE_WIFI64_SHA256SUM'
webinterface_wifi32_sha256sum='WEBINTERFACE_WIFI32_SHA256SUM'
service_file_sha256sum='SERVICE_FILE_SHA256SUM'
config_sha256sum='CONFIG_SHA256SUM'
repo_name='REPO_NAME'
gowget_checksum='GOWGET_CHECKSUM'
wget_checksum='WGET_CHECKSUM'
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

platform=$(uname -m)

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

sha_check() {
	if ! sha256sum -c <(echo "$1  $2") >/dev/null 2>&1; then
		echo "$2 sha256sum did not pass, error downloading ${pkgname}"
		echo "Exiting installer and removing installed files"
		[[ -f $binfile ]] && rm $binfile
		[[ -f $servicefile ]] && rm $servicefile
		[[ -f $wget_path ]] && rm $wget_path
		exit 1
	fi
}

extract_downloader() {
	wget_path="/tmp/downloader-$version"
	[[ -f "$wget_path" ]] && rm "$wget_path"
	PAYLOAD_LINE=$(awk '/^__PAYLOAD__/ { print NR + 1; exit 0; }' $0)

	if [[ "$platform" == "aarch64" ]]; then
		tail -n +$PAYLOAD_LINE $0 | tar -xzf - gowget -O > "$wget_path" 
		sha_check "$gowget_checksum" "$wget_path"
	else
		tail -n +$PAYLOAD_LINE $0 | tar -xzf - wget -O > "$wget_path" 
		sha_check "$wget_checksum" "$wget_path"
	fi
	chmod 755 "$wget_path"
}

install() {
	printf "\n${pkgname}\n"
	printf "View the web interface over wifi\n"
	printf "This program will be installed in %s\n" "${localbin}"
	printf "%s will be added to the path in ~/.bashrc if necessary\n" "${localbin}"

	extract_downloader

	mkdir -p $localbin
	case :$PATH: in
	*:$localbin:*) ;;
	*) echo "PATH=\"${localbin}:\$PATH\"" >>/home/root/.bashrc ;;
	esac


	[[ -f $binfile ]] && rm $binfile
	if [[ "$platform" == "aarch64" ]]; then
		"$wget_path" -O "$binfile" \
			"https://github.com/rM-self-serve/${repo_name}/releases/download/${version}/${pkgname}-arm64"
		sha_check $webinterface_wifi64_sha256sum $binfile
	else
		"$wget_path" -O "$binfile" \
			"https://github.com/rM-self-serve/${repo_name}/releases/download/${version}/${pkgname}-arm32"
		sha_check $webinterface_wifi32_sha256sum $binfile
	fi

	chmod +x $binfile
	ln -s $binfile $aliasfile

	[[ -f $servicefile ]] && rm $servicefile
	"$wget_path" -O "$servicefile" \
		"https://github.com/rM-self-serve/${repo_name}/releases/download/${version}/${pkgname}.service"	

	sha_check "$service_file_sha256sum" "$servicefile"

	if ! [ -f $configfile ]; then
		mkdir -p $configdir
		"$wget_path" -O "$configfile" \
			"https://github.com/rM-self-serve/${repo_name}/releases/download/${version}/config.default.toml"
		
		sha_check "$config_sha256sum" "$configfile"
	fi

	mkdir -p $ssldir
	mkdir -p $authdir
	mkdir -p $assetsdir

	[[ -f $faviconfile ]] && rm $faviconfile
	"$wget_path" -O "favicon.ico" \
		"https://github.com/rM-self-serve/${repo_name}/releases/download/${version}/favicon.ico"	

	systemctl daemon-reload

	printf "\nFinished installing ${pkgname}, removing install script\n\n"
	printf "Run the following command to use ${pkgname}\n"
	printf "systemctl enable ${pkgname} --now\n\n"

	[[ -f $wget_path ]] && rm $wget_path
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
exit 0
