use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::Json};
use serde::{Deserialize, Serialize};
use stripe::{
    CreatePaymentIntent, CreatePaymentIntentAutomaticPaymentMethods, Currency, PaymentIntent,
};

// https://github.com/arlyon/async-stripe/blob/master/examples/payment-intent.rs

#[derive(Serialize, Deserialize)]
pub(crate) struct PaymentIntentParams {
    currency: Currency,
    amount: i64,
}
// TODO client, charge, methods

pub async fn create_payment_intent(
    State(storage): State<crate::KeysStorage>,
    Json(payload): Json<PaymentIntentParams>,
) -> Result<Json<PaymentIntent>, (StatusCode, String)> {
    let client = stripe::Client::new(&*storage.api_key);

    let payment_intent = {
        let mut create_intent = CreatePaymentIntent::new(payload.amount, payload.currency);
        create_intent.automatic_payment_methods =
            Some(CreatePaymentIntentAutomaticPaymentMethods {
                enabled: true,
                ..Default::default()
            });
        PaymentIntent::create(&client, create_intent).await
    };

    match payment_intent {
        Ok(intent) => Ok(Json(intent)),
        Err(error) => Err((StatusCode::BAD_REQUEST, error.to_string())),
    }
}
