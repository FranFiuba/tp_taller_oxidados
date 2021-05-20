pub struct Logger{
    logger: String,
}


impl Logger {
    pub fn new() -> Logger {
        Logger {
            logger: String::new(),
        }
    }
}