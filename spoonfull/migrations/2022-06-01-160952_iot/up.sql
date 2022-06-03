-- Your SQL goes here
CREATE TABLE iot_temperature (
    id TEXT PRIMARY KEY NOT NULL,
    ip VARCHAR(255) NOT NULL,
    temperature REAL NOT NULL,
    protocol VARCHAR(16) NOT NULL,
    date BIGINT NOT NULL
);

CREATE TABLE iot_led (
    id TEXT PRIMARY KEY NOT NULL,
    ip VARCHAR(255) NOT NULL,
    led_status boolean NOT NULL,
    protocol VARCHAR(16) NOT NULL,
    date BIGINT NOT NULL
);

CREATE TABLE iot_humidity (
    id TEXT PRIMARY KEY NOT NULL,
    ip VARCHAR(255) NOT NULL,
    humidity REAL NOT NULL,
    protocol VARCHAR(16) NOT NULL,
    date BIGINT NOT NULL
);