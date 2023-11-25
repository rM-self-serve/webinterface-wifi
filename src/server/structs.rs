use crate::config::factory::Config;
use std::{error::Error, sync::Arc};
use tokio::{sync::Mutex, task::JoinHandle};

pub struct SystemState {
    pub srvr_thread: Option<JoinHandle<Result<(), Box<dyn Error + Send + Sync>>>>,
    pub connection_hash: Option<u64>,
    pub config: Config,
}
impl SystemState {
    pub fn init(config: Config) -> Arc<Mutex<SystemState>> {
        let system_state = SystemState {
            srvr_thread: None,
            connection_hash: None,
            config: config,
        };
        Arc::new(Mutex::new(system_state))
    }
}
