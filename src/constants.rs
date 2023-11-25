pub const DEF_CNFG_PATH: &str = "/home/root/.config/webinterface-wifi/config.toml";
pub const DEF_PASS_PATH: &str = "/home/root/.local/share/webinterface-wifi/auth/login.pass";
pub const DEF_SSL_CERT_PATH: &str = "/home/root/.local/share/webinterface-wifi/ssl/ssl_cert.pem";
pub const DEF_SSL_PRIV_PATH: &str = "/home/root/.local/share/webinterface-wifi/ssl/ssl_priv.rsa";
pub const DEF_WIFI_INT: &str = "wlan0";
pub const DEF_WEBINT_IP: &str = "10.11.99.1";
pub const DEF_WEBINT_PORT: u16 = 80;
pub const DEF_EDITOR: &str = "nano";
pub const DEF_LOG_LEVEL: &str = "warn";

pub const DEF_CNTRL_PORT: u16 = 6396;
pub const RUNTIME_ENV: &str = "WEBINT_WIFI_RUN_ENV";
pub const RUNTIME_ENV_DAEMON: &str = "DAEMON";
pub const RUNTIME_LOG_LVL: &str = "WEBINT_WIFI_LOGLVL";
pub const _INTERNAL_LOG_NAME: &str = "webinterface_wifi";
pub const CMD_ENV: &str = "/usr/bin/env";
pub const TCP_BUFFER_SIZE: usize = 4096;

pub const AUTH_REALM: &str = "login";
pub const FAVICON_PATH: &str = "/home/root/.local/share/webinterface-wifi/assets/favicon.ico";

pub const SIGINT: i32 = 2;
pub const SIGUSR1: i32 = 10;
pub const SIGTERM: i32 = 15;

pub const CLI_ABOUT: &str = r"
View the web interface over wifi.
Requires the web interface to be accesible at 10.11.99.1:80.
Once running, type the wifi ip address into your browser to view the web interface.
Source+Docs: https://github.com/rM-self-serve/webinterface-wifi

Enable/Use:
$ systemctl enable --now webinterface-onboot

Disable:
systemctl disable --now webinterface-onboot";
