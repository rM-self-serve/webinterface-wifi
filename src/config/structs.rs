// Since almost every field of the config can be omitted,
// parse everything as Options

use crate::constants::{DEF_WEBINT_IP, DEF_WEBINT_PORT, DEF_WIFI_INT};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt};

macro_rules! opt_struct_convert {
    // make every field in the struct an Option<type>
    // make every field and the struct public
    // make the struct #[derive(Debug, Clone, Serialize, Deserialize)]
    () => {};
    ($struct:ident {
        $($field:ident:$type:ty$(,)?)*
    }) => {
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct $struct {
            $(pub $field: Option<$type>),*
            }
    };
}

macro_rules! opt_struct_add {
    // create a struct with the name of the first identifier (Ex: Device)
    //   where every field has the type listed (Require: x) or Option<type> (Option: x)
    // create a struct with the name of the second identifier (Ex: DeviceOPT)
    //   where every field is an Option<type>
    // make every field and the structs public
    // make the structs #[derive(Debug, Clone, Serialize, Deserialize)]
    () => {};
    (Require:$type:ty) => {$type};
    (Option:$type:ty) => {Option<$type>};
    ($struct:ident {
        $($is_opt:ident:$field:ident:$type:ty$(,)?)*
    }, $optstruct:ident) => {
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct $struct {
            $(pub $field: opt_struct_add!($is_opt:$type)),*
            }
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct $optstruct {
             $(pub $field: Option<$type>),*
            }
    };
}

opt_struct_convert! {
    ConfigCore {
        conf: ConfigContOPT,
        networks: HashMap<String, Network>,
        undefined_networks: Network,
        blocklist: ABList,
        allowlist: ABList,
        device: DeviceOPT,
    }
}

opt_struct_add! {
    ConfigCont {
        Require: network_filter: String,
        Option: ssl_cert_path: String,
        Option: ssl_priv_path: String,
        Option: login_path: String,
        Option: daemon_port: u16,
    }, ConfigContOPT
}

opt_struct_convert! {
    Network {
        __name: String,
        router_ssid: String,
        login_enforced: bool,
        ssl: bool,
        listen_ip: String,
        listen_port: u16,
        http_redirect_port: u16,
    }
}

impl fmt::Display for Network {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut fnl_str = "".to_string();

        if let Some(x) = &self.__name {
            let _ = write!(f, "{}\n", x.bright_blue());
        }

        if let Some(x) = &self.router_ssid {
            fnl_str += &format!("router_ssid: {x}\n");
        }
        if let Some(x) = self.login_enforced {
            fnl_str += &format!("login_enforced: {x}\n");
        }
        if let Some(x) = self.ssl {
            fnl_str += &format!("ssl: {x}\n");
        }
        if let Some(x) = &self.listen_ip {
            fnl_str += &format!("listen_ip: {x}\n");
        }
        if let Some(x) = self.listen_port {
            fnl_str += &format!("listen_port: {x}\n");
        }
        if let Some(x) = self.http_redirect_port {
            fnl_str += &format!("http_redirect_port: {x}");
        }

        write!(f, "{fnl_str}")
    }
}

opt_struct_convert! {
    ABList {
        networks: Vec<String>
    }
}

opt_struct_add! {
    Device {
        Require: webint_port: u16,
        Require: webint_ip: String,
        Require: wifi_interface: String,
    }, DeviceOPT
}

impl Default for Device {
    fn default() -> Self {
        Device {
            webint_port: DEF_WEBINT_PORT,
            webint_ip: DEF_WEBINT_IP.to_string(),
            wifi_interface: DEF_WIFI_INT.to_string(),
        }
    }
}
