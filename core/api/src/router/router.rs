use crate::middleware::cors_middleware::cors_middleware;

use salvo::{handler, Router};

// use super::plugin::get_plugin_router;

pub async fn get_all_route() -> Router {
    Router::new()
        .push(Router::with_path("api").hoop(cors_middleware()))
        .push(Router::with_path("health").get(hello))
    // .push(get_plugin_router())
}

#[handler]
async fn hello() -> &'static str {
    "Hello World!"
}
