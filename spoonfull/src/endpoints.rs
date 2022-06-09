use crate::ValueDbConnection;
use diesel::prelude::*;
use rocket::{response::Redirect, serde::json::Json};

use dto::data;
use dto::payload;

use crate::db_model::{IotHumidity, IotLed, IotTemperature};
use crate::db_schema::*;

mod dto;
pub mod led;
pub mod humidity;
pub mod temperature;

#[get("/")]
pub async fn index() -> Redirect {
    Redirect::to("/home")
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
