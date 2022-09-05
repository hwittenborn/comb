use colored::Colorize;

pub fn error(string: &str) {
    print!("{} {}", "Err:".red().bold(), string);
}
