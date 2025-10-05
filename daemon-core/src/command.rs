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
    pub target_oid: String,
}

impl DaemonCommand {
    pub fn new(action: DaemonAction, target_oid: String) -> Self {
        Self { action, target_oid }
    }
}

impl TryFrom<&str> for DaemonCommand {
    type Error = DaemonServerError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut args = value.split(" ");
        let action_arg = args.next().ok_or(DaemonServerError::ParseCommand)?;
        let parsed_action: DaemonAction = action_arg.try_into()?;

        let oid_arg = args
            .next()
            .ok_or(DaemonServerError::ParseCommand)?
            .to_owned();

        Ok(Self {
            action: parsed_action,
            target_oid: oid_arg,
        })
    }
}

impl From<DaemonCommand> for String {
    fn from(val: DaemonCommand) -> Self {
        let str_action: String = val.action.into();
        format!("{} {}", str_action, val.target_oid)
    }
}

#[cfg(test)]
mod tests {
    use super::{DaemonAction, DaemonCommand};

    #[test]
    fn test_parse_command() {
        let example_oid = "1111bbbbb".to_owned();

        let cmd = DaemonCommand {
            action: super::DaemonAction::Index,
            target_oid: example_oid.clone(),
        };
        let serialized: String = cmd.into();
        let parsed = DaemonCommand::try_from(serialized.as_str()).unwrap();
        assert_eq!(parsed.action, DaemonAction::Index);
        assert_eq!(parsed.target_oid, example_oid);
    }
}
