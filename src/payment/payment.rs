use axum::{http::StatusCode, response::Json};
use serde::{Deserialize, Serialize};
use stripe::{CreatePaymentIntent, Currency, PaymentIntent};

#[derive(Serialize, Deserialize)]
pub(crate) struct PaymentIntentParams {
    api_key: String,
    amount: i64,
}

pub async fn create_payment_intent(
    Json(payload): Json<PaymentIntentParams>,
) -> Result<Json<PaymentIntent>, (StatusCode, String)> {
    let client = stripe::Client::new(payload.api_key);

    // we create an intent to pay
    let payment_intent = {
        let mut create_intent = CreatePaymentIntent::new(payload.amount, Currency::USD);
        create_intent.payment_method_types = Some(vec!["card".to_string()]);

        PaymentIntent::create(&client, create_intent).await
    };

    match payment_intent {
        Ok(intent) => Ok(Json(intent)),
        Err(error) => Err((StatusCode::BAD_REQUEST, error.to_string())),
    }
}
