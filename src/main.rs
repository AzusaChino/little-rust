use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};

use crate::exercise::{Exercise, ExerciseList};

#[macro_use]
mod ui;
mod demo;

mod exercise;

const VERSION: &str = "4.6.0";

#[derive(FromArgs, PartialEq, Debug)]
struct Args {
    #[argh(switch)]
    no_capture: bool,
    #[argh(switch, short = 'v')]
    version: bool,
    #[argh(subcommand)]
    nested: Option<Subcommands>,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum Subcommands {
    Verify(VerifyArgs),
    Watch(WatchArgs),
    Run(RunArgs),
    Hint(HintArgs),
    List(ListArgs),
}

fn main() {
    let args: Args = argh::from_env();
    if args.version {
        println!("v{}", VERSION);
        std::process::exit(0);
    }
    if args.nested.is_none() {
        println!();
        println!(r#"       welcome to...                      "#);
        println!(r#"                 _   _ _                  "#);
        println!(r#"  _ __ _   _ ___| |_| (_)_ __   __ _ ___  "#);
        println!(r#" | '__| | | / __| __| | | '_ \ / _` / __| "#);
        println!(r#" | |  | |_| \__ \ |_| | | | | | (_| \__ \ "#);
        println!(r#" |_|   \__,_|___/\__|_|_|_| |_|\__, |___/ "#);
        println!(r#"                               |___/      "#);
        println!();
    }

    if !Path::new("info.toml").exists() {
        println!(
            "{} must be run from the main directory",
            std::env::current_exe().unwrap().to_str().unwrap()
        );
        println!("try `cd little-rust/`!");
        std::process::exit(1);
    }

    if !rustc_exists() {
        println!("We cannot find `rustc`.");
        println!("Try running `rustc --version` to diagnose your problem.");
        println!("For instructions on how to install Rust, check the README.");
        std::process::exit(1);
    }

    let toml_str = &fs::read_to_string("info.toml").unwrap();
    let _exercises = toml::from_str::<ExerciseList>(toml_str).unwrap().exercises;
    let _verbose = args.no_capture;

    let command = args.nested.unwrap_or_else(|| {
        let text = fs::read_to_string("default_out.txt").unwrap();
        println!("{}", text);
        std::process::exit(0);
    });

    match command {
        Subcommands::List(_sub_args) => {}
        _ => {}
    }
}

fn rustc_exists() -> bool {
    Command::new("rustc")
        .args(&["--version"])
        .stdout(Stdio::null())
        .spawn()
        .and_then(|mut child| child.wait())
        .map(|status| status.success())
        .unwrap_or(false)
}