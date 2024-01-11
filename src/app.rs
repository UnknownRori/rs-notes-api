use std::sync::Arc;

use axum::{
    routing::{get, IntoMakeService},
    Extension, Router,
};
use color_eyre::Report;
use tokio::signal::ctrl_c;

use crate::controllers::hello_world::hello_world;

#[derive(Clone)]
struct AppState {}

impl Default for AppState {
    fn default() -> AppState {
        AppState {}
    }
}

pub struct App<'a> {
    host: &'a str,
    router: IntoMakeService<Router>,
    state: Arc<AppState>,
}

impl<'a> App<'a> {
    pub async fn new(host: &'a str) -> Result<App, Report> {
        let state = Arc::new(AppState::default());
        let state_clone = state.clone();
        let router = Router::new()
            .layer(Extension(state_clone))
            .route("/", get(hello_world))
            .into_make_service();

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
