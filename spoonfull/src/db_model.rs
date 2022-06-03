use diesel::{sql_types::*, Queryable};

#[derive(Queryable)]
pub struct IotTemperature {
    pub id: String,
    pub ip: String,
    pub temperature: f32,
    pub protocol: String,
    pub date: i64,
}

#[derive(Queryable)]
pub struct IotHumidity {
    pub id: String,
    pub ip: String,
    pub humidity: f32,
    pub protocol: String,
    pub date: i64,
}

#[derive(Queryable)]
pub struct IotLed {
    pub id: String,
    pub ip: String,
    pub led_status: bool,
    pub protocol: String,
    pub date: i64,
}
