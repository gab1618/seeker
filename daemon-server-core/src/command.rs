use std::path::PathBuf;

use crate::error::DaemonServerError;

#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum DaemonAction {
    Index,
}

impl TryFrom<&str> for DaemonAction {
    type Error = DaemonServerError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "index" => Ok(Self::Index),
            _ => Err(DaemonServerError::ParseCommand),
        }
    }
}
impl Into<String> for DaemonAction {
    fn into(self) -> String {
        match self {
            DaemonAction::Index => String::from("index"),
        }
    }
}
pub struct DaemonCommand {
    pub action: DaemonAction,
    pub filepath: PathBuf,
}

impl DaemonCommand {
    pub fn new(action: DaemonAction, filepath: PathBuf) -> Self {
        Self { action, filepath }
    }
}

impl TryFrom<&str> for DaemonCommand {
    type Error = DaemonServerError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut args = value.split(" ");
        let action_arg = args.next().ok_or(DaemonServerError::ParseCommand)?;
        let parsed_action: DaemonAction = action_arg.try_into()?;

        let file_path_arg = args.next().ok_or(DaemonServerError::ParseCommand)?;
        let parsed_file_path: PathBuf = file_path_arg.into();

        Ok(Self {
            action: parsed_action,
            filepath: parsed_file_path,
        })
    }
}

impl Into<String> for DaemonCommand {
    fn into(self) -> String {
        let str_action: String = self.action.into();
        format!("{} {}", str_action, self.filepath.display())
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::{DaemonAction, DaemonCommand};

    #[test]
    fn test_parse_command() {
        let example_path = PathBuf::from("./test.txt");

        let cmd = DaemonCommand {
            action: super::DaemonAction::Index,
            filepath: example_path.clone(),
        };
        let serialized: String = cmd.into();
        let parsed = DaemonCommand::try_from(serialized.as_str()).unwrap();
        assert_eq!(parsed.action, DaemonAction::Index);
        assert_eq!(parsed.filepath, example_path);
    }
}
