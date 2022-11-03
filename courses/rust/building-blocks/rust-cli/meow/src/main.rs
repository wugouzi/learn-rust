// use std::env;
extern crate clap;
// use clap::App;
// use clap;
// use clap::App;

use clap::{arg, command, value_parser, ArgAction, Command};

fn main() {
    let m = App::new("My Program")
    .author("Me, me@mail.com")
    .version("1.0.2")
    .about("Explains in brief what the program does")
    .arg(
        Arg::with_name("in_file").index(1)
    )
    .after_help("Longer explanation to appear after the options when \
                 displaying the help information from --help or -h")
    .get_matches();

    // let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);
}
