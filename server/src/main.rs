use std::{collections::HashMap, thread, time::Duration};

use luminmq_server::{
    group::{Group, Groups},
    msg::Message,
    server::LuminMQServer,
    topic::Topic,
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    LuminMQServer::start().await;
    Ok(())
}
