use crate::constants::AUTH_REALM;
use colored::Colorize;
use log::error;
use rpassword::read_password;
use sha256::digest;
use std::{
    fs::File,
    io::{prelude::*, Error, ErrorKind, Write},
    path::{Path, PathBuf},
};

pub fn create(path: &PathBuf) -> std::io::Result<()> {
    if Path::new(path).exists() {
        println!(
            "{}{}",
            "Login file already exists at: ".red(),
            path.display()
        );
        print!("Overwrite user/password? (N/y): ");
        std::io::stdout().flush()?;
        let mut ovrwrt = String::new();
        std::io::stdin().read_line(&mut ovrwrt)?;
        ovrwrt = ovrwrt.replace("\n", "");
        if ovrwrt.to_lowercase() != "y" {
            return Ok(());
        }
    }

    let mut user = String::new();
    print!("User: ");
    std::io::stdout().flush()?;
    std::io::stdin().read_line(&mut user)?;
    user = user.replace("\n", "");

    // due to the way the authentication header is parsed
    if user.contains(",") {
        error!("Usernames can only contain letters and numbers");
        return Ok(());
    }

    print!("Password: ");
    std::io::stdout().flush()?;
    let pass1 = read_password()?;

    print!("Retype Password: ");
    std::io::stdout().flush()?;
    let pass2 = read_password()?;

    if pass1 != pass2 {
        println!("Passwords do not match, try again.");
        return Ok(());
    }

    let user_hash = digest(format!("{}:{}:{}", user, AUTH_REALM, pass1));

    if let Err(err) = write_login(path, user_hash) {
        error!("Failed to create login file {}", err);
        return Err(err);
    };
    println!("Login file created at: {}", path.display());
    Ok(())
}

fn write_login(path: &PathBuf, user_hash: String) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(user_hash.as_bytes())?;
    Ok(())
}

pub fn load_login(path: &str) -> Result<String, Error> {
    let mut login_file =
        File::open(path).map_err(|e| error(format!("Failed to open {}: {}", path, e)))?;
    let mut contents = String::new();
    login_file
        .read_to_string(&mut contents)
        .map_err(|e| error(format!("Failed to open {}: {}", path, e)))?;

    Ok(contents)
}

fn error(err: String) -> Error {
    Error::new(ErrorKind::Other, err)
}
