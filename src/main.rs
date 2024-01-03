use anyhow::anyhow;
use oak_containers_orchestrator_client::OrchestratorClient;
use oak_user_silo_trusted::proto::user_silo::ApplicationConfig;
use prost::Message;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::net::TcpListener;

const TRUSTED_APP_PORT: u16 = 8080;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    println!("Starting Oak User Silo...");
    let mut client = OrchestratorClient::create()
        .await
        .map_err(|error| anyhow!("couldn't create Orchestrator client: {:?}", error))?;
    let application_config =
        ApplicationConfig::decode(client.clone().get_application_config().await?.as_slice())?;
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), TRUSTED_APP_PORT);
    let listener = TcpListener::bind(addr).await?;
    let join_handle = tokio::spawn(oak_user_silo_trusted::oak_user_silo::create(
        listener,
        client.clone(),
        application_config,
    ));
    println!("Notifying ready...");
    client.notify_app_ready().await?;
    println!("Awaiting finish...");
    join_handle.await??;
    println!("Finished...");
    Ok(())
}
