use axum::{extract::Path, http::StatusCode, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BalanceInfo {
    customer_id: i32,
    balance: i32,
    currency: stripe::Currency,
}

pub async fn get_balance(
    Path(path): Path<super::UserPathSegment>,
) -> Result<Json<BalanceInfo>, (StatusCode, String)> {
    Ok(Json(BalanceInfo { customer_id: path.id, balance: 0, currency: stripe::Currency::USD }))
}
