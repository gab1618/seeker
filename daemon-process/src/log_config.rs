use core::fmt;
use std::process::exit;

use systemd_journal_logger::JournalLog;

use crate::error::{DaemonProcessErr, DaemonProcessResult};

pub fn setup_logging() -> DaemonProcessResult<()> {
    JournalLog::new()
        .map_err(|_| DaemonProcessErr::SetupLogger)?
        .install()
        .map_err(|_| DaemonProcessErr::StartLogger)?;
    log::set_max_level(log::LevelFilter::Info);

    Ok(())
}

/// This function unwraps the result and logs the error using the log crate
pub fn unwrap_log<T, E: fmt::Display + fmt::Debug>(res: Result<T, E>) -> T {
    if let Err(err) = res {
        log::error!("{err}");
        exit(1);
    }
    res.unwrap()
}
