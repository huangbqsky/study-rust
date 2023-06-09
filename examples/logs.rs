
use log::{Record, Level, Metadata, LevelFilter, SetLoggerError};

static CONSOLE_LOGGER: ConsoleLogger = ConsoleLogger;
struct ConsoleLogger;

impl log::Log for ConsoleLogger{
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
       if self.enabled(record.metadata()) {
          println!("Rust says: {} - {} - {:?}", record.level(), record.args(), record.metadata());
       }
    }

    fn flush(&self) {
        
    }
}

fn main() -> Result<(), SetLoggerError> {
    log::set_logger(&CONSOLE_LOGGER)?;
    log::set_max_level(LevelFilter::Info);

    log::info!("Hello log!");
    log::warn!("warning!");
    log::error!("oops!");

    Ok(())
    
}