pub mod error;

pub struct EnvArgs {
    pub bind_url: String,
}

impl EnvArgs {
    pub fn load() -> anyhow::Result<Self> {
        let daemon_bind_url_env = std::env::var("SEEKER_DAEMON_BIND_URL")?;
        Ok(Self {
            bind_url: daemon_bind_url_env,
        })
    }
}
