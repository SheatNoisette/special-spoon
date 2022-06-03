use diesel::Queryable;

#[derive(Queryable)]
pub struct IotTemperature {
    pub ip: String,
    pub temperature: f32,
    pub protocol: String,
    pub date: i32,
}

#[derive(Queryable)]
pub struct IotHumidity {
    pub ip: String,
    pub humidity: f32,
    pub protocol: String,
    pub date: i32,
}

#[derive(Queryable)]
pub struct IotLed {
    pub ip: String,
    pub led_status: bool,
    pub protocol: String,
    pub date: i32,
}
