use crate::error::DaemonServerError;

#[cfg_attr(test, derive(Debug, Clone, PartialEq, Eq))]
pub struct DaemonResponse {
    pub message: String,
    pub status: DaemonResponseStatus,
}

impl Into<String> for &DaemonResponse {
    fn into(self) -> String {
        let str_status: String = (&self.status).into();
        format!("{} {}", str_status, self.message)
    }
}

impl TryFrom<&str> for DaemonResponse {
    type Error = DaemonServerError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut args = value.split(" ");
        let status_arg = args.next().ok_or(DaemonServerError::ParseResponse)?;
        let message_arg = args.collect::<Vec<&str>>().join(" ");
        let parsed_status: DaemonResponseStatus = status_arg.try_into()?;

        Ok(Self {
            message: message_arg.to_string(),
            status: parsed_status,
        })
    }
}

#[cfg_attr(test, derive(Debug, Clone, PartialEq, Eq))]
pub enum DaemonResponseStatus {
    Ok,
    Err,
}

impl Into<String> for &DaemonResponseStatus {
    fn into(self) -> String {
        match self {
            DaemonResponseStatus::Ok => "OK".to_string(),
            DaemonResponseStatus::Err => "ERR".to_string(),
        }
    }
}

impl TryFrom<&str> for DaemonResponseStatus {
    type Error = DaemonServerError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "OK" => Ok(Self::Ok),
            "ERR" => Ok(Self::Err),
            _ => Err(DaemonServerError::ParseResponse),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::response::DaemonResponse;

    #[test]
    fn parse_response() {
        let example_response = DaemonResponse {
            message: "Some message".to_string(),
            status: super::DaemonResponseStatus::Ok,
        };
        let serialized_response: String = (&example_response).into();

        let parsed_response: DaemonResponse =
            serialized_response.as_str().try_into().unwrap();

        assert_eq!(example_response, parsed_response);
    }
}
