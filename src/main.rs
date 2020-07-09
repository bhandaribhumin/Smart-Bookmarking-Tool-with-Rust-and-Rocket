#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[get("/search")]
fn search() -> &'static str {
    "Hello from the search page!"
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    // rocket::ignite().mount("/", routes![index]).launch();
    rocket::ignite().mount("/", routes![index, search]).launch();
}
