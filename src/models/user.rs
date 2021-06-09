pub struct User {
    id: i32,
    nickname: String,
    name: String,
    email: String,
    pic: Option<String>,
    description: Option<String>,
    birthday: Option<chrono::Date<chrono::Utc>>,
    location: Option<String>,
    country: Option<i32>, // Should be casted to a country enum
    join_date: chrono::Date<chrono::Utc>,
    reputation: f32,
    ip: String,
    password: String,
    role: i32, // Should be casted to a role enum
}
