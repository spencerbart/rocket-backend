#[macro_use] extern crate rocket;
use rocket::serde::{Deserialize, json::Json};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/hello/<name>")]
fn hello(name: String) -> String {
    format!("Hello, {}! You are the greatest programmer ever", name)
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct User<'r> {
    name: &'r str,
    // age: u8,
}

#[post("/user", format = "application/json", data = "<user>")]
fn create_user(user: Json<User<'_>>) -> String {
    format!("Hello, {}! You are the greatest programmer ever", user.name)
}

#[get("/user/<id>")]
fn get_user(id: u32) -> String {
    format!("User {}", id)
}

#[launch]
fn rocket() -> _ {
    print!("this is working");
    rocket::build()
        .mount("/", routes![index, hello])
        .mount("/admin", routes![create_user, get_user])
}