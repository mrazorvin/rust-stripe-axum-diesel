// @generated automatically by Diesel CLI.

diesel::table! {
    payment_intents (id) {
        id -> Integer,
        created -> Integer,
        amount -> Integer,
        currency -> Text,
        method_type -> Text,
        status -> Text,
    }
}
