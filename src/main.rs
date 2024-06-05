use std::{env, net::SocketAddr};

use axum::{
    routing::{get, post},
    Router,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod payment;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::registry().with(tracing_subscriber::fmt::layer().pretty()).init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/create_payment_intent", post(payment::create_payment_intent));

    // run our app with hyper, listening globally on port 3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    tracing::debug!("listening on {addr}");
    axum::serve(listener, app).await.unwrap();
}
