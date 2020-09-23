table! {
    auths (user_id, token) {
        user_id -> Text,
        token -> Text,
    }
}

table! {
    users (id) {
        id -> Text,
        name -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    auths,
    users,
);
