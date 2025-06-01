pub mod heartbeat;
use shared::messages::ChunkID;
use heartbeat::send_heartbeat_periodically;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    let server_id = "chunk-server-1".to_string();
    let master_addr = "127.0.0.1:5001".to_string(); // match the master port
    let available_chunks = vec![
        ChunkID { file_id: "file1".to_string(), index: 0 },
        ChunkID { file_id: "file2".to_string(), index: 1 },
    ];

    send_heartbeat_periodically(server_id, master_addr, available_chunks, 5).await
}

