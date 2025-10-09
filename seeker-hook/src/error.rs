#[derive(Debug)]
pub enum SeekerHookErr {
    StartDaemonClient,
}

pub type SeekerHookResult<T> = Result<T, SeekerHookErr>;
