use axum::{
    extract::Path,
    routing::{get, post, put},
    Router,
};
use serde::{Deserialize, Serialize};
use std::{env, net::SocketAddr, sync::Arc};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub mod schema;

mod connect_db;
mod stripe;

#[derive(Clone)]
pub struct KeysStorage {
    api_key: Arc<String>,
    ws_key: Arc<String>,
}

pub(crate) use connect_db::*;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::registry().with(tracing_subscriber::fmt::layer().pretty()).init();

    let Some(api_key) = env::args().skip(1).next() else {
        panic!("api key is required to run application`");
    };

    let Some(ws_key) = env::args().skip(2).next() else {
        panic!("ws key is required to run application`");
    };

    // custotomer -> payment intent -> charge -> find all charges

    // build our application with a route
    let app = Router::new()
        .route("/payment", put(crate::stripe::create_payment))
        .route("/payment/:id", get(crate::stripe::get_payment))
        .route("/payment/:id/confirm", post(placeholder_handler))
        // TODO request authorization
        .route("/customer/:id/balance", get(placeholder_handler))
        // https://github.com/arlyon/async-stripe/blob/master/examples/webhook-axum.rs
        .route("/webhooks/stripe", post(crate::stripe::handle_webhooks))
        .with_state(KeysStorage { api_key: Arc::new(api_key), ws_key: Arc::new(ws_key) });

    // run our app with hyper, listening globally on port 3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    tracing::debug!("listening on {addr}");
    axum::serve(listener, app).await.unwrap();
}

#[derive(Serialize, Deserialize)]
struct IdPath {
    id: i64,
}

async fn placeholder_handler(Path(path): Path<IdPath>) -> String {
    format!("placeholder router {}", path.id)
}
