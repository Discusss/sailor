// @generated automatically by Diesel CLI.

diesel::table! {
    keys (key) {
        key -> Text,
        created_at -> Timestamp,
        expires_at -> Timestamp,
        last_used_at -> Timestamp,
        owner -> Text,
        uses -> Int4,
        ips -> Array<Nullable<Text>>,
        user_agent -> Text,
        created_by -> Text,
        notes -> Text,
    }
}

diesel::table! {
    links (id) {
        id -> Int4,
        domain -> Text,
        #[max_length = 255]
        category -> Varchar,
        priority -> Int4,
        public_notes -> Text,
        submitted_by -> Text,
        submitted_at -> Nullable<Timestamp>,
        submitted_ip -> Nullable<Text>,
        submitted_user_agent -> Nullable<Text>,
        submitted_reason -> Text,
        approved_by -> Nullable<Text>,
        approved_at -> Nullable<Timestamp>,
        approved_key -> Text,
        notes -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    keys,
    links,
);
