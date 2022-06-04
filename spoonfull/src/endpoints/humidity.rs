use crate::ValueDbConnection;
use diesel::prelude::*;
use rocket::{response::status, serde::json::Json};
use uuid::Uuid;

use super::dto::payload;

use crate::db_schema::*;

#[post("/humidity", format = "application/json", data = "<payload>")]
pub async fn set(
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
