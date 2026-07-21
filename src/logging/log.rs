use owo_colors::OwoColorize;

pub(crate) fn info(text: &str) {
    println!("{} {}", "[INFO ]".green().bold(), text)
}

pub(crate) fn warn(text: &str) {
    println!("{} {}", "[WARN ]".yellow().bold(), text)
}

pub(crate) fn error(text: &str) {
    eprintln!("{} {}", "[ERROR]".red().bold(), text)
}
