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
