#[macro_use]
extern crate rocket;
use rocket::response::{status, Redirect};
use rocket::serde::json::Json;
use rocket::serde::Deserialize;
use rocket::{Build, Rocket};
use rocket_dyn_templates::Template;

mod about;
mod home;

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
struct DeviceIdentity {
    pub ip: String,
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
struct HumidityPayload {
    identity: DeviceIdentity,
    humidity: f32,
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
struct TemperaturePayload {
    identity: DeviceIdentity,
    temperature: f32,
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
struct LedPayload {
    identity: DeviceIdentity,
    status: bool,
}

#[get("/")]
fn index() -> Redirect {
    Redirect::to("/home")
}

#[post("/temperature", format = "application/json", data = "<payload>")]
fn temperature(payload: Json<TemperaturePayload>) -> status::Accepted<()> {
    println!("Received temperature:\n {:?}", payload);
    status::Accepted::<()>(None)
}

#[post("/humidity", format = "application/json", data = "<payload>")]
fn humidity(payload: Json<HumidityPayload>) -> status::Accepted<()> {
    println!("Received humidity:\n {:?}", payload);
    status::Accepted::<()>(None)
}

#[post("/led", format = "application/json", data = "<payload>")]
fn led_status(payload: Json<LedPayload>) -> status::Accepted<()> {
    println!("Received led status:\n {:?}", payload);
    status::Accepted::<()>(None)
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
    .attach(Template::fairing())
    .mount(
        "/",
        routes![
            index,
            home::home,
            about::about,
            temperature,
            humidity,
            led_status
        ],
    )
}
