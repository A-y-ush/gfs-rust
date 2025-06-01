use shared::messages::{ChunkID, Heartbeat, MasterRequest};
use tokio::net::TcpStream;
use bincode;
use tokio::time::{self, Duration};
use tokio::io::AsyncWriteExt;
use anyhow::Result;
use tracing::{info, error};

/// Periodically sends heartbeat messages to the master server.
pub async fn send_heartbeat_periodically(
    server_id: String,
    master_addr: String,
    available_chunks: Vec<ChunkID>,
    interval_secs: u64,
) -> Result<()> {
    let interval = Duration::from_secs(interval_secs);

    loop {
        let heartbeat = Heartbeat {
            server_id: server_id.clone(),
            available_chunks: available_chunks.clone(),
        };

        let message = MasterRequest::Heartbeat(heartbeat);

        match TcpStream::connect(&master_addr).await {
            Ok(mut stream) => {
                let encoded = bincode::encode_to_vec(&message, bincode::config::standard())?;

                if let Err(e) = stream.write_all(&encoded).await {
                    error!(" Failed to send heartbeat from {}: {:?}", server_id, e);
                } else {
                    info!("Heartbeat sent successfully from {}", server_id);
                }
            }
            Err(e) => {
                error!("🔌 Failed to connect to master at {}: {:?}", master_addr, e);
            }
        }

        time::sleep(interval).await;
    }
}
