use std::io::BufRead;

pub struct GitArgs {
    #[allow(dead_code)]
    pub old_rev: String,
    #[allow(dead_code)]
    pub new_rev: String,
    pub ref_name: String,
}

impl TryFrom<&str> for GitArgs {
    type Error = ParseGitArgsErr;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut args_parts = value.split_whitespace();

        let old_rev = args_parts.next().ok_or(ParseGitArgsErr::InvalidFormat)?;
        let new_rev = args_parts.next().ok_or(ParseGitArgsErr::InvalidFormat)?;
        let ref_name = args_parts.next().ok_or(ParseGitArgsErr::InvalidFormat)?;

        Ok(Self {
            old_rev: old_rev.to_owned(),
            new_rev: new_rev.to_owned(),
            ref_name: ref_name.to_owned(),
        })
    }
}

#[derive(Debug)]
pub enum ParseGitArgsErr {
    InvalidFormat,
    NullInput,
    InputErr,
}

#[cfg(test)]
mod tests {
    use crate::args::GitArgs;

    #[test]
    fn test_parse_args() {
        let example_input =
            "ab123456789012345678901234567890 cd987654321098765432109876543210 refs/heads/main";
        let parsed = GitArgs::try_from(example_input).unwrap();

        assert_eq!(parsed.old_rev, "ab123456789012345678901234567890");
        assert_eq!(parsed.new_rev, "cd987654321098765432109876543210");
        assert_eq!(parsed.ref_name, "refs/heads/main");
    }
}
