use clap::{App as ClapApp, AppSettings, Arg};
mod net_interface;
mod server;
mod set_port;

fn main() {
    let matches = ClapApp::new("webinterface-wifi")
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(
            Arg::new("run")
                .long("run")
                .help("Starts webinterface-wifi in shell with specified port")
                .takes_value(true)
        )
        .arg(
            Arg::new("set-port")
                .long("set-port")
                .help("Configure the default port to expose on the wifi interface")
                .takes_value(true)
        )
        .get_matches();

    if matches.is_present("run") {
        let port = matches.value_of("run").unwrap();
        let _ = server::run_loop(port);
        return;
    }
    if matches.is_present("set-port") {
        let port = matches.value_of("set-port").unwrap();
        set_port::set_port(port);
    }
}
