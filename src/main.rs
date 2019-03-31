#[macro_use]
extern crate structopt;
#[macro_use]
extern crate serde_json;

extern crate reqwest;
extern crate rand;


use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT, CONTENT_TYPE};
use structopt::StructOpt;
use std::process::Command;
use std::collections::HashMap;
use rand::seq::SliceRandom;



const HELP_TEMPLATE: &str = r#"
        ,
        (                          )use structopt::StructOpt;
use std::process::Command;

         \                        /
        ,' ,__,___,__,-._         )
        )-' ,    ,  , , (        /
        ;'"-^-.,-''"""\' \       )
       (      (        ) /  __  /
        \o,----.  o  _,'( ,.^. \
        ,'`.__  `---'    `\ \ \ \_
 ,.,. ,'                   \    ' )
 \ \ \\__  ,------------.  /     /
( \ \ \( `---.-`-^--,-,--\:     :
 \       (   (""""""`----'|     :
  \   `.  \   `.          |      \
   \   ;  ;     )      __ _\      \
   /     /    ,-.,-.'"Y  Y  \      `.
  /     :    ,`-'`-'`-'`-'`-'\       `.
 /      ;  ,'  /              \        `
/      / ,'   /                \

{about}

USAGE:
    {usage}

subcommands:
{subcommands}
"#;

const HELP_WITHOUT_SUB_TEMPLATE: &str = r#"
        ,
        (                          )
         \                        /
        ,' ,__,___,__,-._         )
        )-' ,    ,  , , (        /
        ;'"-^-.,-''"""\' \       )
       (      (        ) /  __  /
        \o,----.  o  _,'( ,.^. \
        ,'`.__  `---'    `\ \ \ \_
 ,.,. ,'                   \    ' )
 \ \ \\__  ,------------.  /     /
( \ \ \( `---.-`-^--,-,--\:     :
 \       (   (""""""`----'|     :
  \   `.  \   `.          |      \
   \   ;  ;     )      __ _\      \
   /     /    ,-.,-.'"Y  Y  \      `.
  /     :    ,`-'`-'`-'`-'`-'\       `.
 /      ;  ,'  /              \        `
/      / ,'   /                \

{about}

USAGE:
    {usage}
"#;

/// DOC COMMENT
#[derive(StructOpt, Debug)]
#[structopt(
    about="marge, your friendly merge assistant",
    raw(template="HELP_TEMPLATE")
)]
enum Opt {
    #[structopt(
        name="setup",
        about="Initialize your configuration",
        raw(template="HELP_WITHOUT_SUB_TEMPLATE")
    )]
    Setup {
      setup: bool,
    },
    #[structopt(name = "buddy",
                about="Configure your merge buddies",
                raw(template="HELP_TEMPLATE")
    )]
    Buddy {
        /// add a merge buddy
        #[structopt(name = "add")]
        add: bool,
        /// list your merge buddies
        #[structopt(name = "list")]
        list: bool,
    },
    #[structopt(name = "merge",
                about="Creates a merge request",
                raw(template="HELP_WITHOUT_SUB_TEMPLATE")
    )]
    Merge {
        merge: bool,
    }
}

fn construct_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("Private-Token", HeaderValue::from_static("wJ7Xg4gWAB4zenVMvysc"));
    headers
}

fn main() -> Result<(), reqwest::Error> {
    // let output = Command::new("sh")
    //     .arg("-c")
    //     .arg("git symbolic-ref --short HEAD")
    //     .output()
    //     .expect("failed to execute process");

    // let hello = output.stdout;
    // println!("You are in this branch now: {:?}", std::str::from_utf8(&hello).unwrap().trim());
    // let matches = Opt::from_args();

    // println!("{:?}", matches);

    // Adam, Manfred, Anzor
    let commitor_ids = [81, 69, 60];
    let chosen = commitor_ids.choose(&mut rand::thread_rng());
    println!("{:?}", chosen);

    let params = json!({
        "id": "506",
        "source_branch": "marge_test",
        "target_branch": "sandbox",
        "assignee_id": chosen.unwrap(),
        "title": "This is a test",
        "description": "a description for this very useful mr",
        "remove_source_branch": true,
        "squash": true,
    });


    let client = reqwest::Client::new();
    let res = client.post("https://git.sclable.com/api/v4/projects/506/merge_requests")
        .headers(construct_headers())
        .json(&params)
        .send();

    println!("{:?}", res);

    Ok(())
}

// how to find out which branch we are in:
// git symbolic-ref --short HEAD
