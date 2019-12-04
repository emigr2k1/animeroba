use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::convert::TryInto;

use super::{Anime, Server};
use crate::db::PostgresDb;
use crate::schema::episodes as dsl;

#[derive(Serialize, Deserialize)]
pub struct LatestEpisode {
    pub episode_id: i32,
    pub number: i32,
    pub anime_id: i32,
    pub anime_name: String,
    pub anime_code_name: String,
    pub anime_cover: Option<String>,
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct Episode {
    pub id: i32,
    pub number: i32,
    pub anime_id: i32,
}

impl Episode {
    pub fn from_number(
        db: &PostgresDb,
        anime_id: i32,
        number: i32,
    ) -> Result<Episode, Box<dyn Error>> {
        let result = dsl::table
            .select(dsl::all_columns)
            .filter(dsl::anime_id.eq(anime_id))
            .filter(dsl::number.eq(number))
            .first::<Self>(&**db)?;
        Ok(result)
    }

    pub fn from_anime_id(db: &PostgresDb, anime_id: i32) -> Result<Vec<Self>, Box<dyn Error>> {
        let results = dsl::table
            .select(dsl::all_columns)
            .filter(dsl::anime_id.eq(anime_id))
            .load::<Self>(&**db)?;
        Ok(results)
    }

    pub fn count(db: &PostgresDb, anime_id: i32) -> Result<i64, Box<dyn Error>> {
        let result = dsl::table
            .select(diesel::dsl::count(dsl::id))
            .filter(dsl::anime_id.eq(anime_id))
            .first(&**db)?;
        Ok(result)
    }

    pub fn servers(&self, db: &PostgresDb) -> Result<Vec<Server>, Box<dyn Error>> {
        Ok(Server::from_episode_id(db, self.id)?)
    }

    pub fn latest_n(db: &PostgresDb, n: u64) -> Result<Vec<LatestEpisode>, Box<dyn Error>> {
        use crate::schema::animes as anime_dsl;

        let latest_eps = dsl::table
            .select(dsl::all_columns)
            .order(dsl::id.desc())
            .limit(n.try_into().unwrap_or(24))
            .load::<Self>(&**db)?;

        let mut animes = Vec::with_capacity(n as usize);
        for ep in &latest_eps {
            let anime = anime_dsl::table
                .select((
                    anime_dsl::id,
                    anime_dsl::name,
                    anime_dsl::code_name,
                    anime_dsl::cover,
                ))
                .filter(anime_dsl::id.eq(ep.anime_id))
                .first::<(i32, String, String, Option<String>)>(&**db)?;
            animes.push(anime);
        }

        // Zip Item: (Episode, (i32, String, String, Option<String>))
        let latest_eps = latest_eps
            .into_iter()
            .zip(animes)
            .map(|v| LatestEpisode {
                episode_id: v.0.id,
                number: v.0.number,
                anime_id: (v.1).0,
                anime_name: (v.1).1,
                anime_code_name: (v.1).2,
                anime_cover: (v.1).3,
            })
            .collect();

        Ok(latest_eps)
    }
}
