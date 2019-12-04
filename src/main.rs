#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

use rocket_contrib::{serve::StaticFiles, templates::Template};

mod controllers;
mod db;
mod models;
mod schema;

fn main() {
    use controllers::*;
    rocket::ignite()
        .register(catchers![error::not_found,])
        .attach(Template::fairing())
        .attach(db::MongoDb::fairing())
        .attach(db::PostgresDb::fairing())
        .mount(
            "/",
            routes![
                //api::get_animes,
                //api::get_anime,
                //api::get_anime_episode,
                home::index,
                browse::browse,
                anime::get_anime,
                //post_anime,
            ],
        )
        .mount(
            "/static",
            StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")),
        )
        .launch();
}
