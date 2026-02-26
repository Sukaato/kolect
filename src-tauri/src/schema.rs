// @generated automatically by Diesel CLI.

diesel::table! {
    albums (id) {
        id -> Nullable<Text>,
        group_id -> Text,
        title -> Text,
        kind -> Nullable<Text>,
        release_date -> Nullable<Text>,
        cover_image -> Nullable<Text>,
        barcode -> Nullable<Text>,
        verified -> Bool,
    }
}

diesel::table! {
    dataset_metadata (rowid) {
        rowid -> Integer,
        version -> Text,
        generated_at -> Text,
        fetched_at -> Integer,
    }
}

diesel::table! {
    groups (id) {
        id -> Nullable<Text>,
        name -> Text,
        agency -> Nullable<Text>,
        debut_year -> Nullable<Integer>,
        is_active -> Bool,
    }
}

diesel::table! {
    lightsticks (id) {
        id -> Nullable<Text>,
        group_id -> Text,
        name -> Nullable<Text>,
        version -> Nullable<Text>,
        release_year -> Nullable<Integer>,
        image -> Nullable<Text>,
        verified -> Bool,
    }
}

diesel::table! {
    user_collection (id) {
        id -> Nullable<Text>,
        product_id -> Text,
        product_type -> Nullable<Text>,
        added_at -> Integer,
    }
}

diesel::joinable!(albums -> groups (group_id));
diesel::joinable!(lightsticks -> groups (group_id));

diesel::allow_tables_to_appear_in_same_query!(
    albums,
    dataset_metadata,
    groups,
    lightsticks,
    user_collection,
);
