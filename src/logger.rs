pub enum log_flags {
    SYNTAX = 1 << 0,
    DB = 1 << 1,
}

// const FLAGS: u32 = log_flags::DB as u32;
const FLAGS: u32 = 0;

pub struct log {
}

impl log {
    pub fn new() -> log {
        log {}
    }

    pub fn has_flag(&self, flag: log_flags) -> bool {
        FLAGS & flag as u32 != 0
    }

    pub fn syntax(&self, message: &str) {
        if self.has_flag(log_flags::SYNTAX) {
            println!("[Syntax]: {}", message);
        }
    }

    pub fn db(&self, message: &str) {
        if self.has_flag(log_flags::DB) {
            println!("[DB]: {}", message);
        }
    }
}
