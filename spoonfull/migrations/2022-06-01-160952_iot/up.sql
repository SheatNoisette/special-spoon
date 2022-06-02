-- Your SQL goes here
CREATE TABLE iot (
    ip VARCHAR(255) PRIMARY KEY NOT NULL,
    led_status boolean NOT NULL,
    temperature REAL NOT NULL,
    humidity REAL NOT NULL,
    protocol VARCHAR(16) NOT NULL,
    date INTEGER NOT NULL
)