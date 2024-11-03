use colored::Colorize;

use super::Console;

pub trait Warn {
    fn warning(&self, text: String);
    fn caution(&self, text: String);
    fn alert(&self, text: String);
}

impl Warn for Console {
    fn warning(&self, text: String) {
        let msg = if self.timestamps {
            format!("{} [WARNING] > {}", Console::get_timestamp(), text)
        } else {
            format!("[WARNING] > {}", text)
        };

        println!("{}", msg.yellow())
    }

    fn alert(&self, text: String) {
        let msg = if self.timestamps {
            format!("{} [ALERT] > {}", Console::get_timestamp(), text)
        } else {
            format!("[ALERT] > {}", text)
        };

        println!("{}", msg.yellow())
    }

    fn caution(&self, text: String) {
        let msg = if self.timestamps {
            format!("{} [CAUTION] > {}", Console::get_timestamp(), text)
        } else {
            format!("[CAUTION] > {}", text)
        };

        println!("{}", msg.yellow())
    }
}