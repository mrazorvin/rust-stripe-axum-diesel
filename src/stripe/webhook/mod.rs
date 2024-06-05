use crate::KeysStorage;
use axum::{
    async_trait,
    body::Body,
    extract::{FromRef, FromRequest, Request},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use stripe::{Event, EventObject, EventType};

// https://github.com/arlyon/async-stripe/blob/master/examples/webhook-axum.rs

pub struct StripeEvent(Event);

#[async_trait]
impl<S> FromRequest<S> for StripeEvent
where
    String: FromRequest<S>,
    KeysStorage: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(req: Request<Body>, state: &S) -> Result<Self, Self::Rejection> {
        let signature = if let Some(sig) = req.headers().get("stripe-signature") {
            sig.to_owned()
        } else {
            return Err(StatusCode::BAD_REQUEST.into_response());
        };

        let ws_key = &*KeysStorage::from_ref(state).ws_key;
        let payload =
            String::from_request(req, state).await.map_err(IntoResponse::into_response)?;

        Ok(Self(
            stripe::Webhook::construct_event(&payload, signature.to_str().unwrap(), ws_key)
                .map_err(|_| StatusCode::BAD_REQUEST.into_response())?,
        ))
    }
}

pub async fn handle_webhooks(StripeEvent(event): StripeEvent) {
    match event.type_ {
        EventType::PaymentIntentSucceeded => {
            if let EventObject::PaymentIntent(session) = event.data.object {
                tracing::debug!(
                    "Payment intent succeeded {:?}",
                    serde_json::to_writer_pretty(std::io::stdout(), &session)
                );
            }
        }
        _ => println!("Unknown event encountered in webhook: {:?}", event.type_),
    }
}
