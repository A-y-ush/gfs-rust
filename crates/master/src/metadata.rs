use std::collections::HashMap;
use shared::messages::ChunkID;
use std ::sync::{Arc,RwLock};


#[derive(Debug, Default, Clone)]
pub struct ChunkMetadata{
    pub chunk_id: ChunkID,
    pub replica_locations: Vec<String>,
}

#[derive(Debug, Default, Clone)]
pub struct MetadataState{
    // stores metadata for each file with 
    // key - file_id: string
    // value - vector of ChunkMetadata
    pub files: Arc<RwLock<HashMap<String, Vec<ChunkMetadata>>>>,
}

impl MetadataState{

    pub fn new() -> Self {
        Self {
            files: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn register_files(&self, file_id: String, chunk_metadata: ChunkMetadata) {
        let mut files = self.files.write().unwrap();
        files.entry(file_id).or_default().push(chunk_metadata);
    }

    pub fn get_chunk_locations(&self, chunk_id: &ChunkID) -> Option<Vec<String>> {
        let files = self.files.read().unwrap();
        files.get(&chunk_id.file_id).and_then(|chunks| {
            chunks.iter()
                .find(|cm| cm.chunk_id == *chunk_id)
                .map(|cm| cm.replica_locations.clone())
        })
    }

    pub fn get_file_metadata(&self, file_id: &str) -> Option<Vec<ChunkMetadata>> {
        let files = self.files.read().unwrap();
        files.get(file_id).cloned()
    }

}