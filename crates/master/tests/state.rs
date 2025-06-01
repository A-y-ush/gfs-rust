use master::state::ChunkServerState;
use shared::messages::{Heartbeat,ChunkID};

#[test]
fn test_heartbeat_updates_state(){
    let state:ChunkServerState = ChunkServerState::new();

    let heartbeat:Heartbeat = Heartbeat{
        server_id: "chunk-1".to_string(),
        available_chunks: vec![
            ChunkID { file_id: "file-a".to_string(), index: 0 },
            ChunkID { file_id: "file-b".to_string(), index: 1 },
        ],
    };

    state.handle_heartbeat(heartbeat.clone());

    let stored_chunks = state.get_available_chunks("chunk-1").unwrap();
    assert_eq!(stored_chunks, heartbeat.available_chunks);

}