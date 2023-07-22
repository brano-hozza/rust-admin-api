// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Nullable<Uuid>,
        username -> Varchar,
        password_hash -> Varchar,
    }
}
