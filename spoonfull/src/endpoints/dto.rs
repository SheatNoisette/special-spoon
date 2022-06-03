use crate::db_model::{IotHumidity, IotLed, IotTemperature};
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct TemperatureData {
    pub temperature: f32,
    pub date: i64,
}

impl From<IotTemperature> for TemperatureData {
    fn from(iot_temperature: IotTemperature) -> Self {
        TemperatureData {
            temperature: iot_temperature.temperature,
            date: iot_temperature.date,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct HumidityData {
    pub humidity: f32,
    pub date: i64,
}

impl From<IotHumidity> for HumidityData {
    fn from(iot_humidity: IotHumidity) -> Self {
        HumidityData {
            humidity: iot_humidity.humidity,
            date: iot_humidity.date,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct LedStatusData {
    pub status: bool,
    pub date: i64,
}

impl From<IotLed> for LedStatusData {
    fn from(iot_led: IotLed) -> Self {
        LedStatusData {
            status: iot_led.led_status,
            date: iot_led.date,
        }
    }
}
