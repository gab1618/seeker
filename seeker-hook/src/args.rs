use std::io::BufRead;

pub struct GitArgs {
    pub oldRev: String,
    pub newRev: String,
    pub refName: String,
}

#[derive(Debug)]
pub enum ParseGitArgsErr {
    InvalidFormat,
}

pub fn parse_git_args(stdin: &std::io::Stdin) -> Result<GitArgs, ParseGitArgsErr> {
    let hook_args_line = stdin
        .lock()
        .lines()
        .next()
        .expect("Could not get hook args")
        .expect("Could not get hook args");

    let hook_args: Vec<&str> = hook_args_line.split_whitespace().collect();
    if hook_args.len() != 3 {
        return Err(ParseGitArgsErr::InvalidFormat);
    }

    let parsed_args = GitArgs {
        oldRev: hook_args[0].to_owned(),
        newRev: hook_args[1].to_owned(),
        refName: hook_args[2].to_owned(),
    };

    Ok(parsed_args)
}
