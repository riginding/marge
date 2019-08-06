#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde;
extern crate git2;
extern crate rand;
extern crate reqwest;

mod git;
mod gitlab;
mod subcomand;
mod error;

use structopt::StructOpt;
use subcomand::Marge;
use crate::error::MargeError;
use std::process;

fn main() {
    let result = run();

    match result {
        Ok(_) => {},
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }

    }

}

fn run() -> Result<(), MargeError> {
    match Marge::from_args() {
        Marge::Merge{} => {
            gitlab::create_merge_request()?;
        },
        _ => {
            println!("something else");
        }
    }

    Ok(())
}
