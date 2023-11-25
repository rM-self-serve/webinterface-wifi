mod auth;
mod cmdline;
mod config;
mod constants;
mod netinfo;
mod server;

use log::debug;
use std::process::ExitCode;

fn main() -> ExitCode {
    if let Err(err) = cmdline::process::run() {
        debug!("{err}");
        return ExitCode::FAILURE;
    };

    ExitCode::SUCCESS
}
