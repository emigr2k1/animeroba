use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::error::Error;

use crate::db::PostgresDb;
use crate::schema::episodes as dsl;
use super::Server;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Episode {
    pub id: i32,
    pub number: i32,
    pub anime_id: i32,
}

impl Episode {
    pub fn from_number(db: &PostgresDb, anime_id: i32, number: i32) -> Result<Episode, Box<dyn Error>> {
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

    pub fn servers(&self, db: &PostgresDb) -> Result<Vec<Server>, Box<dyn Error>> {
        Ok(Server::from_episode_id(db, self.id)?)
    }
}
