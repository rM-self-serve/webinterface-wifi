#!/usr/bin/env bash

pkgname='webinterface-wifi'
removefile='./remove-webint-wf.sh'
localbin='/home/root/.local/bin'
binfile="${localbin}/${pkgname}"
aliasfile="${localbin}/webint-wifi"
servicefile="/lib/systemd/system/${pkgname}.service"
configdir='/home/root/.config/webinterface-wifi'
sharedir='/home/root/.local/share/webinterface-wifi'

printf "\nRemove webinterface-wifi\n"
echo "This will not remove the /home/root/.local/bin directory nor the path in .bashrc"

read -r -p "Would you like to continue with removal? [y/N] " response
case "$response" in
[yY][eE][sS] | [yY])
	echo "Removing webinterface-wifi"
	;;
*)
	echo "exiting removal"
	[[ -f $removefile ]] && rm $removefile
	exit
	;;
esac

[[ -f $binfile ]] && rm $binfile
[[ -f $aliasfile ]] && rm $aliasfile

if systemctl --quiet is-active "$pkgname" 2>/dev/null; then
	echo "Stopping $pkgname"
	systemctl stop "$pkgname"
fi
if systemctl --quiet is-enabled "$pkgname" 2>/dev/null; then
	echo "Disabling $pkgname"
	systemctl disable "$pkgname"
fi

[[ -f $servicefile ]] && rm $servicefile

[[ -f $removefile ]] && rm $removefile

echo "Successfully removed webinterface-wifi"
echo "Did not remove ${configdir}"
echo "Did not remove ${sharedir}"
