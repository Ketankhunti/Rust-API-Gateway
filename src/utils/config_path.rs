use std::{path::PathBuf};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, value_name = "FILE", default_value = "gateway_config.yaml")]
    pub config: PathBuf,
}