use std::sync::Arc;

use axum::{routing::IntoMakeService, Router};
use color_eyre::Report;
use tokio::{signal::ctrl_c, sync::RwLock};

use crate::{router::router, SharedState};

#[derive(Clone)]
struct AppState {}

impl Default for AppState {
    fn default() -> AppState {
        AppState {}
    }
}

#[allow(dead_code)]
pub struct App<'a> {
    host: &'a str,
    router: IntoMakeService<Router>,
    state: SharedState<AppState>,
}

impl<'a> App<'a> {
    pub async fn new(host: &'a str) -> Result<App, Report> {
        let state = Arc::new(RwLock::new(AppState::default()));
        let state_clone = state.clone();
        let router = router(state_clone);

        Ok(App {
            host,
            router,
            state,
        })
    }

    pub async fn serve(self) -> Result<(), Report> {
        let listener = tokio::net::TcpListener::bind(self.host).await?;
        let host_path = listener.local_addr().unwrap();
        tracing::info!("Server is running at {}", host_path);

        let router = self.router;

        axum::serve(listener, router)
            .with_graceful_shutdown(App::shutdown_signal())
            .await?;

        Ok(())
    }

    async fn shutdown_signal() {
        let _ = ctrl_c().await;
        tracing::info!("Shutting down server gracefully...");
    }
}
