// @generated automatically by Diesel CLI.

diesel::table! {
    charge (id) {
        id -> Integer,
    }
}

diesel::table! {
    customers (id) {
        id -> Integer,
    }
}

diesel::table! {
    payments (id) {
        id -> Integer,
        created -> Integer,
        amount -> Integer,
        currency -> Text,
        method_type -> Text,
        status -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(charge, customers, payments,);