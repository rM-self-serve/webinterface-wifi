pub const APP_NAME: &str = "webinterface-wifi";

pub const SERVCIE_FILE: &str = "/lib/systemd/system/webinterface-wifi.service";
pub const EXEC_REGEX: &str = r"/webinterface-wifi *--run *\d+";
pub const EXEC_START: &str = "/webinterface-wifi --run";

pub const WEB_INTERFACE: &str = "http://10.11.99.1:80";

pub const CLI_ABOUT: &str = r"
View the web interface over wifi, if running.
Requires the web interface to be accesible at 10.11.99.1:80.
webinterface-wifi will run by default on port 80.
Once running, type the wifi ip address into your browser to view the web interface.
";
