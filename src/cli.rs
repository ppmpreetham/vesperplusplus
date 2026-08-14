use std::sync::Arc;

use crate::api::server::serve_api;
use crate::config::get_username;
use crate::{config::init_config, fetcher::fetch_cli};
use clap::{Args, Parser, ValueEnum};

#[derive(ValueEnum, Clone, Debug)]
pub enum OutputFormat {
    Txt,
    Html,
    Pdf,
    Xlsx,
    Csv,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(flatten)]
    mode: Mode,

    /// Port for the server (only valid with --api)
    #[arg(short, long, requires = "api")]
    port: Option<u16>,

    // TODO
    /// Filter by site type (e.g., photo,~dating)
    #[arg(long, value_delimiter = ',')]
    filter: Vec<String>,

    // TODO
    /// Output a Prettified Document
    #[arg(short = 'o', long = "output", value_enum, default_value_t = OutputFormat::Txt)]
    format: OutputFormat,

    // TODO
    /// Optional AI results
    #[arg(long, value_enum)]
    ai: OutputFormat,
}

#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
pub struct Mode {
    /// Username of the account to enumerate
    pub username: Option<String>,

    /// API for special purposes (POST + SSE)
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub api: bool,
}

pub async fn run() {
    let cli = Cli::parse();

    match (cli.mode.api, cli.mode.username) {
        (true, _) => {
            serve_api(cli.port).await;
        }
        (false, Some(user)) => {
            init_config(user);
            fetch_cli().await;
        }
        _ => unreachable!(),
    }
}
