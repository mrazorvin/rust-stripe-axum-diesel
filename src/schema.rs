// @generated automatically by Diesel CLI.

diesel::table! {
    charge (id) {
        id -> Integer,
        amount -> Integer,
        created -> Integer,
        payment_id -> Integer,
    }
}

diesel::table! {
    customers (id) {
        id -> Integer,
        name -> Text,
        email -> Text,
        description -> Text,
    }
}

diesel::table! {
    payments (id) {
        id -> Integer,
        created -> Integer,
        amount -> Integer,
        currency -> Text,
        status -> Text,
        method_type -> Text,
        customer_id -> Integer,
    }
}

diesel::joinable!(charge -> payments (payment_id));
diesel::joinable!(payments -> customers (customer_id));
diesel::allow_tables_to_appear_in_same_query!(charge, customers, payments,);
