use std::fmt::Display;

#[derive(Debug)]
pub enum SeekerEnvErr {
    LoadDaemonBindUrl,
}

impl Display for SeekerEnvErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SeekerEnvErr::LoadDaemonBindUrl => write!(f, "Could not load daemon bind url env arg"),
        }
    }
}

pub type SeekerEnvResult<T> = Result<T, SeekerEnvErr>;
