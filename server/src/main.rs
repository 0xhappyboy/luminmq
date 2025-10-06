use luminmq_server::server::LuminMQServer;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    LuminMQServer::start().await;
    Ok(())
}
