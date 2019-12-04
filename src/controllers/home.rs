use rocket::http::Status;
use rocket_contrib::templates::{tera::Context, Template};

use crate::db::PostgresDb;
use crate::models::Episode;

#[get("/?<quantity>")]
pub fn index(db: PostgresDb, quantity: Option<u64>) -> Result<Template, Status> {
    let quantity = quantity.unwrap_or(24);
    let latest_eps = Episode::latest_n(&db, quantity).map_err(|_| Status::InternalServerError)?;

    let mut ctx = Context::new();
    ctx.insert("latest_eps", &latest_eps);

    Ok(Template::render("index", ctx))
}
