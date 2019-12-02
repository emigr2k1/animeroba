use postgres::rows::Row;
use serde::{Deserialize, Serialize};
use std::error::Error;

use super::{db_utils::get_cell, Server};
use crate::db::PostgresDb;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Episode {
    pub id: i32,
    pub number: i32,
    pub anime_id: i32,
}

impl Episode {
    pub(super) fn from_row(row: &Row) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            id: get_cell(&row, "episode.id")?,
            number: get_cell(&row, "episode.number")?,
            anime_id: get_cell(&row, "episode.anime_id")?,
        })
    }

    pub fn from_id(db: &PostgresDb, id: i32) -> Result<Episode, Box<dyn Error>> {
        let query = db.query(r#"
            SELECT
                e.id as "episode.id",
                e.number as "episode.number",
                e.anime_id as "episode.anime_id"
            FROM episodes as e
            WHERE e.id = $1"#, &[&id])?;
        if let Some(row) = query.iter().next() {
            Ok(Self::from_row(&row)?)
        } else {
            Err(format!("postgres: episode with id: {} not found", id).into())
        }
    }

    pub fn from_number(db: &PostgresDb, anime_id: i32, id: i32) -> Result<Episode, Box<dyn Error>> {
        let query = db.query(r#"
            SELECT
                e.id as "episode.id",
                e.number as "episode.number",
                e.anime_id as "episode.anime_id"
            FROM episodes as e
            WHERE e.number = $1 AND e.anime_id = $2"#, &[&id, &anime_id])?;
        if let Some(row) = query.iter().next() {
            Ok(Self::from_row(&row)?)
        } else {
            Err(format!("postgres: episode with id: {} not found", id).into())
        }
    }

    pub fn from_anime_id(db: &PostgresDb, anime_id: i32) -> Result<Vec<Self>, Box<dyn Error>> {
        let query = db.query(
            r#"
            SELECT
                e.id as "episode.id",
                e.number as "episode.number",
                e.anime_id as "episode.anime_id"
            FROM episodes as e
            WHERE e.anime_id = $1"#,
            &[&anime_id],
        )?;
        let mut episodes = Vec::new();
        for row in &query {
            let episode = Episode::from_row(&row)?;
            episodes.push(episode);
        }
        Ok(episodes)
    }

    pub(super) fn video_servers(&self, db: &PostgresDb) -> Result<Vec<Server>, Box<dyn Error>> {
        Ok(Server::from_episode_id(db, self.id)?)
    }
}
