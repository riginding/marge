#[macro_use]
extern crate serde_json;

extern crate git2;
extern crate rand;
extern crate reqwest;

mod gitlab;
mod subcommand;
mod error;

use std::error::Error;
use structopt::StructOpt;
use subcommand::Marge;

fn main() -> Result<(), Box<dyn Error>> {
    //    match Marge::from_args() {
    //        Marge::Merge{} => {
    //            gitlab::create_merge_request()?;
    //        },
    //        _ => {
    //            println!("something else");
    //        }
    //    }

    println!("{:?}", gitlab::git_path());

    Ok(())
}
