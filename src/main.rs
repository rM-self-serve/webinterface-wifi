use clap::{App as ClapApp, AppSettings, Arg};
use constants::APP_NAME;
mod constants;
mod net_interface;
mod server;
mod set_port;

fn main() {
    let matches = ClapApp::new(APP_NAME)
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(
            Arg::new("run")
                .long("run")
                .help("Starts webinterface-wifi in shell with specified port")
                .takes_value(true)
                .value_name("port"),
        )
        .arg(
            Arg::new("set-port")
                .long("set-port")
                .help("Configure the default port to expose on the wifi interface")
                .takes_value(true)
                .value_name("port"),
        )
        .get_matches();

    if matches.is_present("run") {
        let port = matches.value_of("run").unwrap();
        let _ = server::run_loop(port);
    } else if matches.is_present("set-port") {
        let port = matches.value_of("set-port").unwrap();
        set_port::set_port(port);
    }
}
