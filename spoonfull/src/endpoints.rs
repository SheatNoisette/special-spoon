use crate::ValueDbConnection;
use diesel::prelude::*;
use rocket::{
    response::{status, Redirect},
    serde::json::Json,
};
use uuid::Uuid;

use dto::data;
use dto::payload;
use dto::payload::DeviceIdentity;

use crate::db_model::{IotHumidity, IotLed, IotTemperature};
use crate::db_schema::*;

mod dto;

#[get("/")]
pub async fn index() -> Redirect {
    Redirect::to("/home")
}

#[post("/temperature", format = "application/json", data = "<payload>")]
pub async fn temperature(
    payload: Json<payload::Temperature>,
    conn: ValueDbConnection,
) -> status::Accepted<()> {
    info!("Received temperature:\n {:?}", payload);
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
    payload: Json<payload::Humidity>,
    conn: ValueDbConnection,
) -> status::Accepted<()> {
    info!("Received humidity:\n {:?}", payload);
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
pub async fn get_data(number: i64, conn: ValueDbConnection) -> Json<payload::IotData> {
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
        .map(data::Temperature::from)
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
        .map(data::Humidity::from)
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
        .map(data::LedStatus::from)
        .collect();

    Json(payload::IotData::new(temperature, humidity, led))
}

#[post("/led", format = "application/json", data = "<payload>")]
pub async fn set_led(payload: Json<payload::Led>, conn: ValueDbConnection) -> status::Accepted<()> {
    info!("Received led status:\n {:?}", payload);
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
pub async fn led_status(conn: ValueDbConnection) -> Json<payload::Led> {
    let mut led_status = payload::Led {
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
                false
            })
        })
        .await;
    Json(led_status)
}
