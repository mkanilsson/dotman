use colored::{ColoredString, Colorize};

fn format(t: ColoredString, message: &str) {
    println!("  [{}] {}", t, message);
}

pub fn success(message: &str) {
    format("SUCCESS".green().bold(), message);
}

pub fn info(message: &str) {
    format("INFO".blue().bold(), message);
}

pub fn warning(message: &str) {
    format("WARNING".yellow().bold(), message);
}

pub fn error(message: &str) {
    format("ERROR".red().bold(), message);
}

pub fn fatal(message: &str) {
    format("FATAL".red().bold(), message);
}
