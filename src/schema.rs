table! {
    animes (id) {
        id -> Int4,
        name -> Text,
        code_name -> Text,
        score -> Int4,
        synopsis -> Nullable<Text>,
        release_date -> Date,
        kind -> Int4,
        cover -> Nullable<Text>,
        status -> Int4,
        genres -> Array<Int4>,
        banner -> Nullable<Text>,
    }
}

table! {
    episodes (id) {
        id -> Int4,
        number -> Int4,
        anime_id -> Int4,
    }
}

table! {
    video_servers (id) {
        id -> Int4,
        name -> Text,
        url -> Text,
        episode_id -> Int4,
    }
}

joinable!(episodes -> animes (anime_id));
joinable!(video_servers -> episodes (episode_id));

allow_tables_to_appear_in_same_query!(
    animes,
    episodes,
    video_servers,
);
