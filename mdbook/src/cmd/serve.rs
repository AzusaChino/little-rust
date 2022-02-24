use clap::{App, Arg, ArgMatches, SubCommand};

// Create clap subcommand arguments
pub fn make_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("serve")
        .about("Serves a book at http://localhost:3000, and rebuilds it on changes")
        .arg_from_usage(
            "-d, --dest-dir=[dest-dir] 'Output directory for the book{n}\
             Relative paths are interpreted relative to the book's root directory.{n}\
             If omitted, mdBook uses build.build-dir from book.toml or defaults to `./book`.'",
        )
        .arg_from_usage(
            "[dir] 'Root directory for the book{n}\
             (Defaults to the Current Directory when omitted)'",
        )
        .arg(
            Arg::with_name("hostname")
                .short("n")
                .long("hostname")
                .takes_value(true)
                .default_value("localhost")
                .empty_values(false)
                .help("Hostname to listen on for HTTP connections"),
        )
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .takes_value(true)
                .default_value("3000")
                .empty_values(false)
                .help("Port to use for HTTP connections"),
        )
        .arg_from_usage("-o, --open 'Opens the book server in a web browser'")
}
