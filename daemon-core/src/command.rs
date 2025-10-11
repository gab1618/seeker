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
impl From<DaemonAction> for String {
    fn from(val: DaemonAction) -> Self {
        match val {
            DaemonAction::Index => String::from("index"),
        }
    }
}
pub struct DaemonCommand {
    pub action: DaemonAction,
    pub repo_path: String,
}

impl DaemonCommand {
    pub fn new(action: DaemonAction, repo_path: String) -> Self {
        Self { action, repo_path }
    }
}

impl TryFrom<&str> for DaemonCommand {
    type Error = DaemonServerError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut args = value.split(" ");
        let action_arg = args.next().ok_or(DaemonServerError::ParseCommand)?;
        let parsed_action: DaemonAction = action_arg.try_into()?;

        let path_arg = args
            .next()
            .ok_or(DaemonServerError::ParseCommand)?
            .trim()
            .to_owned();

        Ok(Self {
            action: parsed_action,
            repo_path: path_arg,
        })
    }
}

impl From<DaemonCommand> for String {
    fn from(val: DaemonCommand) -> Self {
        let str_action: String = val.action.into();
        format!("{} {}", str_action, val.repo_path)
    }
}

#[cfg(test)]
mod tests {
    use super::{DaemonAction, DaemonCommand};

    #[test]
    fn test_parse_command() {
        let example_example_path = "./repos/repo.git".to_owned();

        let cmd = DaemonCommand {
            action: super::DaemonAction::Index,
            repo_path: example_example_path.clone(),
        };
        let serialized: String = cmd.into();
        let parsed = DaemonCommand::try_from(serialized.as_str()).unwrap();
        assert_eq!(parsed.action, DaemonAction::Index);
        assert_eq!(parsed.repo_path, example_example_path);
    }
}
