use std::{
    fs::OpenOptions,
    io::{Read, Write},
    path::PathBuf,
};

pub struct StateManager {
    config_dir_path: PathBuf,
}

pub enum StateValue {
    LastIndexedCommit,
}

impl StateManager {
    pub fn new(config_dir_path: PathBuf) -> Self {
        std::fs::create_dir_all(&config_dir_path).unwrap();
        Self { config_dir_path }
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
    #[allow(dead_code)]
    pub fn save_state_value(&self, state: StateValue, value: &str) -> std::io::Result<()> {
        let mut f = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(self.get_state_file_path(state))?;

        write!(f, "{value}").unwrap();

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
        let tracker = StateManager::new(config_dir_path.path().to_owned());
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
