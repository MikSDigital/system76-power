use fern::{Dispatch, InitError};
use log::{Level, LevelFilter};
use std::io;

pub fn setup_logging() -> Result<(), InitError> {
    Dispatch::new()
        // Exclude logs for crates that we use
        .level(LevelFilter::Off)
        // Include only the logs for this binary
        .level_for("system76_power", LevelFilter::Debug)
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}] {}",
                record.level(),
                message
            ))
        })
        .chain(io::stderr())
        .apply()?;
    Ok(())
}