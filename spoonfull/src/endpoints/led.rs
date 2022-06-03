use super::dto::payload::{self, DeviceIdentity};
use crate::db_schema::*;
use crate::ValueDbConnection;
use diesel::prelude::*;
use rocket::{response::status, serde::json::Json};
use uuid::Uuid;

#[post("/led", format = "application/json", data = "<payload>")]
pub async fn set(payload: Json<payload::Led>, conn: ValueDbConnection) -> status::Accepted<()> {
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
pub async fn get(conn: ValueDbConnection) -> Json<payload::Led> {
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
