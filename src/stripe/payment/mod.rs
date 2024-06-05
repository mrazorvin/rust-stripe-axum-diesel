use serde::{Deserialize, Serialize};

mod create_payment;
mod get_payment;
mod model;

pub(crate) use create_payment::*;
pub(crate) use get_payment::*;
pub(crate) use model::*;

#[derive(Serialize, Deserialize)]
pub(crate) struct PaymentPathSegment {
    id: i32,
}
