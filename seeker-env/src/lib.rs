use crate::error::SeekerEnvErr;

pub mod error;

pub struct EnvArgs {
    pub elasticsearch_cluster_url: String,
    pub elasticsearch_index_name: String,
}

impl EnvArgs {
    pub fn load() -> anyhow::Result<Self> {
        let elasticsearch_cluster_url =
            std::env::var("SEEKER_DAEMON_BIND_URL").map_err(SeekerEnvErr::LoadESClusterUrl)?;
        let elasticsearch_index_name =
            std::env::var("SEEKER_DAEMON_ES_INDEX_NAME").map_err(SeekerEnvErr::LoadESIndexName)?;
        Ok(Self {
            elasticsearch_cluster_url,
            elasticsearch_index_name,
        })
    }
}
