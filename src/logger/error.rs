use colored::Colorize;

use super::Console;

pub trait Error {
    fn error(&self, text: String);
    fn invalid(&self, text: String);
    fn critical(&self, text: String);
    fn fail(&self, text: String);
}

impl Error for Console {
    fn error(&self, text: String) {
        let msg = if self.timestamps {
            format!("{} [ERROR] > {}", Console::get_timestamp(), text)
        } else {
            format!("[ERROR] > {}", text)
        };

        println!("{}", msg.red())
    }

    fn invalid(&self, text: String) {
        let msg = if self.timestamps {
            format!("{} [INVALID] > {}", Console::get_timestamp(), text)
        } else {
            format!("[INVALID] > {}", text)
        };

        println!("{}", msg.red())
    }

    fn critical(&self, text: String) {
        let msg = if self.timestamps {
            format!("{} [CRITIC] > {}", Console::get_timestamp(), text)
        } else {
            format!("[CRITIC] > {}", text)
        };

        println!("{}", msg.red())
    }

    fn fail(&self, text: String) {
        let msg = if self.timestamps {
            format!("{} [FAIL] > {}", Console::get_timestamp(), text)
        } else {
            format!("[FAIL] > {}", text)
        };

        println!("{}", msg.red())
    }
}