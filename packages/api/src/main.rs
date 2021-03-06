#[macro_use] extern crate rocket;

// Try visiting:
// http://127.0.0.1:8000/downloads
#[get("/downloads")]
fn downloads() -> &'static str {
    "downloads"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![downloads])
}
