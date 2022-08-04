#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/hello/<name>")]
fn hello(name: String) -> String {
    format!("Hello, {}! You are the greatest programmer ever", name)
}

#[post("/hello", format = "json", data = "<name>")]
fn hello_post(name: String) -> String {
    format!("Hello, {}!", name)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, hello, hello_post])
        .mount("/this", routes![index, hello, hello_post])
}