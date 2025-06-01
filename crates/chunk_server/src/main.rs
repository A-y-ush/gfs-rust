pub mod heartbeat;
use shared::messages::ChunkID;
use heartbeat::send_heartbeat_periodically;

#[tokio::main]
async fn main() {
    let server_id = "server-1".to_string();
    let master_addr = "127.0.0.1:5000".to_string();
    let available_chunks = vec![
        ChunkID {
            file_id: "file123".to_string(),
            index: 0,
        }
    ];

    send_heartbeat_periodically(server_id, master_addr, available_chunks)
        .await
        .expect("heartbeat loop failed");
}
