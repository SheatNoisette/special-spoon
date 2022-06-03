use crate::ValueDbConnection;
use rocket::response::{status, Redirect};
use rocket::serde::json::Json;
use rocket::serde::Deserialize;
use diesel::prelude::*;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::db_schema::*;

#[derive(Debug, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct DeviceIdentity {
    pub ip: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct HumidityPayload {
    identity: DeviceIdentity,
    humidity: f32,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct TemperaturePayload {
    identity: DeviceIdentity,
    temperature: f32,
}

#[derive(Debug, Deserialize, Clone)]
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
    conn.run(move |conn| {
        diesel::insert_into(iot_temperature::table)
            .values((
                iot_temperature::id.eq(Uuid::new_v4().to_string()),
                iot_temperature::ip.eq(payload.identity.ip.clone()),
                iot_temperature::temperature.eq(payload.temperature),
                iot_temperature::protocol.eq("rest"),
                iot_temperature::date.eq(chrono::Utc::now().naive_utc().timestamp()),
            ))
            .execute(conn)
            .expect("Error saving temperature into DB");
    }).await;
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
