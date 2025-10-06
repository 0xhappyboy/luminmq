use luminmq_client::client::LuminMQClient;
use std::io::{self};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    LuminMQClient::start().await;
    Ok(())
}
