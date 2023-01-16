# WebInterface-Wifi

### THIS IS NOT SECURE AND SHOULD ONLY BE USED ON PRIVATE, TRUSTED WIFI NETWORKS. DO NOT INSTALL IF YOU EVER CONNECT TO PUBLIC WIFI. ANYONE ON THE SAME WIFI NETWORK CAN UPLOAD/DOWNLOAD FILES TO/FROM YOUR TABLET.

This program will make the web interface available on wifi so long as the web interface is accessible internally.

You will type the following commands after ssh'ing into your ReMarkable Tablet.

## Install

`wget -O - https://raw.githubusercontent.com/rM-self-serve/webinterface-wifi/master/install-webint-wf.sh | sh`

## Remove

`wget -O - https://raw.githubusercontent.com/rM-self-serve/webinterface-wifi/master/remove-webint-wf.sh | sh`

## Use

webinterface-wifi will already be running on installation! 

To view the web interface, find the remarkable's wifi ip address and type it in the browser.
- Ex : http://10.0.0.10/ 

To change the port on the wifi interface, run 

- `webinterface-wifi --set-port 7777`

## Stop

Stop program:

- `systemctl stop webinterface-wifi`

Do not run program on boot: 

- `systemctl disable webinterface-wifi`

## Manual install

You will need docker/podman, cargo, and the cargo crate named cross. There are surely other ways to cross compile for armv7-unknown-linux-gnueabihf as well.

`cross build --target armv7-unknown-linux-gnueabihf --release`

Then copy the binary 'target/armv7-unknown-linux-gnueabihf/release/webinterface-wifi' to the device and enable/start it as a systemd service.

## How Does it Work?

This program will start a reverse proxy on the wifi interface on the port specified. The proxy will start/stop based on if usb0 interface has the ip 10.11.99.1 and the wlan0 interface has an ip address. It will automatically be available whenever you connect to a new wifi network. 
