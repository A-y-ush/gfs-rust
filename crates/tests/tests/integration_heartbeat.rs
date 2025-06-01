use master::state::ChunkServerState;
use master::server::start_heartbeat_server;
use chunk_server::heartbeat::send_heartbeat_periodically;
use shared::messages::ChunkID;
use std::sync::Arc;
use std::net::SocketAddr;
use std::time::Duration;
use tokio::task;
use tokio::time::sleep;

#[tokio::test]
async fn test_heartbeat_integration() {

    let state = ChunkServerState::new();
    let addr: SocketAddr = "127.0.0.1:9010".parse().unwrap(); 

    let state_clone = state.clone();
    let state_arc =  Arc::new(state_clone);
    task::spawn(async move {
        start_heartbeat_server(state_arc).await.unwrap();
    });

    sleep(Duration::from_millis(100)).await;

    let chunk_server_id = "test-chunk-1".to_string();
    let master_addr = addr.to_string();

    let available_chunks = vec![
        ChunkID {
            file_id: "file1".to_string(),
            index: 0,
        },
        ChunkID {
            file_id: "file1".to_string(),
            index: 1,
        },
    ];


    task::spawn(async move {
        send_heartbeat_periodically(chunk_server_id.clone(), master_addr,available_chunks).await.unwrap();
    });

    // Wait for heartbeats to be sent
    sleep(Duration::from_secs(6)).await;

    // Check if master's state was updated
    let chunks = state.get_available_chunks("test-chunk-1");
    assert!(chunks.is_some(), "Master did not receive heartbeat");
    let chunks = chunks.unwrap();

    assert!(chunks.contains(&ChunkID {
        file_id: "file1".to_string(),
        index: 0
    }));

    assert!(chunks.contains(&ChunkID {
        file_id: "file1".to_string(),
        index: 1
    }));
}
