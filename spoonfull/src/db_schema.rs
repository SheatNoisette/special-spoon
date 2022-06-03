table! {
    iot_humidity (id) {
        id -> Text,
        ip -> Text,
        humidity -> Float,
        protocol -> Text,
        date -> BigInt,
    }
}

table! {
    iot_led (id) {
        id -> Text,
        ip -> Text,
        led_status -> Bool,
        protocol -> Text,
        date -> BigInt,
    }
}

table! {
    iot_temperature (id) {
        id -> Text,
        ip -> Text,
        temperature -> Float,
        protocol -> Text,
        date -> BigInt,
    }
}

allow_tables_to_appear_in_same_query!(
    iot_humidity,
    iot_led,
    iot_temperature,
);
