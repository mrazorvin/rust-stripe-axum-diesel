use axum::extract::Path;

use super::PaymentPathSegment;

async fn placeholder_handler(Path(path): Path<PaymentPathSegment>) -> String {
    format!("placeholder router {}", path.id)
}
