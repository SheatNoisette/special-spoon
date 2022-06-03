use super::dto::payload;
use crate::db_schema::*;
use crate::ValueDbConnection;
use diesel::prelude::*;
use rocket::{response::status, serde::json::Json};
use uuid::Uuid;

#[post("/temperature", format = "application/json", data = "<payload>")]
pub async fn set(
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
