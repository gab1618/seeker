use std::path::PathBuf;
use std::time::SystemTime;

use crate::error::{SeekerHookErr, SeekerHookResult};

pub struct Logger {}

impl Logger {
    pub fn setup_logging(log_file_path: PathBuf) -> SeekerHookResult<()> {
        fern::Dispatch::new()
            .format(|out, message, record| {
                out.finish(format_args!(
                    "[{} {} {}] {}",
                    humantime::format_rfc3339_seconds(SystemTime::now()),
                    record.level(),
                    record.target(),
                    message
                ))
            })
            .level(log::LevelFilter::Debug)
            .chain(std::io::stdout())
            .chain(fern::log_file(log_file_path).map_err(|_| SeekerHookErr::SetupLogFile)?)
            .apply()
            .map_err(|_| SeekerHookErr::ApplyLogConfig)?;
        Ok(())
    }
}
