struct Comment {
    id: i32,
    user_id: i32,
    anime_id: i32,
    parent_id: Option<i32>,
    date: chrono::Date,
    content: String,
    votes: i32,
    edit_date: Option<chrono::Date>,
    is_deleted: bool,
}
