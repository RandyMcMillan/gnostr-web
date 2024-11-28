use std::sync::Arc;

mod state;
use env_logger::Env;
use log::{error, info};
use state::State;
use tokio::sync::Mutex;

mod config;
mod git;
mod site;
mod ssh;
mod utils;
mod vars;

async fn start() -> anyhow::Result<()> {
    info!("Loading state...");
    let state = State::new().await?;
    let state = Arc::new(Mutex::new(state));

    info!("Starting server...");
    let _ = sd_notify::notify(true, &[sd_notify::NotifyState::Ready]);
    ssh::start_server(state).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    if let Err(e) = start().await {
        error!("{:#}", e);
    }
}
