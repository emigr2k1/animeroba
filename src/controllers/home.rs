use actix_web::{error, web, Error, HttpResponse};
use tera::{Context, Tera};

use crate::db::Pool;
use crate::models::Episode;

pub async fn index(db: web::Data<Pool>, tmpl: web::Data<Tera>) -> Result<HttpResponse, Error> {
    let quantity = 24;
    let latest_eps = Episode::latest_n(
        &*db.get()
            .map_err(|_| error::ErrorInternalServerError("Internal error"))?,
        quantity,
    )
    .map_err(|_| error::ErrorInternalServerError("Internal error"))?;

    let mut ctx = Context::new();
    ctx.insert("latest_eps", &latest_eps);

    let html_res = tmpl
        .render("index.tera", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Internal error"))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(html_res))
}
