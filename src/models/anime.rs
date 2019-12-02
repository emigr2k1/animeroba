use postgres::{rows::Row, types::FromSql};
use rocket::request::FromForm;
use serde::{Deserialize, Serialize};
use std::error::Error;

use super::{db_utils::get_cell, Episode};

use crate::db::PostgresDb;

#[derive(Debug, Serialize, Deserialize)]
pub struct Anime {
    pub id: i32,
    pub name: String,
    pub code_name: String,
    pub score: i32,
    pub synopsis: String,
    pub release_date: chrono::NaiveDate,
    pub kind: i32,
    pub cover: String,
    pub status: i32,
    pub genres: Vec<i32>,
}

impl Anime {
    pub fn all(db: &PostgresDb) -> Result<Vec<Self>, Box<dyn Error>> {
        let query = db.query(r#"
            SELECT
                a.id as "anime.id", a.name as "anime.name", a.code_name as "anime.code_name",
                a.score as "anime.score", a.synopsis as "anime.synopsis",
                a.release_date as "anime.release_date", a.kind as "anime.kind", a.cover as "anime.cover",
                a.status as "anime.status", a.genres as "anime.genres"
            FROM animes as a"#, &[])?;
        let mut animes = Vec::new();
        for row in &query {
            let anime = Self::from_row(&row)?;
            animes.push(anime);
        }
        Ok(animes)
    }

    pub fn from_code_name(db: &PostgresDb, code_name: &str) -> Result<Self, Box<dyn Error>> {
        let query = db.query(r#"
            SELECT
                a.id as "anime.id", a.name as "anime.name", a.code_name as "anime.code_name",
                a.score as "anime.score", a.synopsis as "anime.synopsis",
                a.release_date as "anime.release_date", a.kind as "anime.kind", a.cover as "anime.cover",
                a.status as "anime.status", a.genres as "anime.genres"
            FROM animes as a
            WHERE a.code_name = $1"#, &[&code_name])?;
        if let Some(row) = query.iter().next() {
            Ok(Self::from_row(&row)?)
        } else {
            Err(format!("postgres: anime with code_name: {} not found", code_name).into())
        }
    }

    pub fn episodes(&self, db: &PostgresDb) -> Result<Vec<Episode>, Box<dyn Error>> {
        Ok(Episode::from_anime_id(db, self.id)?)
    }

    pub fn episode(&self, db: &PostgresDb, episode_number: i32) -> Result<Episode, Box<dyn Error>> {
        Ok(Episode::from_number(db, self.id, episode_number)?)
    }

    fn from_row(row: &Row) -> Result<Anime, Box<dyn Error>> {
        Ok(Self {
            id: get_cell(&row, "anime.id")?,
            name: get_cell(&row, "anime.name")?,
            code_name: get_cell(&row, "anime.code_name")?,
            score: get_cell(&row, "anime.score")?,
            synopsis: get_cell(&row, "anime.synopsis")?,
            release_date: get_cell(&row, "anime.release_date")?,
            kind: get_cell(&row, "anime.kind")?,
            cover: get_cell(&row, "anime.cover")?,
            status: get_cell(&row, "anime.status")?,
            genres: get_cell(&row, "anime.genres")?,
        })
    }
}
