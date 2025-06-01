use shared::messages::{ChunkID, Heartbeat};
use tokio::net::TcpStream;
use bincode;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Create a test heartbeat
    let heartbeat = Heartbeat {
        server_id: "test-chunk-server".to_string(),
        available_chunks: vec![
            ChunkID {
                file_id: "file1".to_string(),
                index: 0,
            },
            ChunkID {
                file_id: "file2".to_string(),
                index: 1,
            },
        ],
    };

    let mut stream = TcpStream::connect("127.0.0.1:5000").await?;
    println!("✅ Connected to master");

    let encoded = bincode::encode_to_vec(&heartbeat, bincode::config::standard())?;

    stream.writable().await?;
    tokio::io::AsyncWriteExt::write_all(&mut stream, &encoded).await?;

    println!("📤 Sent heartbeat to master");

    Ok(())
}
