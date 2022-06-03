use crate::ValueDbConnection;
use diesel::prelude::*;
use rocket::response::{status, Redirect};
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db_schema::*;

#[derive(Debug, Deserialize, Serialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
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
    })
    .await;
    status::Accepted::<()>(None)
}

#[post("/humidity", format = "application/json", data = "<payload>")]
pub async fn humidity(
    payload: Json<HumidityPayload>,
    conn: ValueDbConnection,
) -> status::Accepted<()> {
    println!("Received humidity:\n {:?}", payload);
    conn.run(move |conn| {
        diesel::insert_into(iot_humidity::table)
            .values((
                iot_humidity::id.eq(Uuid::new_v4().to_string()),
                iot_humidity::ip.eq(payload.identity.ip.clone()),
                iot_humidity::humidity.eq(payload.humidity),
                iot_humidity::protocol.eq("rest"),
                iot_humidity::date.eq(chrono::Utc::now().naive_utc().timestamp()),
            ))
            .execute(conn)
            .expect("Error saving humidity into DB");
    })
    .await;
    status::Accepted::<()>(None)
}

#[post("/led", format = "application/json", data = "<payload>")]
pub async fn set_led(payload: Json<LedPayload>, conn: ValueDbConnection) -> status::Accepted<()> {
    println!("Received led status:\n {:?}", payload);
    conn.run(move |conn| {
        diesel::insert_into(iot_led::table)
            .values((
                iot_led::id.eq(Uuid::new_v4().to_string()),
                iot_led::ip.eq(payload.identity.ip.clone()),
                iot_led::led_status.eq(payload.status),
                iot_led::protocol.eq("rest"),
                iot_led::date.eq(chrono::Utc::now().naive_utc().timestamp()),
            ))
            .execute(conn)
            .expect("Error saving led into DB");
    })
    .await;
    status::Accepted::<()>(None)
}

#[get("/led", format = "application/json")]
pub async fn led_status(conn: ValueDbConnection) -> Json<LedPayload> {
    let mut led_status = LedPayload {
        identity: DeviceIdentity { ip: "".to_string() },
        status: false,
    };

    led_status.status = conn
        .run(move |conn| {
            let led_status_query = iot_led::table
                .select(iot_led::led_status)
                .order(iot_led::date.desc())
                .first::<bool>(conn);
            led_status_query.unwrap_or_else(|_| {
                warn!("Cannot find requested led value");
                return false;
            })
        })
        .await;
    Json(led_status)
}
