#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::response::content;
use rocket_contrib::{
    templates::Template,
    serve::StaticFiles,
};

#[get("/")]
fn index() -> Template {
    Template::render("index", std::collections::HashMap::<(), ()>::new())
}

#[catch(404)]
fn not_found() -> content::Html<&'static str> {
    content::Html("<h1>404!</h1> page not found")
}

fn main() {
    rocket::ignite()
        .register(catchers![not_found])
        .attach(Template::fairing())
        .mount("/", routes![index])
        .mount("/static", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")))
        .launch();
}
