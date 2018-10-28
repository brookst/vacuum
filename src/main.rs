#![warn(unused_extern_crates)]
extern crate atty;
extern crate chrono;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate structopt;
extern crate yansi;

use structopt::StructOpt;
use yansi::Paint;

mod launch_library;
use launch_library::{Launches, BASE_URL};

const CREDITS: &str = "Credits:
https://launchlibrary.net/ - spaceflight database
https://github.com/Belar/space-cli - original node.js implementation";

#[derive(Debug, StructOpt)]
#[structopt(
    name = "vacuum",
    about = "A CLI for listing upcoming spaceflight events."
)]
struct Opt {
    #[structopt(short = "v", parse(from_occurrences))]
    verbose: u8,
    #[structopt(subcommand)]
    cmd: Cmd,
}

#[derive(Debug, StructOpt)]
enum Cmd {
    #[structopt(name = "about")]
    /// Program information
    About,
    #[structopt(name = "launch")]
    /// List launch events
    Launch {
        #[structopt(short = "n", default_value = "1")]
        /// Number of launch events to list
        number: u32,
        #[structopt(short = "v")]
        /// List more details for each event
        verbose: bool,
    },
}

fn main() {
    // Disable colour if output is not to a tty
    if atty::isnt(atty::Stream::Stdout) {
        Paint::disable();
    }

    let opt = Opt::from_args();
    if opt.verbose > 1 {
        println!("{:?}", opt);
    }
    match &opt.cmd {
        Cmd::About => {
            println!("{} - a spaceflight event CLI", Paint::green("Vacuum"));
            println!();
            println!("{}", CREDITS);
        }
        Cmd::Launch { number, verbose } => {
            let url = format!("{}/launch/next/{}", BASE_URL, number);
            if opt.verbose > 0 {
                println!("URL: {:?}", url);
            }
            let mut response = reqwest::get(&url).expect("Failed to lookup");

            let launches: Launches = response.json().expect("Failed to parse");

            for launch in launches {
                if *verbose {
                    println!("{:#}", launch)
                } else {
                    println!("{}", launch)
                }
            }
        }
    }
}
