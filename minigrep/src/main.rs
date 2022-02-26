#![allow(unused)]

use std::error::Error;
use std::{env, fs, process};

use minigrep::hello::srv;
use minigrep::Config;

fn main() {
    // srv();
    run();
}

fn run() {
    let args: Vec<String> = env::args().collect();
    // cargo run needle haystack
    // ["target/debug/minigrep", "needle", "haystack"]
    println!("{:?}", args);

    // under the folder of current project
    // let (query, filename) = parse_config(&args);
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Error parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
        process::exit(-1);
    }
}

fn parse_config_obj(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    Config {
        query,
        filename,
        case_sensitive: false,
    }
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];

    (query, filename)
}
