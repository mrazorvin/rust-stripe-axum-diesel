use axum::{extract::State, http::StatusCode, response::Json};
use diesel::{insert_into, ExpressionMethods, RunQueryDsl};
use serde::{Deserialize, Serialize};
use stripe::{
    CreatePaymentIntent, CreatePaymentIntentAutomaticPaymentMethods, Currency, PaymentIntent,
};

use crate::schema;

// https://github.com/arlyon/async-stripe/blob/master/examples/payment-intent.rs

#[derive(Serialize, Deserialize)]
pub struct PaymentIntentParams {
    currency: Currency,
    amount: i64,
}

pub async fn create_payment(
    State(storage): State<crate::KeysStorage>,
    Json(payload): Json<PaymentIntentParams>,
) -> Result<Json<PaymentIntent>, (StatusCode, String)> {
    use schema::payments::dsl::*;

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
        Ok(intent) => {
            let insert_result = insert_into(payments)
                .values((
                    created.eq(intent.created as i32),
                    amount.eq(intent.amount as i32),
                    currency.eq("usd"),
                    status.eq("wait_confirmation"),
                    method_type.eq("card"),
                    customer_id.eq(0),
                ))
                .execute(&mut crate::db());

            match insert_result {
                Ok(_) => Ok(Json(intent)),
                Err(error) => Err((StatusCode::BAD_REQUEST, error.to_string())),
            }
        }
        Err(error) => Err((StatusCode::BAD_REQUEST, error.to_string())),
    }
}
