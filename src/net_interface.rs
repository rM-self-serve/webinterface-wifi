use local_ip_address::list_afinet_netifas;
use std::net::IpAddr;

#[derive(Debug, Clone)]
pub struct RMipv4s {
    pub usb0: Option<IpAddr>,
    pub wlan0: Option<IpAddr>,
}

impl RMipv4s {
    pub fn new() -> RMipv4s {
        let mut export = RMipv4s {
            usb0: None,
            wlan0: None,
        };
        let network_interfaces = list_afinet_netifas().unwrap();
        for (name, ip) in network_interfaces {
            if name == "usb0" && ip.is_ipv4() {
                export.usb0 = Some(ip)
            }
            if name == "wlan0" && ip.is_ipv4() {
                export.wlan0 = Some(ip)
            }
        }
        export
    }
}
