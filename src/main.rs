use logger::{Console, Info, OK, Error, Warn};

mod logger;

fn main() {
    let cls = Console::new(true);

    let text = String::from("EXAMPLE");

    // VALID
    cls.valid(text.clone());
    cls.ok(text.clone());
    cls.success(text.clone());

    // Warn
    cls.warning(text.clone());
    cls.caution(text.clone());
    cls.alert(text.clone());

    // ERROR
    cls.error(text.clone());
    cls.invalid(text.clone());
    cls.critical(text.clone());
    cls.fail(text.clone());

    // INFO
    cls.info(text);
}
