use dashmap::DashMap;
use shared::messages::{ChunkID,Heartbeat};
use std::sync::Arc;

// stores the current state of each chunk server
#[derive(Debug, Default, Clone)]
pub struct ChunkServerState{
    servers: Arc<DashMap<String,Vec<ChunkID>>>,
}

impl ChunkServerState{
    pub fn new()->Self{
        Self{
            servers: Arc::new(DashMap::new()),
        }
    }

    /// Handles a heartbeat message from a chunk server 
    /// This updates the state with the server's available chunks 
    /// A server can send a heartbeat to the master to inform it of its status and available chunks.
    /// a server represents a chunk server that stores chunks of files.

    pub fn handle_heartbeat(&self,heartbeat:Heartbeat){
        // Update the state with the heartbeat information
        let server_id = heartbeat.server_id;
        let available_chunks = heartbeat.available_chunks;

        // Insert or update the server's available chunks
        self.servers.insert(server_id, available_chunks);
    }

    pub fn update_heartbeat(&self, heartbeat: Heartbeat) {
    let server_id = heartbeat.server_id.clone();
    let new_chunks = heartbeat.available_chunks;

    match self.servers.get(&server_id) {
        Some(existing) if *existing == new_chunks => {
        }
        _ => {
            self.servers.insert(server_id, new_chunks);
        }
    }
}


    pub fn get_available_chunks(&self,server_id:&str)->Option<Vec<ChunkID>>{
        self.servers.get(server_id).map(|entry| entry.value().clone())
    }

}

