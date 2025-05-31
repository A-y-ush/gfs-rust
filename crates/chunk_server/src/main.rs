#[tokio::main]
async fn main() -> anyhow::Result<()> {
    chunk_server::run().await
}
