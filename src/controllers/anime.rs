use rocket::http::Status;
use rocket_contrib::templates::{tera::Context, Template};

use crate::db::PostgresDb;
use crate::models::Anime;

#[get("/anime/<code_name>")]
pub fn get_anime(db: PostgresDb, code_name: String) -> Result<Template, Status> {
    let anime = Anime::from_code_name(&db, &code_name).map_err(|_| Status::NotFound)?;
    let num_episodes = anime
        .count_episodes(&db)
        .map_err(|_| Status::InternalServerError)?;
    let mut context = Context::new();
    context.insert("anime", &anime);
    context.insert("num_episodes", &num_episodes);
    context.insert("genres_str", &anime.genres_str());
    Ok(Template::render("anime", context))
}
