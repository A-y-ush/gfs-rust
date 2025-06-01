use master::metadata::{ChunkMetadata, MetadataState};
use shared::messages::ChunkID;

#[test]
fn test_register_and_query_metadata() {
    let state = MetadataState::new();

    let file_id = "file1".to_string();
    let chunk_id = ChunkID { file_id: file_id.clone(), index: 0 };

    let chunk = ChunkMetadata {
        chunk_id: chunk_id.clone(),
        replica_locations: vec!["server1".to_string(), "server2".to_string()],
    };

    state.register_files(file_id.clone(), chunk.clone());

    let replicas = state.get_chunk_locations(&chunk_id);
    assert!(replicas.is_some());
    assert_eq!(replicas.unwrap(), chunk.replica_locations);
}
