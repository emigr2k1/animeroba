use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::error::Error;

use super::Episode;
use crate::db::PostgresDb;
use crate::schema::animes::{self, dsl};

#[derive(Serialize, Deserialize, Queryable)]
pub struct Anime {
    pub id: i32,
    pub name: String,
    pub code_name: String,
    pub score: i32,
    pub synopsis: Option<String>,
    pub release_date: chrono::NaiveDate,
    pub kind: i32,
    pub cover: Option<String>,
    pub status: i32,
    pub genres: Vec<i32>,
    pub banner: Option<String>,
}

impl Anime {
    pub fn from_page(
        db: &PostgresDb,
        page: i64,
        quantity: i64,
    ) -> Result<Vec<Self>, Box<dyn Error>> {
        let results = dsl::animes
            .select(animes::all_columns)
            .offset(page * quantity - quantity)
            .limit(quantity)
            .load::<Anime>(&**db)?;
        Ok(results)
    }

    pub fn from_code_name(db: &PostgresDb, code_name: &str) -> Result<Self, Box<dyn Error>> {
        Ok(dsl::animes
            .filter(dsl::code_name.eq(code_name))
            .first(&**db)?)
    }

    pub fn count_episodes(&self, db: &PostgresDb) -> Result<i64, Box<dyn Error>> {
        Ok(Episode::count(db, self.id)?)
    }

    pub fn episodes(&self, db: &PostgresDb) -> Result<Vec<Episode>, Box<dyn Error>> {
        Ok(Episode::from_anime_id(db, self.id)?)
    }

    pub fn episode(&self, db: &PostgresDb, episode_number: i32) -> Result<Episode, Box<dyn Error>> {
        Ok(Episode::from_number(db, self.id, episode_number)?)
    }

    pub fn genres_str(&self) -> Vec<String> {
        use super::Genre;
        use num_traits::FromPrimitive;
        self.genres
            .iter()
            .filter_map(|g| Genre::from_i32(*g))
            .map(|g| g.to_string())
            .collect()
    }
}
