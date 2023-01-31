use crate::constants::{APP_NAME, EXEC_REGEX, EXEC_START, SERVCIE_FILE};
use regex::Regex;
use std::{
    fs::{read_to_string, write},
    process::Command,
};

pub fn set_port(port: &str) {
    // Use regex to change the port number in the ExecStart line
    // of the webinterface-wifi.service file
    // Ex: ExecStart=/home/root/.local/bin/webinterface-wifi --run 80
    let service_file =
        read_to_string(SERVCIE_FILE).expect(&format!("Unable to read {SERVCIE_FILE}"));
    let re = Regex::new(EXEC_REGEX).unwrap();
    if !validate_port(port) || !vaidate_service_file(&re, &service_file) {
        return;
    }
    let new = &format!("{} {}", EXEC_START, port);
    let reout = re.replace(&service_file, new).to_string();
    write(SERVCIE_FILE, reout).expect(&format!("Unable to write {SERVCIE_FILE}"));
    let daemon_out = Command::new("systemctl").args(["daemon-reload"]).output();
    match daemon_out {
        Ok(_) => {}
        Err(e) => println!("Error reloading daemon: {e}"),
    }
    let restart_out = Command::new("systemctl")
        .args(["restart", APP_NAME])
        .output();
    match restart_out {
        Ok(_) => println!("Starting on port: {port}"),
        Err(e) => println!("Error restarting service: {e}"),
    }
}

fn vaidate_service_file(re: &Regex, file_txt: &str) -> bool {
    if !re.is_match(file_txt) {
        println!("Can't find '{EXEC_START} <port>' in {SERVCIE_FILE}");
        return false;
    }
    true
}

fn validate_port(port: &str) -> bool {
    let port_int_op = port.parse::<u32>();
    if port_int_op.is_err() {
        println!("Error: Port must be valid number");
        return false;
    }
    let port_int = port_int_op.unwrap();
    if port_int != 80 && (port_int < 1024 || port_int > 65535) {
        println!(
            r"Error: Port number must be 80 or between 1024-65535
        You can manually change {SERVCIE_FILE} if necessary.
        "
        );
        return false;
    }
    true
}
