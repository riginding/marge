#[macro_use]
extern crate serde_json;
extern crate clap;
extern crate git2;
extern crate rand;
extern crate reqwest;

mod config;
mod error;
mod git;
mod gitlab;
mod macros;
mod subcommand;

use crate::config::Config;
use crate::error::MargeError;
use crate::gitlab::create_merge_request;
use console::Style;
use std::process;
use subcommand::parse_matches;

pub type Result<T> = std::result::Result<T, MargeError>;

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

fn run() -> Result<()> {
    let matches = parse_matches();

    if let Some(matches) = matches.subcommand_matches("merge") {
        if !Config::exists() {
            return Ok(marge_error!(
                "I found no config file please try: marge init"
            ));
        }

        let config = Config::read()?;
        if matches.is_present("suggest") {
            println!("suggest reviewer");
        } else {
            create_merge_request(config)?;
        }
    }

    if matches.subcommand_matches("init").is_some() {
        Config::init()?;
    }

    Ok(())
}
