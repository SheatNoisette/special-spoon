use diesel::Queryable;

#[derive(Queryable)]
pub struct Iot {
    pub ip: String,
    pub led_status: bool,
    pub temperature: f32,
    pub humidity: f32,
    pub protocol: String,
    pub date: i32,
}
