use rocket_contrib::database;
use rocket_contrib::databases::mongodb;

#[database("mongo_db")]
pub struct Db(mongodb::db::Database);
