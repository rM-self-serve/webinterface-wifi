pub const APP_NAME: &str = "webinterface-wifi";

pub const SERVCIE_FILE: &str = "/lib/systemd/system/webinterface-wifi.service";
pub const EXEC_REGEX: &str = r"/webinterface-wifi *--run *\d+";
pub const EXEC_START: &str = "/webinterface-wifi --run";

pub const WEB_INTERFACE: &str = "http://10.11.99.1:80";
