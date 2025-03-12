// @generated automatically by Diesel CLI.

diesel::table! {
    birthdays (id) {
        id -> Integer,
        user_id -> BigInt,
        date -> Date,
        announced_this_year -> Bool,
    }
}

diesel::table! {
    guild_settings (id) {
        id -> Integer,
        guild_id -> BigInt,
        announcements_channel_id -> Nullable<BigInt>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    birthdays,
    guild_settings,
);
