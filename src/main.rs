#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/hello")]
fn hello() -> &'static str {
    "Hello, test!"
}

#[get("/hello/<name>")]
fn hi(name: String) -> String {
    format!("Hello, {}!", name)
}

fn main() {
    rocket::ignite().mount("/", routes![index, hello, hi]).launch();
}
