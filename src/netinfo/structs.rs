use super::ssid_cmd;
use crate::config::factory::Config;
use colored::Colorize;
use local_ip_address::list_afinet_netifas;
use log::debug;
use std::io::{Error, ErrorKind};
use std::net::IpAddr;
use std::time::Duration;
use tokio::time::sleep;

#[derive(Debug, Clone, Hash)]
pub struct NetInfo {
    pub webint_ntrfc: IpAddr,
    pub wifi_ntrfc: IpAddr,
    pub ssid: String,
}

impl NetInfo {
    pub async fn from_config(config: &Config) -> std::io::Result<Option<NetInfo>> {
        let device = config.device.to_owned();
        let info = tokio::spawn(async move {
            NetInfo::from_sync(&device.wifi_interface, &device.webint_interface, false)
        });

        tokio::select! {
            val = info => val?,
            _ = sleep(Duration::from_millis(1000)) =>
                Err(Error::new(ErrorKind::Other, "Timed out while gathering network info.".to_string()))
        }
    }

    pub fn from_sync(
        wifi_interface: &str,
        webint_interface: &str,
        is_cli: bool,
    ) -> std::io::Result<Option<NetInfo>> {
        let mut webint_ntrfc: Option<IpAddr> = None;
        let mut wifi_ntrfc: Option<IpAddr> = None;

        let network_interfaces =
            list_afinet_netifas().map_err(|err| Error::new(ErrorKind::Other, err))?;
        for (name, ip) in network_interfaces {
            if &name == wifi_interface {
                if ip.is_ipv4() {
                    wifi_ntrfc = Some(ip)
                }
            }
            if &name == webint_interface {
                if ip.is_ipv4() {
                    webint_ntrfc = Some(ip)
                }
            }
        }

        let mut success = true;
        match wifi_ntrfc.as_ref() {
            Some(val) => print_or_dbug(
                &format!(
                    "wifi interface: {} ip: {}",
                    wifi_interface.bright_blue(),
                    val.to_string().bright_blue()
                ),
                is_cli,
            ),
            None => {
                print_or_dbug(
                    &format!(
                        "Could not find ip of wifi interface: {}",
                        wifi_interface.red()
                    ),
                    is_cli,
                );
                success = false;
            }
        }
        match webint_ntrfc.as_ref() {
            Some(val) => print_or_dbug(
                &format!(
                    "webint interface: {} ip: {}",
                    webint_interface.bright_blue(),
                    val.to_string().bright_blue()
                ),
                is_cli,
            ),
            None => {
                print_or_dbug(
                    &format!(
                        "Could not find ip of webint interface: {}",
                        webint_interface.red()
                    ),
                    is_cli,
                );
                success = false;
            }
        }

        let ssid_opt = ssid_cmd::get_ssid(wifi_interface)?;
        let Some(ssid) = ssid_opt.as_ref() else {
            print_or_dbug("Could not find router ssid", is_cli);
            return Ok(None);
        };
        print_or_dbug(&format!("router ssid: {}", ssid.bright_blue()), is_cli);

        if !success {
            return Ok(None);
        }

        Ok(Some(NetInfo {
            webint_ntrfc: webint_ntrfc.unwrap(),
            wifi_ntrfc: wifi_ntrfc.unwrap(),
            ssid: ssid.to_owned(),
        }))
    }
}

fn print_or_dbug(output: &str, is_cli: bool) {
    if is_cli {
        println!("{}", output);
    } else {
        debug!("{}", output);
    }
}
