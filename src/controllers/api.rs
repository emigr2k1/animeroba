use rocket::http::Status;
use rocket_contrib::json::Json;

use crate::db::PostgresDb;
use crate::models::{Anime, Episode, Server};

#[get("/api/animes?<page>&<quantity>")]
pub fn get_animes(db: PostgresDb, page: Option<u32>, quantity: Option<u32>) -> Json<Vec<Anime>> {
    let page = page.unwrap_or(1);
    let quantity = quantity.unwrap_or(1);
    let animes = Anime::from_page(&db, page as i64, quantity as i64).unwrap();
    Json(animes)
}

#[get("/api/animes/<code_name>")]
pub fn get_anime(db: PostgresDb, code_name: String) -> Result<Json<(Anime, Vec<Episode>)>, Status> {
    let anime = Anime::from_code_name(&db, &code_name).map_err(|_| Status::NotFound)?;
    let episodes = Episode::from_anime_id(&db, anime.id as i32).map_err(|_| Status::NotFound)?;
    Ok(Json((anime, episodes)))
}

#[get("/api/animes/<code_name>/<episode>")]
pub fn get_anime_episode(
    db: PostgresDb,
    code_name: String,
    episode: u32,
) -> Result<Json<(Anime, Episode, Vec<Server>)>, Status> {
    let anime = Anime::from_code_name(&db, &code_name).map_err(|_| Status::NotFound)?;
    let episode =
        Episode::from_number(&db, anime.id, episode as i32).map_err(|_| Status::NotFound)?;
    let servers = episode
        .servers(&db)
        .map_err(|_| Status::InternalServerError)?;
    Ok(Json((anime, episode, servers)))
}

//#[post("/api/animes", data = "<anime>")]
//pub fn post_anime(db: MongoDb, anime: Json<Anime>) -> String {
//    unimplemented!()
//}
