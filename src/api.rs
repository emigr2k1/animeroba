use rocket::{
    request::{Form, FromForm},
    http::Status,
};
use rocket_contrib::databases::mongodb::{self, bson, db::ThreadedDatabase};
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

use crate::db::Db;

#[derive(Serialize, Deserialize)]
pub struct Anime {
    pub code_name: String,
    pub title: String,
    pub synopsis: String,
    pub score: f32,
    pub release_date: String,
    pub kind: String,
    pub cover: String,
    pub status: String,
    pub genres: Vec<String>,
    pub episodes: Vec<Episode>,
}

#[derive(Serialize, Deserialize)]
pub struct Episode {
    pub number: f32,
    pub servers: Vec<Server>,
}

#[derive(Serialize, Deserialize, FromForm)]
pub struct Server {
    pub name: String,
    pub url: String,
}

#[get("/api/animes?<page>")]
pub fn get_animes(db: Db, page: Option<u32>) -> Json<Vec<Anime>> {
    let page = page.unwrap_or(1);

    let col = db.collection("animes");
    let mut cursor = col.find(None, None).unwrap();

    let mut animes = Vec::with_capacity(30);
    for anime in cursor {
        if let Ok(item) = anime {
            let anime = mongodb::from_bson(bson::Bson::Document(item)).unwrap();
            animes.push(anime);
        }
    }
    Json(animes)
}

#[post("/api/animes", data = "<anime>")]
pub fn post_anime(db: Db, anime: Json<Anime>) -> String {
    let mut col = db.collection("animes");
    let anime_bson = bson::to_bson(&anime.into_inner()).unwrap();
    let anime_doc = match anime_bson {
        bson::Bson::Document(doc) => doc,
        _ => return "Not a document".to_string(),
    };
    col.insert_one(anime_doc, None)
        .map(|_| "Added".to_string())
        .unwrap()
}

#[get("/api/animes/<code_name>")]
pub fn get_anime(db: Db, code_name: String) -> Result<Json<Anime>, Status> {
    use mongodb::doc;
    let col = db.collection("animes");
    let anime_bson = col
        .find_one(Some(doc! {"code_name": code_name}), None)
        .map_err(|_| Status::InternalServerError)?
        .ok_or(Status::NotFound)?;
    let anime = bson::from_bson(bson::Bson::Document(anime_bson)).map_err(|_|Status::InternalServerError)?;
    Ok(Json(anime))
}

#[put("/api/animes/<name>", data = "<anime>")]
pub fn put_anime(name: String, anime: Json<Anime>) -> String {
    unimplemented!()
}

#[post("/api/animes/<name>", data = "<episode>")]
pub fn post_anime_episode(name: String, episode: Json<Episode>) -> String {
    unimplemented!()
}

#[get("/api/animes/<code_name>/<episode>")]
pub fn get_anime_episode(db: Db, code_name: String, episode: u32) -> Result<Json<Anime>, Status> {
    use mongodb::doc;
    let col = db.collection("animes");
    let anime_bson = col
        .find_one(Some(doc! {"code_name": code_name}), None)
        .map_err(|_| Status::InternalServerError)?
        .ok_or(Status::NotFound)?;
    let anime = bson::from_bson(bson::Bson::Document(anime_bson)).map_err(|_|Status::InternalServerError)?;
    Ok(Json(anime))
}
