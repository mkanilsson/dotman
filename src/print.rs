use colored::{ColoredString, Colorize};

fn format(t: ColoredString, message: &str) {
    println!("{}: {}", t, message);
}

pub fn success(message: &str) {
    format("SUCCESS".green().bold(), message);
}

pub fn info(message: &str) {
    println!("{}", message);
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

pub struct Printer {
    package: String,
}

impl Printer {
    pub fn new(package: String) -> Self {
        Self { package }
    }

    pub fn success(&self, message: &str) {
        success(&format!("{}: {}", self.package.bold().italic(), message))
    }

    pub fn info(&self, message: &str) {
        info(&format!("{}: {}", self.package.bold().italic(), message))
    }

    pub fn warning(&self, message: &str) {
        warning(&format!("{}: {}", self.package.bold().italic(), message))
    }

    pub fn fatal(&self, message: &str) {
        fatal(&format!("{}: {}", self.package.bold().italic(), message))
    }
}
