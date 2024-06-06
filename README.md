# Payment and Balance System

This is simple application where customer could initiate and confirm stripe payment, and check own balance.

To run server use `cargo run YOUR_STRIPE_TOKEN STRIP_WS_TOKEN`   
To build production executable`cargo build --release`

What was implemented: 
1. Axum server with router state and extractor
2. Stripe payment initialization and webhooks handling
3. Diesel queiry with aggregation from multiple 

The current implementation of the payment system has several limitations and missing features:
1. **Customer Management**: 
    - You cannot create or delete customers
2. **Authorization**: 
    - No efforts have been made to implement any kind of security.
3. **Currency Handling**: 
    - Currencies must be stored in a separate table with appropriate exchange rates. This allows for calculating balances in the desired currency, instead of maintaining balances on a per-currency basis.
4. **Partial Charges**: 
    - Partial charges are not supported.
5. **Webhook Handling**: 
    - The reaction to Stripe webhooks is missing because I didn't have a stable IP to expose my server to Stripe. Additionally, there wasn't enough time to fully understand the Stripe payment workflow.
6. **HTTP Error Handling**: 
    - I am not sure about handling HTTP errors. In previous work, we never manually sent code 500 and used only custom 400+ codes to represent domain-specific errors.


Diesel migrations will be applied by `build.rs` script

## Commands 
* Testing stripe webhooks
    * `stripe listen --forward-to http://localhost:3000/stripe_webhooks`
    * `stripe trigger payment_intent.succeeded`