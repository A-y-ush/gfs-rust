pub mod state;
pub mod server;
pub mod metadata;
use anyhow::Result;
use std::sync::Arc;
use state::ChunkServerState;
use server::start_master_server;
use std::net::SocketAddr;

pub async fn run() -> Result<()> {
    let state = Arc::new(ChunkServerState::new());
    let addr: SocketAddr = "127.0.0.1:5001".parse()?; // Fixed: Direct parse without SocketAddr::V4
    start_master_server(addr, state).await?;
    Ok(())
}