use rocket::response::content;

#[catch(404)]
pub fn not_found() -> content::Html<&'static str> {
    content::Html("<h1>404!</h1> page not found")
}
