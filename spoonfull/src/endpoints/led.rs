use crate::db_schema::*;
use crate::ValueDbConnection;
use super::dto::payload;

use diesel::prelude::*;
use rocket::response::status::Accepted;
use rocket::response::status::BadRequest;
use rocket::{response::status, serde::json::Json};
use uuid::Uuid;


#[post("/led/<status>")]
pub async fn set(
    status: String,
    conn: ValueDbConnection,
) -> Result<Accepted<()>, BadRequest<String>> {
    info!("Received led status:\n {:?}", status);

    let led_status = match status.as_str() {
        "on" => true,
        "off" => false,
        _ => {
            warn!("Invalid led status: {}", status);
            return Err(status::BadRequest(Some("Invalid led status".to_string())));
        }
    };

    conn.run(move |conn| {
        diesel::insert_into(iot_led::table)
            .values((
                iot_led::id.eq(Uuid::new_v4().to_string()),
                iot_led::ip.eq("server"),
                iot_led::led_status.eq(led_status),
                iot_led::protocol.eq("rest"),
                iot_led::date.eq(chrono::Utc::now().naive_utc().timestamp()),
            ))
            .execute(conn)
            .expect("Error saving led into DB");
    })
    .await;
    Ok(status::Accepted::<()>(None))
}

#[get("/led", format = "application/json")]
pub async fn get(conn: ValueDbConnection) -> Json<payload::Led> {
    let mut led_status = payload::Led {
        ip: "".to_string(),
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
