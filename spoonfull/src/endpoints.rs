use crate::ValueDbConnection;
use diesel::prelude::*;
use rocket::response::{status, Redirect};
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db_model::*;
use crate::db_schema::*;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct DeviceIdentity {
    pub ip: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct HumidityPayload {
    identity: DeviceIdentity,
    humidity: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct TemperatureData {
    pub temperature: f32,
    pub date: i64,
}

impl From<IotTemperature> for TemperatureData {
    fn from(iot_temperature: IotTemperature) -> Self {
        TemperatureData {
            temperature: iot_temperature.temperature,
            date: iot_temperature.date,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct HumidityData {
    pub humidity: f32,
    pub date: i64,
}

impl From<IotHumidity> for HumidityData {
    fn from(iot_humidity: IotHumidity) -> Self {
        HumidityData {
            humidity: iot_humidity.humidity,
            date: iot_humidity.date,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct LedStatusData {
    pub status: bool,
    pub date: i64,
}

impl From<IotLed> for LedStatusData {
    fn from(iot_led: IotLed) -> Self {
        LedStatusData {
            status: iot_led.led_status,
            date: iot_led.date,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct IotDataPayload {
    temperature: Vec<TemperatureData>,
    humidity: Vec<HumidityData>,
    led: Vec<LedStatusData>,
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

#[get("/data/<number>", format = "application/json")]
pub async fn get_data(number: i64, conn: ValueDbConnection) -> Json<IotDataPayload> {
    // Fetch the last 10 temperature values
    let temperature = conn
        .run(move |conn| {
            iot_temperature::table
                .select(iot_temperature::all_columns)
                .order(iot_temperature::date.desc())
                .limit(number)
                .load::<IotTemperature>(conn)
                .unwrap()
        })
        .await
        .into_iter()
        .map(TemperatureData::from)
        .collect();

    let humidity = conn
        .run(move |conn| {
            iot_humidity::table
                .select(iot_humidity::all_columns)
                .order(iot_humidity::date.desc())
                .limit(number)
                .load::<IotHumidity>(conn)
                .unwrap()
        })
        .await
        .into_iter()
        .map(HumidityData::from)
        .collect();

    let led = conn
        .run(move |conn| {
            iot_led::table
                .select(iot_led::all_columns)
                .order(iot_led::date.desc())
                .limit(number)
                .load::<IotLed>(conn)
                .unwrap()
        })
        .await
        .into_iter()
        .map(LedStatusData::from)
        .collect();

    Json(IotDataPayload {
        temperature,
        humidity,
        led,
    })
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
        .run(|conn| {
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
