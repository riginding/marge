#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde;
extern crate git2;
extern crate rand;
extern crate reqwest;

mod git;
mod gitlab;
mod subcomand;
mod error;

use std::error::Error;
use structopt::StructOpt;
use subcomand::Marge;

fn main() -> Result<(), Box<dyn Error>> {
       match Marge::from_args() {
           Marge::Merge{} => {
               gitlab::create_merge_request()?;
           },
           _ => {
               println!("something else");
           }
       }

    println!("{:?}", git::git_path());
    println!("TEST");

    Ok(())
}
