use shared::messages::*;
use bincode;
#[test]
fn test_heartbeat_serialization() {
    let heartbeat:Heartbeat= Heartbeat {
        server_id: "server-1".to_string(),
        available_chunks: vec![
            ChunkID { file_id: "file123".to_string(), index: 0 },
        ],
    };

    let encoded :Vec<u8>= bincode::encode_to_vec(&heartbeat, bincode::config::standard()).unwrap();
    let decoded: Heartbeat = bincode::decode_from_slice(&encoded, bincode::config::standard()).unwrap().0;
    assert_eq!(heartbeat.server_id, decoded.server_id);
    assert_eq!(heartbeat.available_chunks, decoded.available_chunks);
}
