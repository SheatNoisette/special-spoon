use crate::db_model::{IotHumidity, IotLed, IotTemperature};
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Temperature {
    pub temperature: f32,
    pub date: i64,
}

impl From<IotTemperature> for Temperature {
    fn from(iot_temperature: IotTemperature) -> Self {
        Temperature {
            temperature: iot_temperature.temperature,
            date: iot_temperature.date,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Humidity {
    pub humidity: f32,
    pub date: i64,
}

impl From<IotHumidity> for Humidity {
    fn from(iot_humidity: IotHumidity) -> Self {
        Humidity {
            humidity: iot_humidity.humidity,
            date: iot_humidity.date,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct LedStatus {
    pub status: bool,
    pub date: i64,
}

impl From<IotLed> for LedStatus {
    fn from(iot_led: IotLed) -> Self {
        LedStatus {
            status: iot_led.led_status,
            date: iot_led.date,
        }
    }
}
