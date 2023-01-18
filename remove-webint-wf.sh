echo "Remove webinterface-wifi"
echo "This will not remove the /home/root/.local/bin directory nor the path from .bashrc"

read -r -p "Would you like to continue with removal? [y/N] " response
case "$response" in
[yY][eE][sS] | [yY])
    echo "Removing webinterface-wifi"
    ;;
*)
    echo "exiting removal" && exit
    ;;
esac

rm /home/root/.local/bin/webinterface-wifi

systemctl disable webinterface-wifi --now

rm /lib/systemd/system/webinterface-wifi.service