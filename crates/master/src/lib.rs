pub mod state;
pub mod server;
pub mod metadata;
use anyhow::Result;
use std::sync::Arc;
use state::ChunkServerState;
use server::start_heartbeat_server;

pub async fn run() -> Result<()> {
    let state = Arc::new(ChunkServerState::new());
    start_heartbeat_server(state).await?;
    Ok(())
}
