#![feature(async_fn_in_trait)]

use clap::Parser;
use std::path::PathBuf;

pub mod server;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Parser)]
#[command(name = "chatgpt")]
#[command(author = "Kevin K. <codinlog@foxmail.com>")]
#[command(version = "1.0")]
#[command(about = "Hi (~_~)", long_about = None)]
struct Cli {
    /// gpt auth key
    #[arg(long, value_name = "STRING")]
    key: Option<String>,

    /// api file
    #[arg(long, value_name = "FILE")]
    api: Option<PathBuf>,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
}

fn auth() -> Result<String> {
    let key = env!("ENV_CHATGPT_AUTH_KEY");

    Ok(key.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_cli() -> Result<()> {
        use clap::CommandFactory;
        Cli::command().debug_assert();

        Ok(())
    }
}
