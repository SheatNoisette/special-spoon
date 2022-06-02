use crate::ValueDbConnection;
use rocket::response::{status, Redirect};
use rocket::serde::json::Json;
use rocket::serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct DeviceIdentity {
    pub ip: String,
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct HumidityPayload {
    identity: DeviceIdentity,
    humidity: f32,
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct TemperaturePayload {
    identity: DeviceIdentity,
    temperature: f32,
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct LedPayload {
    identity: DeviceIdentity,
    status: bool,
}

#[get("/")]
pub async fn index() -> Redirect {
    Redirect::to("/home")
}

#[post("/temperature", format = "application/json", data = "<payload>")]
pub async fn temperature(
    payload: Json<TemperaturePayload>,
    conn: ValueDbConnection,
) -> status::Accepted<()> {
    println!("Received temperature:\n {:?}", payload);
    status::Accepted::<()>(None)
}

#[post("/humidity", format = "application/json", data = "<payload>")]
pub async fn humidity(
    payload: Json<HumidityPayload>,
    conn: ValueDbConnection,
) -> status::Accepted<()> {
    println!("Received humidity:\n {:?}", payload);
    status::Accepted::<()>(None)
}

#[post("/led", format = "application/json", data = "<payload>")]
pub async fn led_status(
    payload: Json<LedPayload>,
    conn: ValueDbConnection,
) -> status::Accepted<()> {
    println!("Received led status:\n {:?}", payload);
    status::Accepted::<()>(None)
}
