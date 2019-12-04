use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::error::Error;

use crate::db::PostgresDb;
use crate::schema::video_servers as dsl;

#[derive(Serialize, Deserialize, Queryable)]
pub struct Server {
    pub id: i32,
    pub name: String,
    pub url: String,
    pub episode_id: i32,
}

impl Server {
    pub fn from_episode_id(
        db: &PostgresDb,
        episode_id: i32,
    ) -> Result<Vec<Server>, Box<dyn Error>> {
        let results = dsl::table
            .select(dsl::all_columns)
            .filter(dsl::episode_id.eq(episode_id))
            .load::<Self>(&**db)?;
        Ok(results)
    }
}
