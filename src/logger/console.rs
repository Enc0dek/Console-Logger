use chrono::Local;

pub struct Console{
    pub timestamps: bool

}

impl Console {
    pub fn new(timestamps: bool) -> Console{
        Console{
            timestamps
        }
    }

    pub fn get_timestamp() -> String{
        Local::now().format("%H:%M:%S").to_string()
    }
}