use std::str::FromStr;

use axum::{extract::Path, http::StatusCode, Json};
use diesel::{
    expression::{is_aggregate, ValidGrouping},
    prelude::*,
};
use serde::{Deserialize, Serialize};

use crate::schema;

#[derive(Serialize, Deserialize)]
pub struct BalanceInfo {
    currency: stripe::Currency,
    balance: Option<i64>,
}

impl ValidGrouping<(schema::customers::id, schema::payments::currency)>
    for schema::payments::currency
{
    type IsAggregate = is_aggregate::Yes;
}

pub async fn get_balance(
    Path(path): Path<super::UserPathSegment>,
) -> Result<Json<Vec<BalanceInfo>>, (StatusCode, String)> {
    // GROUP BY from multi tables not fully supported, check https://github.com/diesel-rs/diesel/issues/2544 for workaround
    let query: Result<Vec<(String, Option<i64>)>, diesel::result::Error> = schema::customers::table
        .inner_join(schema::payments::table.inner_join(schema::charge::table))
        .group_by((schema::customers::id, schema::payments::currency))
        .filter(schema::customers::id.eq(path.id))
        .select((schema::payments::currency, diesel::dsl::sum(schema::charge::amount)))
        .load(&mut crate::db());

    match query {
        Ok(balances) => {
            return Ok(Json(
                balances
                    .into_iter()
                    .map(|(currency, balance)| BalanceInfo {
                        balance,
                        currency: stripe::Currency::from_str(&currency).unwrap(),
                    })
                    .collect(),
            ))
        }
        Err(error) => {
            return Err((
                StatusCode::BAD_REQUEST,
                format!("can't fetch customer balance {}", error.to_string()),
            ))
        }
    }
}
