use clap::{App as ClapApp, AppSettings, Arg};
use constants::{APP_NAME, CLI_ABOUT};
mod constants;
mod net_interface;
mod server;
mod set_port;

fn main() {
    let matches = ClapApp::new(APP_NAME)
        .setting(AppSettings::ArgRequiredElseHelp)
        .about(CLI_ABOUT)
        .arg(
            Arg::new("run")
                .long("run")
                .help("Starts webinterface-wifi in current shell with specified port")
                .takes_value(true)
                .value_name("port"),
        )
        .arg(
            Arg::new("set-port")
                .long("set-port")
                .help("Set the port webinterface-wifi will run on")
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
