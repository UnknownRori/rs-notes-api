use axum::{
    routing::{get, IntoMakeService},
    Extension, Router,
};

use crate::{controllers::hello_world::hello_world, SharedState};

pub fn router<T>(state: SharedState<T>) -> IntoMakeService<Router>
where
    T: std::marker::Send + std::marker::Sync + 'static,
{
    Router::new()
        .layer(Extension(state))
        .route("/", get(hello_world))
        .into_make_service()
}
