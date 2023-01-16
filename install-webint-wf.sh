echo "View the web interface over wifi"

wget https://github.com/rM-self-serve/webinterface-wifi/releases/download/v1.0.0/webinterface-wifi \
-P /usr/bin

chmod +x /usr/bin/webinterface-wifi

wget https://raw.githubusercontent.com/rM-self-serve/webinterface-wifi/master/webinterface-wifi.service \
 -P /lib/systemd/system

systemctl daemon-reload
systemctl enable webinterface-wifi --now