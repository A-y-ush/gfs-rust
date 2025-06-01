use tokio::net::TcpListener;
use tokio::io::AsyncReadExt;
use shared::messages::Heartbeat;
use crate::state::ChunkServerState;
use std::sync::Arc;
use anyhow::Result;
use bincode;

/// Starts the TCP server to receive heartbeats from chunk servers.
pub async fn start_heartbeat_server(state: Arc<ChunkServerState>) -> Result<()> {
    let listener = TcpListener::bind("0.0.0.0:5000").await?;
    println!("🟢 Master server listening on port 5000 for heartbeats...");

    loop {
        let (mut socket, _) = listener.accept().await?;
        let state = Arc::clone(&state);

        tokio::spawn(async move {
            let mut buffer = vec![0u8; 4096];

            match socket.read(&mut buffer).await {
                Ok(n) if n > 0 => {
                    let data = &buffer[..n];

                    match bincode::decode_from_slice::<Heartbeat,_>(
                        data,
                        bincode::config::standard(),
                    ) {
                        Ok((heartbeat, _)) => {
                            println!("Received heartbeat from {}", heartbeat.server_id);
                            state.update_heartbeat(heartbeat);
                        }
                        Err(e) => eprintln!("Failed to deserialize heartbeat: {:?}", e),
                    }
                }
                Ok(_) => {
                    eprintln!("Received empty message");
                }
                Err(e) => {
                    eprintln!("Failed to read from socket: {:?}", e);
                }
            }
        });
    }
}
