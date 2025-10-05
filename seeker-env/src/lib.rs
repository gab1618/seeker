use crate::error::SeekerEnvErr;

pub mod error;

pub struct EnvArgs {
    pub bind_url: String,
    pub elasticsearch_cluster_url: String,
}

impl EnvArgs {
    pub fn load() -> anyhow::Result<Self> {
        let daemon_bind_url_env =
            std::env::var("SEEKER_DAEMON_BIND_URL").map_err(SeekerEnvErr::LoadDaemonBindUrl)?;
        let elastic_search_cluster_url = std::env::var("SEEKER_ELASTICSEARCH_CLUSTER_URL")
            .map_err(SeekerEnvErr::LoadElasticSearchClusterUrl)?;
        Ok(Self {
            bind_url: daemon_bind_url_env,
            elasticsearch_cluster_url: elastic_search_cluster_url,
        })
    }
}
