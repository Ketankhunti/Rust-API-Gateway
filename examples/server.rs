use rustway::{utils::config_path::Cli};
use rustway::run;

use clap::Parser;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {

    let cli = Cli::parse();

    run(cli.config).await

}