#[macro_use]
extern crate structopt;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "marge")]
/// marge, the friendly merge assistant
enum Opt {
    /// setup marge configuration
    #[structopt(name = "setup")]
    Setup {
      setup: bool,
    },
    #[structopt(name = "buddy")]
    /// configure your buddies
    Buddy {
        /// add a merge buddy
        #[structopt(name = "add")]
        add: bool,
        /// list your merge buddies
        #[structopt(name = "list")]
        list: bool,
    },
}

fn main() {
    let matches = Opt::from_args();

    println!("{:?}", matches);
}
