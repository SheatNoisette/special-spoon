#include <DFRobot_DHT11.h>
#include <uTimerLib.h>
#include <Arduino.h>
#include <ArduinoJson.h>
#include <WiFi.h>
#include <WiFiMulti.h>
#include <HTTPClient.h>

#include "config-mcu.h"

WiFiMulti wifiMulti;
DFRobot_DHT11 DHT;

bool is_wifi_connected = false;

#define LED_ENDPOINT "http://" SERVER_IP "/led"
#define TEMPERATURE_ENDPOINT "http://" SERVER_IP "/temperature"
#define HUMIDITY_ENDPOINT "http://" SERVER_IP "/humidity"

bool getLedStateRest() {
    String payload;
    int code;
  
    DynamicJsonDocument doc(PAYLOAD_SIZE);
    DeserializationError error;
    HTTPClient http;
    
    http.begin(LED_ENDPOINT);
    code = http.GET();

    if (code != HTTP_CODE_OK) {
        Serial.printf("[REST] GET /led failed with code %d\n", code);
        http.end();
        return false;
    }

    payload = http.getString();
    error  = deserializeJson(doc, payload.c_str());
  
    if (error) {
      Serial.println(error.f_str());
      http.end();
      return false;
    }

    http.end();

    Serial.printf("[REST] Got led status - %i\n", doc["status"] == true);

    return doc["status"];
}

void updateLed() {
  if (is_wifi_connected == false) return;
  
  digitalWrite(LED_BUILTIN, getLedStateRest() ? HIGH : LOW);
}

void pushDHT() {
  if (is_wifi_connected == false) return;

  DHT.read(DHT11_PIN);

  delay(100);

  float temp = DHT.temperature;
  float humid = DHT.humidity;

  pushWeatherData(TEMPERATURE_ENDPOINT, "temperature", temp);
  pushWeatherData(HUMIDITY_ENDPOINT, "humidity", humid);

}

void pushWeatherData(const char *endpoint, const char *nameData, float payload) {
  if (is_wifi_connected == false) return;

  int code;
  String output;
  
  DynamicJsonDocument doc(PAYLOAD_SIZE);
  HTTPClient http;

  // Build JSON
  doc["ip"] = WiFi.localIP();
  doc[nameData] = payload;
  serializeJson(doc, output);

  // Make HTTP request
  http.begin(endpoint);
  http.addHeader("Content-Type","application/json");
  
  code = http.POST((uint8_t *)output.c_str(), output.length());

  if (code != HTTP_CODE_ACCEPTED) {
        Serial.printf("[REST] GET %s failed with code %d\n",endpoint, code);
        http.end();
        return;
   }

   Serial.printf("[REST] Pushed %s = %f !\n", nameData, payload);

   http.end();
}

void updateData() {
  updateLed();
  pushDHT();
}

void setup() {

    Serial.begin(115200);

    Serial.println("Connecting to "WIFI_SSID" PWD:" WIFI_PASSWORD);
    wifiMulti.addAP(WIFI_SSID, WIFI_PASSWORD);

    for(uint8_t t = 4; t > 0; t--) {
        Serial.printf("[SETUP] WAIT %d...\n", t);
        Serial.flush();
        delay(1000);
    }


    // Enable builtin led
    Serial.println("[SETUP] Enable Led");
    pinMode(LED_BUILTIN, OUTPUT);
    digitalWrite(LED_BUILTIN, HIGH);

    Serial.println("[SETUP] Set timer...");
    TimerLib.setInterval_s(updateData, DATA_REFRESH_INTERVAL_SECONDS);

    Serial.println("[SETUP] DONE! Waiting for connection...");

    delay(100);
}

void loop() {
    // wait for WiFi connection
    if((wifiMulti.run() == WL_CONNECTED) && (is_wifi_connected == false)) {
      Serial.println("[LOOP] Connected");
      Serial.flush();
      is_wifi_connected = true;
    }
}
