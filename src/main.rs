#[macro_use]
extern crate diesel;

#[macro_use]
extern crate tera;

use actix_web::{guard, middleware, web, App, HttpResponse, HttpServer, Responder};
use diesel::r2d2::ConnectionManager;

mod controllers;
mod db;
mod models;
mod schema;

fn main() {
    use controllers::*;

    HttpServer::new(|| {
        let diesel_pool = db::Pool::new(ConnectionManager::new(
            "postgres://emigr2k1:Loc4lpass9@localhost/animeroba",
        ))
        .expect("Could not create diesel postgres pool");

        let tera = tera::Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*"))
            .expect("Could not parse templates");
        App::new()
            .wrap(middleware::NormalizePath)
            .data(tera)
            .data(diesel_pool.clone())
            .route("/", web::get().to(home::index))
            .route("/browse{x:(/)?}", web::get().to(browse::browse))
            .route("/anime/{code_name}{x:(/)?}", web::get().to(anime::get_anime))
            .route("/anime/{code_name}/{id}{x:(/)?}", web::get().to(anime::get_episode))
            .service(actix_files::Files::new("/static", "./static"))
    })
    .bind("192.168.100.5:8000")
    .unwrap()
    .run()
    .unwrap();

    //rocket::ignite()
    //    .register(catchers![error::not_found,])
    //    .attach(Template::fairing())
    //    .attach(db::MongoDb::fairing())
    //    .attach(db::PostgresDb::fairing())
    //    .mount(
    //        "/",
    //        routes![
    //            //api::get_animes,
    //            //api::get_anime,
    //            //api::get_anime_episode,
    //            home::index,
    //            browse::browse,
    //            anime::get_anime,
    //            anime::get_episode,
    //            //post_anime,
    //        ],
    //    )
    //    .mount(
    //        "/static",
    //        StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")),
    //    )
    //    .launch();
}
