use std::io::BufRead;

pub struct GitArgs {
    #[allow(dead_code)]
    pub old_rev: String,
    #[allow(dead_code)]
    pub new_rev: String,
    pub ref_name: String,
}

#[derive(Debug)]
pub enum ParseGitArgsErr {
    InvalidFormat,
    NullInput,
    InputErr,
}

pub fn parse_git_args(stdin: &std::io::Stdin) -> Result<GitArgs, ParseGitArgsErr> {
    let hook_args_line = stdin
        .lock()
        .lines()
        .next()
        .ok_or(ParseGitArgsErr::NullInput)?
        .map_err(|_| ParseGitArgsErr::InputErr)?;

    let hook_args: Vec<&str> = hook_args_line.split_whitespace().collect();
    if hook_args.len() != 3 {
        return Err(ParseGitArgsErr::InvalidFormat);
    }

    let parsed_args = GitArgs {
        old_rev: hook_args[0].to_owned(),
        new_rev: hook_args[1].to_owned(),
        ref_name: hook_args[2].to_owned(),
    };

    Ok(parsed_args)
}
