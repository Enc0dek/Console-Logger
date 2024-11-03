use colored::Colorize;
use super::console::Console;

pub trait OK {
    fn valid(&self, text: String);
    fn ok(&self, text: String);
    fn success(&self, text: String);
}

impl OK for Console {
    fn valid(&self, text: String) {
        let msg = if self.timestamps {
            format!("{} [VALID] > {}", Console::get_timestamp(), text)
        } else {
            format!("[VALID] > {}", text)
        };

        println!("{}", msg.green())
    }

    fn ok(&self, text: String) {
        let msg = if self.timestamps {
            format!("{} [OK] > {}", Console::get_timestamp(), text)
        } else {
            format!("[OK] > {}", text)
        };

        println!("{}", msg.green())
    }

    fn success(&self, text: String) {
        let msg = if self.timestamps {
            format!("{} [SUCCESS] > {}", Console::get_timestamp(), text)
        } else {
            format!("[SUCCESS] > {}", text)
        };

        println!("{}", msg.green())
    }
}