use structopt::StructOpt;

pub const CONSOLE_TEMPLATE: &str = r#"
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

/// DOC COMMENT
#[derive(StructOpt, Debug)]
#[structopt(
    about="marge, your friendly merge assistant",
    raw(template="CONSOLE_TEMPLATE")
)]
pub enum Marge {
    #[structopt(
        name="setup",
        about="Initialize your configuration",
        raw(template="CONSOLE_TEMPLATE")
    )]
    Setup { },
    #[structopt(name = "buddy",
                about="Configure your merge buddies",
                raw(template="CONSOLE_TEMPLATE")
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
                raw(template="CONSOLE_TEMPLATE")
    )]

    Merge { }
}