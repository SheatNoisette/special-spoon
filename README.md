# Special spoon

A simple IOT project with MQTT and REST

## Spoonful

Spoonfull is the backend server. You need Rust to run this:
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

You need to build the database first:
```
cargo install diesel_cli --no-default-features --features sqlite
diesel migration run --database-url=db.sqlite
```

## Spoonfull-MCU

Spoonfull-MCU is the Embedded REST client for Spoonful designed for ESP32.

## Spoonfull-MCU-mqtt

Spoonfull-MCU-mqtt is the Embedded ESP32 MQTT client for Spoonful.

## Flan

Flan is a MQTT REST bridge. It makes it easy to use MQTT to send and receive data from the REST API.
