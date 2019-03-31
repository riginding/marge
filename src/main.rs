#[macro_use]
extern crate structopt;
#[macro_use]
extern crate serde_json;

extern crate reqwest;
extern crate rand;

use reqwest::header::{HeaderMap, HeaderValue};
use structopt::StructOpt;
use std::process::Command;
use rand::seq::SliceRandom;



const HELP_TEMPLATE: &str = r#"
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

{all-args}
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
enum Marge {
    #[structopt(
        name="setup",
        about="Initialize your configuration",
        raw(template="HELP_WITHOUT_SUB_TEMPLATE")
    )]
    Setup { },
    #[structopt(name = "buddy",
                about="Configure your merge buddies",
                raw(template="HELP_TEMPLATE")
    )]
    Buddy {
        /// add a merge buddy
        #[structopt(short = "a")]
        add: bool,
        /// list your merge buddies
        #[structopt(short = "l")]
        list: bool,
    },
    #[structopt(name = "merge",
                about="Creates a merge request",
                raw(template="HELP_WITHOUT_SUB_TEMPLATE")
    )]
    Merge { }
}

fn construct_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("Private-Token", HeaderValue::from_static("wJ7Xg4gWAB4zenVMvysc"));
    headers
}

fn create_merge_request() -> Result<(), reqwest::Error> {
    let params = json!({
        "id": "506", // dreamfactory hardcoded
        "source_branch": active_branch(),
        "target_branch": "sandbox",
        "assignee_id": chose_assignee(),
        "title": "This is a test",
        "description": "a description for this very useful mr",
        "remove_source_branch": true,
        "squash": true,
    });


    let client = reqwest::Client::new();
    let res = client.post("https://git.sclable.com/api/v4/projects/506/merge_requests")
        .headers(construct_headers())
        .json(&params)
        .send()?;

    println!("{:?}", params);

    Ok(())
}

fn main() {
    match Marge::from_args() {
        Marge::Merge{} => {
            create_merge_request();
        },
        _ => {
            println!("something else");
        }
    }

}

fn chose_assignee() -> i32 {
    // Adam, Manfred, Anzor
    let commitor_ids = [81, 69, 60];
    *commitor_ids.choose(&mut rand::thread_rng()).unwrap()
}

fn active_branch() -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg("git symbolic-ref --short HEAD")
        .output()
        .expect("failed to execute process");

    let hello = output.stdout;
    std::str::from_utf8(&hello).unwrap().trim().to_string()
}

// how to find out which branch we are in:
// git symbolic-ref --short HEAD