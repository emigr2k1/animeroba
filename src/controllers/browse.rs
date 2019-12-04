use rocket::http::Status;
use rocket_contrib::templates::{Template, tera::Context};

use crate::db::PostgresDb;
use crate::models::Anime;

#[get("/explorar?<page>&<quantity>")]
pub fn browse(db: PostgresDb, page: Option<u32>, quantity: Option<u32>) -> Result<Template, Status> {
    let page = page.unwrap_or(1);
    let quantity = quantity.unwrap_or(48);
    let animes =
        Anime::from_page(&db, page as i64, quantity as i64).map_err(|_| Status::NotFound)?;
    let mut ctx = Context::new();
    ctx.insert("animes", &animes);
    Ok(Template::render("browse", ctx))
}
