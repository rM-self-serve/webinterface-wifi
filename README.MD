# WebInterface-Wifi

> :warning: **This is not secure and should only be used on private, trusted wifi networks. Anyone on the same wifi network can upload/download files to/from your tablet.** :warning:

This program will make the web interface available on wifi, so long as the web interface is internally accessible.

### Limitiations 

Without additional programs, 'Internally accessible' means the web interface will only be available over wifi while the device is plugged in and the web interface is enabled/reachable at 10.11.99.1, of course all while being connected to wifi.

To get around these limitations, you can use these programs to start the web interface and ensure it is internally accessible.

- https://github.com/rM-self-serve/webinterface-onboot

- https://github.com/rM-self-serve/webinterface-persist-ip

### Mobile Use

Drag and drop does not work on mobile, though it is very simple to add an upload button to circumnavigate this.

- https://github.com/rM-self-serve/upload_button

---

#### Type the following commands after ssh'ing into the ReMarkable Tablet

## Installation/Removal

**It is recommended to install via the [toltec package manager](https://toltec-dev.org/).** 

### With toltec

```
opkg update
opkg install webinterface-wifi
opkg remove webinterface-wifi
```

### No toltec

#### Install

`$ wget https://raw.githubusercontent.com/rM-self-serve/webinterface-wifi/master/install-webint-wf.sh && bash install-webint-wf.sh`

#### Remove

`$ wget https://raw.githubusercontent.com/rM-self-serve/webinterface-wifi/master/remove-webint-wf.sh && bash remove-webint-wf.sh`

## Usage

### To use webinterface-wifi, run:

`$ systemctl enable --now webinterface-wifi`

To view the web interface, type the remarkable's wifi ip address in the browser. It can be found in the copyrights and licenses tab in the settings.

- Ex : http://10.0.0.10/ 

### To change the port on the wifi interface, run 

`$ webinterface-wifi --set-port ####`


### To stop using webinterface-wifi, run:

`$ systemctl disable --now webinterface-wifi`

## Manual install

You will need docker/podman, cargo, and the cargo crate named cross. There are surely other ways to cross compile for armv7-unknown-linux-gnueabihf as well.

`cross build --target armv7-unknown-linux-gnueabihf --release`

Then copy the binary 'target/armv7-unknown-linux-gnueabihf/release/webinterface-wifi' to the device and enable/start it as a systemd service.

## How Does it Work?

This program will start a reverse proxy on the wifi interface on the port specified. The proxy will start/stop based on if usb0 interface has the ip 10.11.99.1 and the wlan0 interface has an ip address. It will automatically be available whenever you connect to a new wifi network. 

![mobile_web_ui](https://user-images.githubusercontent.com/122753594/213054617-a4f68efe-08a5-4c45-a866-6103e3e144fd.jpg)

