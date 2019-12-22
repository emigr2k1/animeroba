use actix_web::{error, web, Error, HttpResponse};
use serde::Deserialize;
use tera::{Context, Tera};

use crate::db::Pool;
use crate::models::Anime;

#[derive(Deserialize)]
pub struct BrowseQuery {
    page: Option<i64>,
    quantity: Option<i64>,
}

pub async fn browse(
    web::Query(params): web::Query<BrowseQuery>,
    db: web::Data<Pool>,
    tmpl: web::Data<Tera>,
) -> Result<HttpResponse, Error> {
    let page = params.page.unwrap_or(1);
    let quantity = params.quantity.unwrap_or(48);

    let animes = Anime::from_page(
        &*db.get()
            .map_err(|_| error::ErrorInternalServerError("Internal error"))?,
        page,
        quantity,
    )
    .map_err(|e| {
        println!("{}", e);
        error::ErrorInternalServerError("InternalServerError")
    })?;

    let mut ctx = Context::new();
    ctx.insert("animes", &animes);
    let html_res = tmpl.render("browse.tera", &ctx).map_err(|e| {
        println!("{}", e);
        error::ErrorInternalServerError("InternalServerError")
    })?;

    Ok(HttpResponse::Ok().content_type("text/html").body(html_res))
}
