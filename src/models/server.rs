use postgres::rows::Row;
use serde::{Deserialize, Serialize};
use std::error::Error;

use super::db_utils::get_cell;
use crate::db::PostgresDb;

#[derive(Debug, Default, Serialize, Deserialize, FromForm)]
pub struct Server {
    pub id: i32,
    pub name: String,
    pub url: String,
    pub episode_id: i32,
}

impl Server {
    pub(super) fn from_row(row: &Row) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            id: get_cell(&row, "server.id")?,
            name: get_cell(&row, "server.name")?,
            url: get_cell(&row, "server.url")?,
            episode_id: get_cell(&row, "server.episode_id")?,
        })
    }

    pub fn from_episode_id(
        db: &PostgresDb,
        episode_id: i32,
    ) -> Result<Vec<Server>, Box<dyn Error>> {
        let query = db.query(
            r#"
            SELECT
                s.id as "server.id",
                s.name as "server.name",
                s.url as "server.url",
                s.episode_id as "server.episode_id"
            FROM video_servers as s
            WHERE s.episode_id = $1"#,
            &[&episode_id],
        )?;
        let mut servers = Vec::new();
        for row in &query {
            let server = Server::from_row(&row)?;
            servers.push(server);
        }
        Ok(servers)
    }
}
