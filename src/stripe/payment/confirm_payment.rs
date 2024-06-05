use axum::{extract::Path, http::StatusCode, Json};
use chrono::Utc;
use diesel::prelude::*;

use crate::stripe::Payment;

pub async fn confirm_payment(
    Path(path): Path<super::PaymentPathSegment>,
) -> Result<Json<crate::stripe::Payment>, (StatusCode, String)> {
    use crate::schema::payments::dsl::*;

    let result: Result<crate::stripe::Payment, diesel::result::Error> =
        payments.select(Payment::as_select()).filter(id.eq(path.id)).first(&mut crate::db());
    let Ok(mut payment) = result else {
        return Err((StatusCode::BAD_REQUEST, result.err().unwrap().to_string()));
    };

    if payment.status != "wait_confirmation" {
        return Err((
            StatusCode::BAD_REQUEST,
            format!("Payment with id:{} already confirmed", payment.customer_id),
        ));
    }

    let charge_id = crate::db().transaction(|conn| {
        diesel::update(payments.filter(id.eq(path.id))).set(status.eq("done")).execute(conn)?;
        {
            use crate::schema::charge::dsl::*;
            let charge_id = diesel::insert_into(crate::schema::charge::dsl::charge)
                .values((
                    amount.eq(payment.amount),
                    created.eq(Utc::now().timestamp_millis() as i32),
                    payment_id.eq(payment.id),
                ))
                .execute(&mut crate::db())? as i32;

            return diesel::QueryResult::Ok(charge_id);
        }
    });

    match charge_id {
        Ok(_) => {
            payment.status = String::from("done");
            return Ok(Json(payment));
        }
        Err(error) => {
            return Err((
                StatusCode::BAD_REQUEST,
                format!(
                    "Can't confirm payment for id {} because of {}",
                    payment.customer_id,
                    error.to_string()
                ),
            ));
        }
    }
}
