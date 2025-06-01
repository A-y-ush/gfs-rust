use std::net::SocketAddr;
use rand::{rngs::StdRng, SeedableRng};
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use shared::messages::*;
use crate::state::ChunkServerState;
use bincode;
use rand::seq::IteratorRandom;
use std::sync::Arc;
use anyhow::Result;
use tracing::{ error, info, warn};
pub async fn start_master_server(addr: SocketAddr, state: Arc<ChunkServerState>) -> Result<()> {
    let listener = TcpListener::bind(addr).await?;
    info!("Master server listening on {}", addr);

    loop {
        let (mut socket, _) = listener.accept().await?;
        let state = Arc::clone(&state);

        tokio::spawn(async move {
            let mut buffer = vec![0u8; 4096];

            match socket.read(&mut buffer).await {
                Ok(n) if n > 0 => {
                    let decode_result = bincode::decode_from_slice::<MasterRequest, _>(
                        &buffer[..n],
                        bincode::config::standard(),
                    );

                    match decode_result {
                        Ok((msg, _)) => {
                            match msg {
                                MasterRequest::Heartbeat(hb) => {
                                    info!("Received Heartbeat from {}", hb.server_id);
                                    state.handle_heartbeat(hb);
                                }

                                MasterRequest::CreateFile(req) => {
                                    info!("Received CreateFileRequest for file {}", req.file_id);

                                    let all_servers = state.get_all_server_ids();
                                    let mut rng = StdRng::from_rng(&mut rand::rng());// Fixed: Proper RNG initialization

                                    let mut assignments = Vec::new();

                                    for i in 0..req.num_chunks {
                                        let chunk_id = ChunkID {
                                            file_id: req.file_id.clone(),
                                            index: i.try_into().unwrap(),
                                        };

                                        // Pick up to 3 replicas (can be less if fewer servers available)
                                        let replicas = all_servers
                                            .iter()
                                            .cloned()
                                            .choose_multiple(&mut rng, 3);

                                        assignments.push((chunk_id, replicas));
                                    }

                                    let response = CreateFileResponse {
                                        chunk_assginment: assignments,
                                    };

                                    let encoded = bincode::encode_to_vec(&response, bincode::config::standard())
                                        .expect("Failed to serialize response");
                                    if let Err(e) = socket.write_all(&encoded).await {
                                        error!("Failed to write response: {}", e);
                                    }
                                }
                            }
                        }

                        Err(e) => {
                            error!("Failed to decode MasterRequest: {}", e);
                        }
                    }
                }

                Ok(_) => {
                    warn!("Received empty message");
                }

                Err(e) => {
                    error!("Error reading from socket: {}", e);
                }
            }
        });
    }
}