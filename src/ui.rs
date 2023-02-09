#![allow(unused)]

macro_rules! warn {
    ($fmt:literal, $ex:expr) => {{
        use console::{style, Emoji};
        use std::env;
        let format_str = format!($fmt, $ex);
        if env::var("NO_EMOJI").is_ok() {
            println!("{} {}", style("!").red(), style(format_str).red());
        } else {
            println!(
                "{} {}",
                style(Emoji("⚠️ ", "!")).red(),
                style(formatstr).red()
            )
        }
    }};
}

macro_rules! success {
    ($fmt:literal, $ex:expr) => {{
        use console::{style, Emoji};
        use std::env;
        let formatstr = format!($fmt, $ex);
        if env::var("NO_EMOJI").is_ok() {
            println!("{} {}", style("✓").green(), style(formatstr).green());
        } else {
            println!(
                "{} {}",
                style(Emoji("✅", "✓")).green(),
                style(formatstr).green()
            );
        }
    }};
}
