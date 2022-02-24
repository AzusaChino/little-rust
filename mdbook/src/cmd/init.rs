use anyhow::Result;
use clap::{App, Arg, ArgMatches, SubCommand};
use std::io;
use std::process::Command;

/// create clap subcommand arguments
pub fn make_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("init")
        .about("Create the boilerplate structure and files for a new book")
        .arg_from_usage(
            "
    [dir] 'Directory to create the book in {n}\
    (Defaults to the current directory when omitted)'",
        )
        .arg_from_usage("--theme 'Copies the default theme into your source folder'")
        .arg_from_usage("--force 'Skips confirmation prompts'")
        .arg(
            Arg::with_name("title")
                .long("title")
                .takes_value(true)
                .help("sets the book title")
                .required(false),
        )
        .arg(
            Arg::with_name("ignore")
                .long("ignore")
                .takes_value(true)
                .possible_values(&["none", "git"])
                .help("Creates a VCS ignore file (i.e. .gitignore)")
                .required(false),
        )
}

/// Init command implementation
pub fn execute(args: &ArgMatches) -> Result<()> {
    let book_dir = "";
    let mut builder = String::new();
    let mut config = None;
    if args.is_present("theme") {}

    if let Some(ignore) = args.value_of("ignore") {
        match ignore {
            "git" => println!(""),
            _ => println!(""),
        };
    } else {
        println!("\n Do you want a .gitignore to be created? (y/n)");
        if confirm() {
            // create gitignore
        }
    }
    config = if args.is_present("title") {
        args.value_of("title").map(String::from)
    } else {
        request_book_title()
    };

    if let Some(author) = get_author_name() {
        println!("Obtained user name from gitconfig: {:?}", author);
        // config.book.authors.push(author);
        // builder.with_config(config);
    }

    // builder.build()?;
    println!("\nAll done, no errors...");

    Ok(())
}

fn get_author_name() -> Option<String> {
    let output = Command::new("git")
        .args(&["config", "--get", "user.name"])
        .output()
        .ok()?;
    if output.status.success() {
        Some(String::from_utf8_lossy(&output.stdout).trim().to_owned())
    } else {
        None
    }
}

fn request_book_title() -> Option<String> {
    println!("What title would you like to give the book?");
    io::stdout().flush().unwrap();
    let mut resp = String::new();
    io::stdin().read_line(&mut resp).unwrap();
    let resp = resp.trim();
    if resp.is_empty() {
        None
    } else {
        Some(resp.into())
    }
}

fn confirm() -> bool {
    io::stdout().flush().unwrap();
    let mut s = String::new();
    io::stdin().read_line(&mut s).ok();
    match &*s.trim() {
        "Y" | "y" | "yes" | "Yes" => true,
        _ => false,
    }
}
