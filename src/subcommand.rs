use clap::{App, Arg, ArgMatches, SubCommand};

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

pub fn parse_matches() -> ArgMatches<'static> {
    App::new("marge, your friendly merge assistant")
        .version("1.0")
        .author("Ryan Riginding<ryan@riginding.com>")
        .template(CONSOLE_TEMPLATE)
        .subcommand(
            SubCommand::with_name("merge")
                .about("create a merge request")
                .arg(
                    Arg::with_name("suggest")
                        .short("s")
                        .help("assign a random reviewer"),
                ),
        )
        .subcommand(SubCommand::with_name("init").about("Initialize your configuration"))
        .get_matches()
}
