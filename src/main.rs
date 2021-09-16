#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/world")] // <- route attribute
fn world() -> &'static str {// <- request handler
    "hi, world!"
}

#[get("/small")] // <- route attribute
fn small() -> &'static str {
    // <- request handler
    "hi, small!"
}

#[rocket::main]
async fn main() {
    rocket::build()
        .mount("/", routes![index,world])
        .mount("/hello", routes![small])
        .launch()
        .await;
}