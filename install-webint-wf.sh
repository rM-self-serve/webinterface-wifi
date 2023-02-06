installfile='./install-webint-wf.sh'
localbin='/home/root/.local/bin'
binfile="${localbin}/webinterface-wifi"
servicefile='/lib/systemd/system/webinterface-wifi.service'

printf "\nwebinterface-wifi\n"
printf "View the web interface over wifi\n"
printf "This program will be installed in ${localbin}\n"
printf "The path will automatically be added to .bashrc if necessary\n"
read -r -p "Would you like to continue with installation? [y/N] " response
case "$response" in
[yY][eE][sS] | [yY])
    printf "Installing webinterface-wifi\n"
    ;;
*)
    printf "Exiting installer and removing script\n"
    [[ -f $installfile ]] && rm $installfile
    exit
    ;;
esac

mkdir -p $localbin

case :$PATH: in
*:$localbin:*) ;;
*) echo "export PATH=\$PATH:${localbin}" >>/home/root/.bashrc ;;
esac

[[ -f $binfile ]] && rm $binfile
wget https://github.com/rM-self-serve/webinterface-wifi/releases/download/v1.0.2/webinterface-wifi \
    -P $localbin

chmod +x $localbin/webinterface-wifi

[[ -f $servicefile ]] && rm $servicefile
wget https://raw.githubusercontent.com/rM-self-serve/webinterface-wifi/master/webinterface-wifi.service \
    -P /lib/systemd/system

systemctl daemon-reload

printf '\nFinished installing webinterface-wifi, removing install script\n\n'
printf 'Run the following command to use webinterface-wifi\n'
printf 'systemctl start webinterface-wifi\n\n'
printf 'To automatically start the application after restarting, run:\n'
printf 'systemctl enable webinterface-wifi\n\n'

[[ -f $installfile ]] && rm $installfile
