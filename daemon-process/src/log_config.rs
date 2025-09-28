use systemd_journal_logger::JournalLog;

pub fn setup_logging() -> anyhow::Result<()> {
    JournalLog::new()?.install()?;
    log::set_max_level(log::LevelFilter::Info);

    Ok(())
}
