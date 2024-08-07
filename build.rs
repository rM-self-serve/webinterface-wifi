/*
Building by default will compile a program which stores/looks for data in:
- /home/root/.local/share/webinterface-wifi
And stores/looks for the config in:
- /home/root/.config/webinterface-wifi


Building with the enviroment variable WIW_DATADIR="/opt/etc" will compile
a program which stores/looks for data in:
- /opt/etc/webinterface-wifi
Building with the enviroment variable WIW_CONFDIR="/etc" will compile
a program which stores/looks for data in:
- /etc/webinterface-wifi
*/

use std::env::var;
use std::str;

const CARGOENV: &str = "cargo:rustc-env=";
const PACKAGE_NAME: &str = "webinterface-wifi";

const LOCALDATADIR: &str = "/home/root/.local/share";
const LOCALCONFDIR: &str = "/home/root/.config";

macro_rules! to_build_env {
    ($($destdir:ident: $envname:tt: $fpath:tt$(,)?)*) => {
        $(println!("{}",
            format!(
                "{}{}=/{}/{}/{}",
                CARGOENV,
                $envname,
                $destdir,
                PACKAGE_NAME,
                $fpath
            ).replace("//", "/")
        );)*
    };
}

fn main() {
    let data_dir = var("WIW_DATADIR").unwrap_or(LOCALDATADIR.to_string());
    let conf_dir = var("WIW_CONFDIR").unwrap_or(LOCALCONFDIR.to_string());

    to_build_env!(
        data_dir: "DEF_PASS_PATH": "auth/login.pass",
        data_dir: "DEF_SSL_CERT_PATH": "ssl/ssl_cert.pem",
        data_dir: "DEF_SSL_PRIV_PATH": "ssl/ssl_priv.rsa",
        data_dir: "FAVICON_PATH": "assets/favicon.ico",
        conf_dir: "DEF_CNFG_PATH": "config.toml",
    );
}
