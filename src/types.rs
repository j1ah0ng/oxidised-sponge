use log::{Record, Level, Metadata};

pub const LEN: usize = EXCEPTIONS.len();

pub const EXCEPTIONS: [char; 11] = [
    'C', 'I', 'l', 'K', 'o', 'S',
    'u', 'V', 'W', 'X', 'Z'];

// the sponging is DETERMINISTIC!
// now you can finally enjoy
// DETERMINISTIC SPONGEBOB
pub struct StateMachine {
    pub consec_up: u8,
    pub consec_down: u8, 
}

impl StateMachine {
    pub fn next_is_upper(&self) -> bool {
        self.consec_up < self.consec_down
    }

    pub fn new() -> StateMachine {
        StateMachine {
            consec_up: 0,
            consec_down: 0,
        }
    }
}

pub struct Logger;

impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            eprintln!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}
