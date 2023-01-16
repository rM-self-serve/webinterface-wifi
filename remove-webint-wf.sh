echo "Remove webinterface-wifi"

rm /usr/bin/webinterface-wifi

systemctl disable webinterface-wifi --now

rm /lib/systemd/system/webinterface-wifi.service