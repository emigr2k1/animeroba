use postgres::Connection;
use rocket_contrib::{database, databases::mongodb::db::Database};

#[database("mongo_db")]
pub struct MongoDb(Database);

#[database("postgres_db")]
pub struct PostgresDb(Connection);
