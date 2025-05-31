#[tokio::main]
async fn main() -> anyhow::Result<()> {
    master::run().await
}
