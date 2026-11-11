use owo_colors::OwoColorize;
use std::fmt::Display;

#[derive(Clone, Copy, PartialEq, PartialOrd)]
enum Level {
    Verbose,
    Trace,
    Info,
    Warn,
    Error,
}

const LEVEL: Level = Level::Verbose;

fn write(level: Level, label: impl Display, message: impl Display) {
    if LEVEL > level {
        return;
    }

    let prefix = format!(
        "{}{}{}{}",
        "jafa ".bold().cyan(),
        "[".bright_black(),
        label,
        "]".bright_black()
    );

    if level >= Level::Warn {
        eprintln!("{prefix} {message}");
    } else {
        println!("{prefix} {message}");
    }
}

pub fn verbose(message: impl Display) {
    write(Level::Verbose, "verbose".magenta(), message);
}

pub fn trace(message: impl Display) {
    write(Level::Trace, "trace".blue(), message);
}

pub fn info(message: impl Display) {
    write(Level::Info, "info".bright_green(), message);
}

pub fn warn(message: impl Display) {
    write(Level::Warn, "WARN".yellow(), message);
}

pub fn error(message: impl Display) {
    write(Level::Error, "ERR!".red(), message);
}
