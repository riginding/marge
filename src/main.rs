#[macro_use]
extern crate serde_json;

extern crate reqwest;
extern crate rand;
extern crate git2;

mod subcommand;
mod gitlab;

use std::error::Error;
use subcommand::Marge;
use structopt::StructOpt;


fn main() -> Result<(), Box<dyn Error>> {
//    match Marge::from_args() {
//        Marge::Merge{} => {
//            gitlab::create_merge_request()?;
//        },
//        _ => {
//            println!("something else");
//        }
//    }

    println!("{:?}",gitlab::git_path());

    Ok(())
}
