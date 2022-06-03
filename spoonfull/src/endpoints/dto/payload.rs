use super::data;
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct DeviceIdentity {
    pub ip: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Humidity {
    pub identity: DeviceIdentity,
    pub humidity: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Temperature {
    pub identity: DeviceIdentity,
    pub temperature: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Led {
    pub identity: DeviceIdentity,
    pub status: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct IotData {
    temperature: Vec<data::Temperature>,
    humidity: Vec<data::Humidity>,
    led: Vec<data::LedStatus>,
}

impl IotData {
    pub fn new(
        temperature: Vec<data::Temperature>,
        humidity: Vec<data::Humidity>,
        led: Vec<data::LedStatus>,
    ) -> Self {
        IotData {
            temperature,
            humidity,
            led,
        }
    }
}
