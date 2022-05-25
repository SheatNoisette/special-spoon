#[macro_use] extern crate rocket;
use rocket::{Rocket, Build};
use rocket::serde::json::Json;
use rocket::serde::Deserialize;
use rocket::response::status;

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
struct DeviceIdentity {
    pub ip : String,
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
fn home() -> &'static str {
    "Home page"
}

#[post("/temperature", format = "application/json", data = "<payload>")]
fn temperature(payload : Json<TemperaturePayload>) -> status::Accepted<()> {
    println!("Received temperature:\n {:?}", payload);
    status::Accepted::<()>(None)
}

#[post("/humidity", format = "application/json", data = "<payload>")]
fn humidity(payload : Json<HumidityPayload>) -> status::Accepted<()> {
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
    println!("Launching rocket");
    rocket::build()
        .mount("/", routes![home, temperature, humidity, led_status])
}
