use axum::{extract::Path, http::StatusCode, Json};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};

use crate::stripe::Payment;

pub async fn get_payment(
    Path(path): Path<super::PaymentPathSegment>,
) -> Result<Json<crate::stripe::Payment>, (StatusCode, String)> {
    use crate::schema::payments::dsl::*;

    let result: Result<crate::stripe::Payment, diesel::result::Error> =
        payments.select(Payment::as_select()).filter(id.eq(path.id)).first(&mut crate::db());

    match result {
        Ok(payment) => Ok(Json(payment)),
        Err(error) => Err((StatusCode::BAD_REQUEST, error.to_string())),
    }
}
