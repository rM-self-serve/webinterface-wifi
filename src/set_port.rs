use regex::Regex;
use std::{
    fs::{read_to_string, write},
    process::Command,
};

const SERVCIE_FILE: &str = "/lib/systemd/system/webinterface-wifi.service";
const EXEC_START: &str = "ExecStart=/usr/bin/webinterface-wifi --run";

pub fn set_port(port: &str) {
    // Use regex to change the port number in the ExecStart line
    // of the webinterface-wifi.service file
    // Ex: ExecStart=/usr/bin/webinterface-wifi --run 80
    validate_port(port);
    let service_file =
        read_to_string(SERVCIE_FILE).expect(&format!("Unable to read {}", SERVCIE_FILE));
    let re = Regex::new(&format!(r"{} \d+", EXEC_START)).unwrap();
    let new = &format!("{} {}", EXEC_START, port);
    let reout = re.replace(&service_file, new).to_string();
    write(SERVCIE_FILE, reout).expect(&format!("Unable to write {}", SERVCIE_FILE));
    let out = Command::new("systemctl").args(["daemon-reload"]).output();
    match out {
        Ok(_) => {}
        Err(e) => println!("Error reloading daemon: {e}"),
    }
    let out = Command::new("systemctl")
        .args(["restart", "webinterface-wifi.service"])
        .output();
    match out {
        Ok(_) => {}
        Err(e) => println!("Error restarting service: {e}"),
    }
}

fn validate_port(port: &str) {
    let port_int_op = port.parse::<u32>();
    if port_int_op.is_err() {
        panic!("Error: Port must be valid number")
    }
    let port_int = port_int_op.unwrap();
    if port_int != 80 && (port_int < 1024 || port_int > 65535) {
        panic!("Error: Port number must be 80 or between 1024-65535")
    }
}
