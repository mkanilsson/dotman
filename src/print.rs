use colored::{ColoredString, Colorize};

fn format(t: ColoredString, message: &str) {
    println!("  [{: ^7}] {}", t, message);
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

pub struct Printer {
    package: String,
}

impl Printer {
    pub fn new(package: String) -> Self {
        Self { package }
    }

    pub fn success(&self, message: &str) {
        success(&format!(
            "[Package '{}'] {}",
            self.package.bold().italic(),
            message
        ))
    }

    pub fn info(&self, message: &str) {
        info(&format!(
            "[Package '{}'] {}",
            self.package.bold().italic(),
            message
        ))
    }

    pub fn warning(&self, message: &str) {
        warning(&format!(
            "[Package '{}'] {}",
            self.package.bold().italic(),
            message
        ))
    }

    pub fn error(&self, message: &str) {
        error(&format!(
            "[Package '{}'] {}",
            self.package.bold().italic(),
            message
        ))
    }

    pub fn fatal(&self, message: &str) {
        fatal(&format!(
            "[Package '{}'] {}",
            self.package.bold().italic(),
            message
        ))
    }
}
