use colored::Colorize;
use super::Console;

pub trait Info {
    fn info(&self, text: String);
}

impl Info for Console {
    fn info(&self, text: String) {
        let msg = if self.timestamps {
            format!("{} [INFO] > {}", Console::get_timestamp(), text)
        } else {
            format!("[INFO] > {}", text)
        };

        println!("{}", msg.bright_blue())
    }
}