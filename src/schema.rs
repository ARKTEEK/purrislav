// @generated automatically by Diesel CLI.

diesel::table! {
    birthdays (id) {
        id -> Integer,
        user_id -> BigInt,
        date -> Date,
    }
}
