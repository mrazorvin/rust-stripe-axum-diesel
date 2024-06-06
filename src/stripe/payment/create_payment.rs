use std::str::FromStr;

use axum::{extract::State, http::StatusCode, response::Json};
use diesel::{insert_into, ExpressionMethods, RunQueryDsl};
use serde::{Deserialize, Serialize};
use stripe::{
    CreatePaymentIntent, CreatePaymentIntentAutomaticPaymentMethods, Currency, PaymentIntent,
};

use crate::{schema, stripe::Payment};

// https://github.com/arlyon/async-stripe/blob/master/examples/payment-intent.rs

#[derive(Serialize, Deserialize)]
pub struct PaymentIntentParams {
    currency: Currency,
    amount: i64,
}

pub async fn create_payment(
    State(storage): State<crate::KeysStorage>,
    Json(payload): Json<PaymentIntentParams>,
) -> Result<Json<Payment>, (StatusCode, String)> {
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
            let mut new_payment: Payment = Payment {
                id: 0,
                amount: intent.amount as i32,
                created: intent.created as i32,
                currency: intent.currency.to_string(),
                status: String::from("wait_confirmation"),
                customer_id: 0,
                method_type: String::from("card"),
            };

            let insert_result = insert_into(payments)
                .values((
                    created.eq(new_payment.created),
                    amount.eq(new_payment.amount),
                    currency.eq(new_payment.currency.to_string()),
                    status.eq(&new_payment.status),
                    method_type.eq(&new_payment.method_type),
                    customer_id.eq(new_payment.customer_id),
                ))
                .execute(&mut crate::db());

            match insert_result {
                Ok(new_payment_id) => {
                    new_payment.id = new_payment_id as i32;
                    Ok(Json(new_payment))
                }
                Err(error) => Err((StatusCode::BAD_REQUEST, error.to_string())),
            }
        }
        Err(error) => Err((StatusCode::BAD_REQUEST, error.to_string())),
    }
}
