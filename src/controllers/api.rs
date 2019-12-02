use rocket::http::Status;
use rocket_contrib::{
    databases::mongodb::{self, bson, db::ThreadedDatabase},
    json::Json,
};

use serde::{Deserialize, Serialize};

use crate::db::{MongoDb, PostgresDb};
use crate::models::{Anime, Episode, Server};

#[get("/api/animes?<page>")]
pub fn get_animes(db: PostgresDb, page: Option<u32>) -> Json<Vec<Anime>> {
    let animes = Anime::all(&db).unwrap();
    Json(animes)
}

#[get("/api/animes/<code_name>")]
pub fn get_anime(db: PostgresDb, code_name: String) -> Result<Json<(Anime, Vec<Episode>)>, Status> {
    let anime = Anime::from_code_name(&db, &code_name).map_err(|_| {
        // TODO: log
        Status::NotFound
    })?;
    let episodes = Episode::from_anime_id(&db, anime.id as i32).map_err(|_| Status::NotFound)?;
    Ok(Json((anime, episodes)))
}

#[post("/api/animes", data = "<anime>")]
pub fn post_anime(db: MongoDb, anime: Json<Anime>) -> String {
    unimplemented!()
}

#[get("/api/animes/<code_name>/<episode>")]
pub fn get_anime_episode(
    db: PostgresDb,
    code_name: String,
    episode: u32,
) -> Result<Json<(Anime, Episode, Vec<Server>)>, Status> {
    let anime = Anime::from_code_name(&db, &code_name).map_err(|_| Status::NotFound)?;
    let episode = Episode::from_number(&db, anime.id, episode as i32).map_err(|_| Status::NotFound)?;
    let servers = Server::from_episode_id(&db, episode.id).map_err(|_| Status::InternalServerError)?;
    Ok(Json((anime, episode, servers)))
}
