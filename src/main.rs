#[macro_use]
extern crate serde_json;
extern crate clap;
extern crate git2;
extern crate rand;
extern crate reqwest;

mod error;
mod git;
mod gitlab;
mod init;
mod subcommand;

use crate::error::MargeError;
use std::process;
use subcommand::parse_matches;

fn main() {
    let result = run();

    match result {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    }
}

fn run() -> Result<(), MargeError> {
    let matches = parse_matches();

    if let Some(matches) = matches.subcommand_matches("merge") {
        if matches.is_present("suggest") {
            println!("suggest reviewer");
        } else {
            println!("assign a random reviewer");
        }
    }

    if matches.subcommand_matches("init").is_some() {
        init::find_or_create()?
    }

    Ok(())
}
