use serde::{Deserialize, Serialize};

mod confirm_payment;
mod create_payment;
mod get_payment;
mod model;

pub use confirm_payment::*;
pub use create_payment::*;
pub use get_payment::*;
pub use model::*;

#[derive(Serialize, Deserialize)]
pub struct PaymentPathSegment {
    id: i32,
}
