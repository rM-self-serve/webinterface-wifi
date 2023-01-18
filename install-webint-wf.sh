echo "View the web interface over wifi"
echo "This program will be installed in /home/root/.local/bin"
echo "The path will automatically be added to .bashrc"

read -r -p "Would you like to continue with installation? [y/N] " response
case "$response" in
[yY][eE][sS] | [yY])
    echo "Installing webinterface-wifi"
    ;;
*)
    echo "exiting installer" && exit
    ;;
esac

mkdir -p /home/root/.local/bin
GNU nano 4.9.3 try.sh
if [[ $PATH != *"/home/root/.local/bin"* ]]; then
    echo 'export PATH=$PATH:/home/root/.local/bin' >> /home/root/.bashrc
fi

wget https://github.com/rM-self-serve/webinterface-wifi/releases/download/v1.0.0/webinterface-wifi \
    -P /home/root/.local/bin

chmod +x /home/root/.local/bin/webinterface-wifi

wget https://raw.githubusercontent.com/rM-self-serve/webinterface-wifi/master/webinterface-wifi.service \
    -P /lib/systemd/system

systemctl daemon-reload
