#[macro_use] extern crate rocket;

#[get("/<name>/<age>")]
fn hello(name: &str, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}
#[get("/")]
fn test() -> String {
    format!("Hello,  User Test!")
}
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/hello", routes![hello])
        .mount("/test", routes![test])
}

