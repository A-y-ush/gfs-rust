use shared::messages::{ChunkID,Heartbeat};
use tokio::net::TcpStream;
use bincode;
use std::time::Duration;
use tokio::time;
use tokio::io::AsyncWriteExt;
use std::io;

pub async fn send_heartbeat_periodically(server_id:String,master_adr:String,available_chunks:Vec<ChunkID>)->io::Result<()>{

    loop{
        let heartbeat = Heartbeat{
            server_id:server_id.clone(),
            available_chunks:available_chunks.clone(),
        };

        match TcpStream::connect(master_adr.to_string()).await {
            Ok(mut stream) => {
                let encoded = bincode::encode_to_vec(&heartbeat, bincode::config::standard())
                    .expect("Failed to encode heartbeat");

                if let Err(e) = stream.write_all(&encoded).await {
                    eprintln!("Failed to send heartbeat: {:?}", e);
                } else {
                    println!("Heartbeat sent successfully from {}", server_id);
                }
            }
            Err(e) => {
                eprintln!("Failed to connect to master server: {:?}", e);
            }
        }

        time::sleep(Duration::from_secs(5)).await; // Send heartbeat every 5 seconds
    }

}