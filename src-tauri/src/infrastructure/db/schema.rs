// @generated automatically by Diesel CLI.

diesel::table! {
    agencies (id) {
        id -> Text,
        name -> Text,
        country -> Text,
        image_url -> Nullable<Text>,
        is_deleted -> Integer,
    }
}

diesel::table! {
    album_versions (id) {
        id -> Text,
        album_id -> Text,
        name -> Text,
        format -> Text,
        release_date -> Text,
        region -> Text,
        image_url -> Nullable<Text>,
        is_deleted -> Integer,
    }
}

diesel::table! {
    albums (id) {
        id -> Text,
        name -> Text,
        release_date -> Text,
        region -> Text,
        image_url -> Nullable<Text>,
        group_id -> Nullable<Text>,
        artist_id -> Nullable<Text>,
        is_deleted -> Integer,
    }
}

diesel::table! {
    artist_aliases (id) {
        id -> Text,
        artist_id -> Text,
        name -> Text,
        kind -> Text,
        is_primary -> Integer,
        is_deleted -> Integer,
    }
}

diesel::table! {
    artists (id) {
        id -> Text,
        real_name -> Text,
        birth_date -> Nullable<Text>,
        image_url -> Nullable<Text>,
        solo_debut_date -> Nullable<Text>,
        solo_agency_id -> Nullable<Text>,
        is_deleted -> Integer,
    }
}

diesel::table! {
    digipacks (id) {
        id -> Text,
        album_id -> Text,
        artist_id -> Nullable<Text>,
        name -> Text,
        release_date -> Text,
        region -> Text,
        image_url -> Nullable<Text>,
        is_deleted -> Integer,
    }
}

diesel::table! {
    fanclub_kits (id) {
        id -> Text,
        group_id -> Nullable<Text>,
        artist_id -> Nullable<Text>,
        name -> Text,
        release_date -> Text,
        region -> Text,
        image_url -> Nullable<Text>,
        is_deleted -> Integer,
    }
}

diesel::table! {
    group_members (artist_id, group_id) {
        artist_id -> Text,
        group_id -> Text,
        roles -> Text,
        join_date -> Nullable<Text>,
        leave_date -> Nullable<Text>,
    }
}

diesel::table! {
    groups (id) {
        id -> Text,
        name -> Text,
        debut_date -> Text,
        fandom_name -> Nullable<Text>,
        image_url -> Nullable<Text>,
        agency_id -> Text,
        is_deleted -> Integer,
    }
}

diesel::table! {
    lightsticks (id) {
        id -> Text,
        group_id -> Nullable<Text>,
        artist_id -> Nullable<Text>,
        name -> Text,
        version -> Text,
        release_date -> Text,
        image_url -> Nullable<Text>,
        is_deleted -> Integer,
    }
}

diesel::table! {
    photocards (id) {
        id -> Text,
        artist_id -> Nullable<Text>,
        album_id -> Nullable<Text>,
        album_version_id -> Nullable<Text>,
        digipack_id -> Nullable<Text>,
        release_date -> Text,
        region -> Text,
        image_url -> Nullable<Text>,
        is_deleted -> Integer,
    }
}

diesel::table! {
    user_collection (id) {
        id -> Text,
        album_id -> Nullable<Text>,
        album_version_id -> Nullable<Text>,
        digipack_id -> Nullable<Text>,
        lightstick_id -> Nullable<Text>,
        fanclub_kit_id -> Nullable<Text>,
        photocard_id -> Nullable<Text>,
        acquired_at -> Text,
        notes -> Nullable<Text>,
        is_signed -> Integer,
    }
}

diesel::table! {
    user_favorites_artists (artist_id) {
        artist_id -> Nullable<Text>,
    }
}

diesel::table! {
    user_favorites_groups (group_id) {
        group_id -> Nullable<Text>,
    }
}

diesel::joinable!(album_versions -> albums (album_id));
diesel::joinable!(albums -> artists (artist_id));
diesel::joinable!(albums -> groups (group_id));
diesel::joinable!(artist_aliases -> artists (artist_id));
diesel::joinable!(artists -> agencies (solo_agency_id));
diesel::joinable!(digipacks -> albums (album_id));
diesel::joinable!(digipacks -> artists (artist_id));
diesel::joinable!(fanclub_kits -> artists (artist_id));
diesel::joinable!(fanclub_kits -> groups (group_id));
diesel::joinable!(group_members -> artists (artist_id));
diesel::joinable!(group_members -> groups (group_id));
diesel::joinable!(groups -> agencies (agency_id));
diesel::joinable!(lightsticks -> artists (artist_id));
diesel::joinable!(lightsticks -> groups (group_id));
diesel::joinable!(photocards -> album_versions (album_version_id));
diesel::joinable!(photocards -> albums (album_id));
diesel::joinable!(photocards -> artists (artist_id));
diesel::joinable!(photocards -> digipacks (digipack_id));
diesel::joinable!(user_collection -> album_versions (album_version_id));
diesel::joinable!(user_collection -> albums (album_id));
diesel::joinable!(user_collection -> digipacks (digipack_id));
diesel::joinable!(user_collection -> fanclub_kits (fanclub_kit_id));
diesel::joinable!(user_collection -> lightsticks (lightstick_id));
diesel::joinable!(user_collection -> photocards (photocard_id));
diesel::joinable!(user_favorites_artists -> artists (artist_id));
diesel::joinable!(user_favorites_groups -> groups (group_id));

diesel::allow_tables_to_appear_in_same_query!(
    agencies,
    album_versions,
    albums,
    artist_aliases,
    artists,
    digipacks,
    fanclub_kits,
    group_members,
    groups,
    lightsticks,
    photocards,
    user_collection,
    user_favorites_artists,
    user_favorites_groups,
);
