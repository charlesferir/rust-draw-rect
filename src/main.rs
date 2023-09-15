use clap::{arg, Command};
use log::error;
use simple_logger::SimpleLogger;
use std::{fs::OpenOptions, io::Write};

fn cli() -> Command {
    Command::new(env!("CARGO_PKG_NAME"))
        .author("Charles Ferir, charlesferir1@gmail.com")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Draw rectangles in a file.")
        .arg(arg!(<OUT_FILE> "File in which rectangles should be drawn"))
}

fn main() -> Result<(), std::io::Error> {
    SimpleLogger::new().init().unwrap();

    let matches = cli().get_matches();
    let out_file = matches.get_one::<String>("OUT_FILE").expect("required");

    let mut file = match OpenOptions::new().write(true).create(true).open(out_file) {
        Ok(file) => file,
        Err(e) => {
            error!("Could not open file {out_file}");
            return Err(e);
        }
    };

    file.write_all("Hello, world!".as_bytes())?;
    Ok(())
}
