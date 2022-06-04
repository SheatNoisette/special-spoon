/**
 * BasicHTTPClient.ino
 *
 *  Created on: 24.05.2015
 *
 */

#include <Arduino.h>
#include <ArduinoJson.h>

#include <WiFi.h>
#include <WiFiMulti.h>

#include <HTTPClient.h>

#define USE_SERIAL Serial

WiFiMulti wifiMulti;

#define WIFI_SSID "Hey"
#define WIFI_PASSWORD "z8ui9afj"
#define SERVER_IP "192.168.55.197:8000"
#define SERVER_URL "http://" SERVER_IP "/temperature"

void setup() {

    USE_SERIAL.begin(115200);

    for(uint8_t t = 4; t > 0; t--) {
        USE_SERIAL.printf("[SETUP] WAIT %d...\n", t);
        USE_SERIAL.flush();
        delay(1000);
    }

    wifiMulti.addAP(WIFI_SSID, WIFI_PASSWORD);

    // Enable builtin led
    pinMode(LED_BUILTIN, OUTPUT);

    USE_SERIAL.println("[SETUP] DONE");
}

bool getLedStateRest() {
    HTTPClient http;
    http.begin("http://" SERVER_IP "/led");

    int code = http.GET();

    if (code != HTTP_CODE_OK) {
        USE_SERIAL.printf("[REST] GET /led failed with code %d\n", code);
        return false;
    }

    String payload = http.getString();
    USE_SERIAL.printf("[REST] GET /led returned %s\n", payload.c_str());

    http.end();

    return true;
}

void loop() {
    USE_SERIAL.println("[LOOP] START");

    // wait for WiFi connection
    while(wifiMulti.run() != WL_CONNECTED);

    digitalWrite(LED_BUILTIN, getLedStateRest() ? HIGH : LOW);

    delay(5000);
}
