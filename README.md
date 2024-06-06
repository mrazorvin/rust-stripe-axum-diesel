# Rust + Diesel + Sqlite example

To run server use `cargo run YOUR_STRIPE_TOKEN STRIP_WS_TOKEN`   
To build production executable`cargo build --release`

Diesel migrations will be applied by `build.rs` script

## Commands 
* Testing stripe webhooks
    * `stripe listen --forward-to http://localhost:3000/stripe_webhooks`
    * `stripe trigger payment_intent.succeeded`