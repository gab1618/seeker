use std::{
    fs::OpenOptions,
    io::{Read, Write},
    path::PathBuf,
};

use crate::error::DaemonServerError;

pub struct State {
    config_dir_path: PathBuf,
}

pub enum StateValue {
    LastIndexedCommit,
}

impl State {
    pub fn new(config_dir_path: PathBuf) -> anyhow::Result<Self> {
        std::fs::create_dir_all(&config_dir_path).map_err(DaemonServerError::SetupStateDir)?;
        Ok(Self { config_dir_path })
    }
    fn get_state_file_path(&self, state: StateValue) -> PathBuf {
        match state {
            StateValue::LastIndexedCommit => self.config_dir_path.join("last-commit"),
        }
    }
    pub fn get_state_file_value(&self, state: StateValue) -> Option<String> {
        let mut last_indexed_commit = String::new();
        if OpenOptions::new()
            .read(true)
            .open(self.get_state_file_path(state))
            .map(|mut f| f.read_to_string(&mut last_indexed_commit))
            .is_err()
        {
            return None;
        }

        Some(last_indexed_commit)
    }
    pub fn save_state_value(&self, state: StateValue, value: &str) -> anyhow::Result<()> {
        let mut f = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(self.get_state_file_path(state))
            .map_err(DaemonServerError::SaveStateValue)?;

        write!(f, "{value}").map_err(DaemonServerError::SaveStateValue)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::*;

    #[test]
    fn test_save_and_retrieve_last_indexed_commit() {
        let config_dir_path = tempdir().unwrap();
        let tracker = State::new(config_dir_path.path().to_owned()).unwrap();
        let example_commit = "aaaaa".to_owned();

        assert_eq!(
            tracker.get_state_file_value(StateValue::LastIndexedCommit),
            None
        );
        tracker
            .save_state_value(StateValue::LastIndexedCommit, &example_commit)
            .unwrap();
        assert_eq!(
            tracker.get_state_file_value(StateValue::LastIndexedCommit),
            Some(example_commit)
        );
    }
}
