use std::path::PathBuf;

#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum SeekerDaemonAction {
    Index,
}

impl TryFrom<&str> for SeekerDaemonAction {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "index" => Ok(Self::Index),
            _ => Err(()),
        }
    }
}
impl Into<String> for SeekerDaemonAction {
    fn into(self) -> String {
        match self {
            SeekerDaemonAction::Index => String::from("index"),
        }
    }
}
pub struct SeekerDaemonCommand {
    pub action: SeekerDaemonAction,
    pub filepath: PathBuf,
}

impl SeekerDaemonCommand {
    pub fn new(action: SeekerDaemonAction, filepath: PathBuf) -> Self {
        Self { action, filepath }
    }
}

impl TryFrom<&str> for SeekerDaemonCommand {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut args = value.split(" ");
        let action_arg = args.next().ok_or(())?;
        let parsed_action: SeekerDaemonAction = action_arg.try_into()?;

        let file_path_arg = args.next().ok_or(())?;
        let parsed_file_path: PathBuf = file_path_arg.into();

        Ok(Self {
            action: parsed_action,
            filepath: parsed_file_path,
        })
    }
}

impl Into<String> for SeekerDaemonCommand {
    fn into(self) -> String {
        let str_action: String = self.action.into();
        format!("{} {}", str_action, self.filepath.display())
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{SeekerDaemonAction, SeekerDaemonCommand};

    #[test]
    fn test_parse_command() {
        let example_path = PathBuf::from("./test.txt");

        let cmd = SeekerDaemonCommand {
            action: super::SeekerDaemonAction::Index,
            filepath: example_path.clone(),
        };
        let serialized: String = cmd.into();
        let parsed = SeekerDaemonCommand::try_from(serialized.as_str()).unwrap();
        assert_eq!(parsed.action, SeekerDaemonAction::Index);
        assert_eq!(parsed.filepath, example_path);
    }
}
