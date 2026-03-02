// @generated automatically by Diesel CLI.

diesel::table! {
    collectibles (id) {
        id -> Text,
        name -> Text,
        kind -> Text,
        release_date -> Nullable<Date>,
        version -> Nullable<Text>,
        barcode -> Nullable<Text>,
        image -> Nullable<Text>,
        verified -> Bool,
        group_id -> Text,
    }
}

diesel::table! {
    groups (id) {
        id -> Text,
        name -> Text,
        agency -> Text,
        debut_year -> Integer,
        is_active -> Bool,
    }
}

diesel::joinable!(collectibles -> groups (group_id));

diesel::allow_tables_to_appear_in_same_query!(collectibles, groups,);
