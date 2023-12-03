# WebInterface-Wifi

This program will make the ReMarkable Tablet's web interface available on wifi. 

Password authentication and SSL supported, along with the ability to only run when connected to certain wifi networks.  

![demo](https://github.com/rM-self-serve/webinterface-wifi/assets/122753594/51263588-0efd-46a0-94fc-5e936cdc7615)

### Limitations 

Without additional programs, the web interface will only be available over wifi while the device is plugged in and the web interface is enabled/reachable at 10.11.99.1.
To get around this, use [webinterface-onboot](https://github.com/rM-self-serve/webinterface-onboot).

Drag and drop does not work well on mobile, though it is simple to add an [upload button to circumnavigate this](https://github.com/rM-self-serve/upload_button).

---

#### Type the following commands after ssh'ing into the ReMarkable Tablet

## Installation/Removal

**It is recommended to install via the [toltec package manager](https://toltec-dev.org/).** 

### With toltec

```
$ opkg update
$ opkg install webinterface-wifi
$ opkg remove webinterface-wifi
```

### No toltec

#### Install

```$ wget https://raw.githubusercontent.com/rM-self-serve/webinterface-wifi/master/install-webint-wf.sh && bash install-webint-wf.sh```

#### Remove

```$ wget https://raw.githubusercontent.com/rM-self-serve/webinterface-wifi/master/remove-webint-wf.sh && bash remove-webint-wf.sh```

## Usage

### To use webinterface-wifi, run:

`$ systemctl enable --now webinterface-wifi`

To view the web interface, type the remarkable's wifi ip address in the browser. It can be found in the copyrights and licenses tab in the settings. Ex : http://10.0.0.10/ 

## Security :warning:

**By default, the web interface runs without authentication or encryption.** This means anyone on the same wifi network can access your files. The only way to secure your device on public wifi is by enabling both authentication and encryption.

### SSL/Network Encryption
Obtain an SSL certificate and the corresponding private key, a self signed cert is sufficient. These can be placed at the following paths:

```
# Certificate default path
/home/root/.local/share/webinterface-wifi/ssl/ssl_cert.pem
# If installed with Toltec
/opt/etc/webinterface-wifi/ssl/ssl_cert.pem

# Private Key default path
/home/root/.local/share/webinterface-wifi/ssl/ssl_priv.rsa 
# If installed with Toltec
/opt/etc/webinterface-wifi/ssl/ssl_priv.rsa 
```

Or the paths can be specified in config.toml:
```toml
[conf]
ssl_cert_path="/etc/ssl/ssl_cert.pem"
ssl_priv_path="/etc/ssl/ssl_priv.rsa"
# ...
```

Then enable ssl in each network:
```toml
[networks.arbitrary_name]
ssl=true
# ...

[undefined_networks]
ssl=true
# ...
```
> An SSL keypair will be included in this repository for testing purposes. This should not be considered secure as someone determined could use the provided private key to decrypt your network traffic. These will need to be downloaded separately. 

### Login/Authentication

A login consists of a username and password. The username will not be saved so ensure to remember it along with the password. Since the device is not encrypted, it is important to use a unique password not used elsewhere. **Even with login enabled, anyone on the same wifi network can read whatever files are uploaded/downloaded, use SSL to mitigate this vulnerability.** 

To create a login, run the following command and enter a username and password:
```
$ webinterface-wifi create-login
User: myuser
Password: 
Retype Password: 
```
```
# Login file default path
/home/root/.local/share/webinterface-wifi/auth/login.pass 
# If installed with Toltec
/opt/etc/webinterface-wifi/auth/login.pass 
```
This will create a login file at the default path so that it does not need to be specified in config.toml.

To specify in config.toml:
```toml
[conf]
login_path="/etc/auth/login.pass"
# ...
```
> The password is not stored in plaintext.

## Multiple Wifi Networks
Each wifi network can have settings defined in the config:
```toml
[networks.home]
router_ssid="Home's Wifi Name"
ssl=false
login_enforced=false
listen_ip="auto"
listen_port=80

[networks.coffeshop]
router_ssid="Coffeshop's Wifi Name"
ssl=true
login_enforced=true
listen_ip="auto"
listen_port=443
http_redirect_port=80
```
When the wifi network with the matching SSID connects, these settings will be applied.

If the connected network is not defined (and is not filtered), it will run with the settings of the [undefined_networks] field:
```toml
[undefined_networks]
ssl=false
login_enforced=false
listen_ip="auto"
listen_port=80
```


### Wifi Network Filtering
If you would like your webinterface to be available on your home wifi network but not the airport, you can configure network filtering.

#### Allowlist
The more secure option, this feature ensures the webinterface will only be available on defined networks. 

```toml
[conf]
network_filter="allowlist"

[networks.home]
router_ssid="Home Wifi Name"
# ...

[allowlist]
networks=[ "home" ]
```

#### Blocklist
This option lets you define which networks the webinterface should NOT run on, while running on any network that is not in the list.
```toml
[conf]
network_filter="blocklist"

[networks.airport]
router_ssid="Airport Wifi Name"

[blocklist]
networks=[ "airport" ]

[undefined_networks]
# ...
```

## Editing the Config
> Webinterface-Wifi needs to be explicitly reloaded when the config is edited.

:warning: An invalid config will stop the daemon from running. Restart it with:
```
$ systemctl daemon-reload
$ systemctl restart webinterface-wifi
```

Open in the default config in your editor of choice, defined by the environment variable $EDITOR, or nano if not defined:
```
$ webinterface-wifi edit
```

After saving the file, validation will be performed on the config where potential errors will be raised.

```
# Default Config Path
/home/root/.config/webinterface-wifi/config.toml
```

## Reloading the Config
You may wish to edit the config and reload the program without restarting the daemon.
```
$ webinterface-wifi reload       
Config Valid
Config Reloaded
```

## Validation/Mock Run
To ensure your modified config is valid and do a mock run to see which network may be currently active.
```
$ webinterface-wifi validate
```

## Network Information
```
$ webinterface-wifi net-info
wifi interface: wlan0 ip: 192.168.1.93
webint ip exists: 10.11.99.1
router ssid: Home Wifi Name
```

## Listen IP
In the definition of a network, the 'listen_ip' field is set to "auto" by default. This will find the ip address of the wifi interface and start the server on it. It can also be configured to run on a static ipv4 ip address. The webinterface will be available on this ip address when the device has wifi.
```toml
[networks.home]
listen_ip="0.0.0.0"

[networks.coffeshop]
listen_ip="auto"

[undefined_networks]
listen_ip="169.254.229.31"
```

## Http Redirect to Https
If the defined network has enabled SSL, it can enable the redirection of an unencrypted network connection to an encrypted one. Omitting this field will disable redirection.
```toml
[networks.home]
http_redirect_port=80
```

## Config Information
For more information on the config see the spec and examples in the config folder.

## Incompatibilities
- Password authentication on Safari

## Manual install

You will need docker/podman, cargo, and the cargo crate named cross. There are other ways to cross compile for 32 bit arm as well.

`cross build --target armv7-unknown-linux-gnueabihf --release`

Then copy the binary 'target/armv7-unknown-linux-gnueabihf/release/webinterface-wifi' to the device and enable/start it as a systemd service.

## How Does it Work?

This program will start a reverse proxy on the wifi interface on the port specified. The proxy will start/stop based on if webinterface has the configured ip address and the wifi interface has an ip address. It will automatically be available whenever you connect to a new wifi network. 

![mobile_web_ui](https://github.com/rM-self-serve/webinterface-wifi/assets/122753594/981f3367-653e-40db-b389-703a046a4362)

