// verbatim from: https://docs.rs/log/latest/log/#implementing-a-logger
use log::{Level, Metadata, Record};
use log::{LevelFilter, SetLoggerError};

struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

static LOGGER: SimpleLogger = SimpleLogger;

pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Info))
}

// Tests to double-check that it works.
// The messages are sent to STDOUT and we'll configure the systemd service to send STDOUT to the
// journal.
#[cfg(test)]
mod tests {
    use super::*;
    use log::info;
    #[test]
    fn logs_to_stdout() {
        // to look at the test STDOUT, the tests need to be ran like this:
        // $ cargo test -- --nocapture
        let _ = init();
        info!("hello world!");
        println!("hello");
    }
}
