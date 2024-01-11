use std::sync::Arc;

use tokio::sync::RwLock;

use crate::utils::env;

pub mod app;
pub mod controllers;
pub mod models;
pub mod router;
pub mod utils;

pub type SharedState<T> = Arc<RwLock<T>>;

/// Setup the application tracing and setting up environment variable
pub fn setup() -> Result<(), color_eyre::Report> {
    let _ = env("RUST_LIB_BACKTRACE", "1");
    let _ = env("RUST_LOG", "debug");

    color_eyre::install()?;

    tracing_subscriber::fmt::fmt()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(true)
        .pretty()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    tracing::debug!("Tracing is initialized!");

    Ok(())
}
