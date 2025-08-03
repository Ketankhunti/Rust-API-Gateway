use api_gateway::{utils::config_path::Cli};
use api_gateway::run;

use clap::Parser;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {

    let cli = Cli::parse();

    run(cli.config).await

}