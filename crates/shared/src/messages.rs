use bincode::{Decode, Encode};
use serde::{Serialize, Deserialize};

/// Sent periodically by each ChunkServer to inform the Master of its status and available chunks.
#[derive(Debug, Serialize, Deserialize, Clone,Encode,Decode)]
pub struct Heartbeat {
    pub server_id: String,
    pub available_chunks: Vec<ChunkID>,
}

/// A command issued by the client to the Master, specifying a file operation.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileCommand {
    pub filename: String,
    pub operation: OperationType,
}

/// Type of file operation requested in a FileCommand.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum OperationType {
    Upload,
    Download,
}

/// Identifies a specific chunk of a file.
/// Each file is broken into chunks for storage and redundancy.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash,Encode,Decode)]
pub struct ChunkID {
    pub file_id: String,
    pub index: u32,
}

/// Metadata that describes a chunk and where it is stored.
/// Used by the Master to tell clients where to read/write chunks.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChunkMetadata {
    pub chunk_id: ChunkID,
    pub servers: Vec<String>,
}
