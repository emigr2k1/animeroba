#[macro_use]
extern crate diesel;

#[macro_use]
extern crate tera;

use actix_service::Service;
use actix_service::ServiceFactory;
use actix_web::dev::MessageBody;
use actix_web::{
    dev::ServiceRequest, dev::ServiceResponse, error::Error, guard, http::uri, middleware, web,
    App, HttpResponse, HttpServer, Responder,
};
use fern::colors::{Color, ColoredLevelConfig};
use colored::Colorize;
use std::future::Future;

use diesel::r2d2::ConnectionManager;

mod controllers;
mod db;
mod models;
mod schema;

fn main() {
    use controllers::*;

    let colors = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        .info(Color::BrightGreen)
        .debug(Color::BrightMagenta)
        .trace(Color::BrightCyan);

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{}] [{}] {}",
                colors.color(record.level()),
                record.target().to_string().bright_white(),
                message,
            ))
        })
        .level(log::LevelFilter::Info)
        .chain(std::io::stdout())
        .apply()
        .unwrap();

    HttpServer::new(|| {
        let diesel_pool = db::Pool::new(ConnectionManager::new(
            "postgres://emigr2k1:Loc4lpass9@localhost/animeroba",
        ))
        .expect("Could not create diesel postgres pool");

        let tera = tera::Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*"))
            .expect("Could not parse templates");

        App::new()
            .wrap(middleware::NormalizePath)
            .wrap(middleware::Logger::default())
            .wrap_fn(leading_slash)
            .data(tera)
            .data(diesel_pool.clone())
            .route("/", web::get().to(home::index))
            .route("/browse{_:(/)?}", web::get().to(browse::browse))
            .route(
                "/anime/{code_name}{_:(/)?}",
                web::get().to(anime::get_anime),
            )
            .route(
                "/anime/{code_name}/{id}{_:(/)?}",
                web::get().to(anime::get_episode),
            )
            .service(actix_files::Files::new("/static", "./static"))
    })
    .bind("192.168.100.5:8000")
    .unwrap()
    .run()
    .unwrap();
}

fn leading_slash<S, B>(
    mut req: ServiceRequest,
    srv: &mut S,
) -> impl Future<Output = Result<ServiceResponse<impl MessageBody>, Error>>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    B: MessageBody,
{
    let head = req.head_mut();
    let mut parts = head.uri.clone().into_parts();
    let pq = parts.path_and_query.as_ref().unwrap();
    let path = pq.path();
    let path_len = path.len();

    if path_len > 1 && path.chars().nth((path_len - 1) as usize) == Some('/') {
        let pq_bytes = if let Some(query) = pq.query() {
            web::Bytes::from(format!("{}?{}", &path[0..path_len - 1], query))
        } else {
            web::Bytes::from(path[0..path_len - 1].to_string())
        };

        parts.path_and_query =
            Some(uri::PathAndQuery::from_maybe_shared(pq_bytes).unwrap());

        let uri = uri::Uri::from_parts(parts).unwrap();
        req.match_info_mut().get_mut().update(&uri);
        req.head_mut().uri = uri;
    }

    srv.call(req)
}
