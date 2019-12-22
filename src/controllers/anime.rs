use actix_web::{error, web, Error, HttpResponse};
use tera::{Context, Tera};

use crate::db::Pool;
use crate::models::Anime;

pub async fn get_anime(
    path: web::Path<(String,)>,
    db: web::Data<Pool>,
    tmpl: web::Data<Tera>,
) -> Result<HttpResponse, Error> {
    let code_name = &path.0;

    let anime = Anime::from_code_name(
        &*db.get()
            .map_err(|_| error::ErrorInternalServerError("Internal error"))?,
        &code_name,
    )
    .map_err(|_| error::ErrorNotFound("Anime not found"))?;
    let num_episodes = anime
        .count_episodes(
            &*db.get()
                .map_err(|_| error::ErrorInternalServerError("Internal error"))?,
        )
        .map_err(|_| error::ErrorInternalServerError("Internal error"))?;

    let mut context = Context::new();
    context.insert("anime", &anime);
    context.insert("num_episodes", &num_episodes);
    context.insert("genres_str", &anime.genres_str());

    let html_res = tmpl
        .render("anime.tera", &context)
        .map_err(|_| error::ErrorInternalServerError("Internal error"))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(html_res))
}

pub async fn get_episode(
    path: web::Path<(String, u64)>,
    db: web::Data<Pool>,
    tmpl: web::Data<Tera>,
) -> Result<HttpResponse, Error> {
    let code_name = &path.0;
    let episode_num = path.1;

    let html_res = tmpl
        .render("episode.tera", &Context::new())
        .map_err(|_| error::ErrorInternalServerError("Internal error"))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(html_res))
}
