removefile='./remove-webint-wf.sh'

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

rm /home/root/.local/bin/webinterface-wifi

systemctl disable webinterface-wifi --now

rm /lib/systemd/system/webinterface-wifi.service

[[ -f $removefile ]] && rm $removefile

echo "Successfully removed webinterface-wifi"
