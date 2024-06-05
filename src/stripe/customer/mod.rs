use serde::{Deserialize, Serialize};

mod get_balance;
mod model;

pub use get_balance::*;
pub use model::*;

#[derive(Serialize, Deserialize)]
pub struct UserPathSegment {
    id: i32,
}
