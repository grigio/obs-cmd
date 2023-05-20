use anyhow::Result;
use obws::Client;

#[tokio::main]
async fn main() -> Result<()> {
    // Connect to the OBS instance through obs-websocket.
    let client = Client::connect("localhost", 4455, Some("secret")).await?;

    // Get and print out version information of OBS and obs-websocket.
    let version = client.general().version().await?;
    println!("{version:#?}");

    // Get a list of available scenes and print them out.
    let scene_list = client.scenes().list().await?;
    println!("{scene_list:#?}");

    Ok(())
}