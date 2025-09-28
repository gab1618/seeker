pub mod error;

use error::{SeekerEnvErr, SeekerEnvResult};

pub struct EnvArgs {
    pub bind_url: String,
}

impl EnvArgs {
    pub fn load() -> SeekerEnvResult<Self> {
        let daemon_bind_url_env =
            std::env::var("SEEKER_DAEMON_BIND_URL").map_err(|_| SeekerEnvErr::LoadDaemonBindUrl)?;
        Ok(Self {
            bind_url: daemon_bind_url_env,
        })
    }
}
