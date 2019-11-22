#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::response::content;
use rocket_contrib::{serve::StaticFiles, templates::Template};

mod api;
mod db;

#[get("/")]
fn index() -> Template {
    Template::render("index", std::collections::HashMap::<(), ()>::new())
}

#[get("/explorar")]
fn browse() -> Template {
    Template::render("browse", std::collections::HashMap::<(), ()>::new())
}

#[catch(404)]
fn not_found() -> content::Html<&'static str> {
    content::Html("<h1>404!</h1> page not found")
}

fn main() {
    use api::*;
    rocket::ignite()
        .register(catchers![not_found])
        .attach(Template::fairing())
        .attach(db::Db::fairing())
        .mount(
            "/",
            routes![
                index,
                browse,
                get_animes,
                post_anime,
                get_anime,
                put_anime,
                post_anime_episode,
                get_anime_episode,
            ],
        )
        .mount(
            "/static",
            StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")),
        )
        .launch();
}
