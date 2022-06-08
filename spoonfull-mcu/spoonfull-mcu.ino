#include <Arduino.h>
#include <ArduinoJson.h>
#include <DFRobot_DHT11.h>
#include <HTTPClient.h>
#include <WiFi.h>
#include <WiFiMulti.h>
#include <uTimerLib.h>

#include "config-mcu.h"

WiFiMulti wifiMulti;
DFRobot_DHT11 DHT;

bool is_wifi_connected = false;

// Endpoint for the server
#define LED_ENDPOINT "http://" SERVER_IP "/led"
#define TEMPERATURE_ENDPOINT "http://" SERVER_IP "/temperature"
#define HUMIDITY_ENDPOINT "http://" SERVER_IP "/humidity"

// Get the led status from the server
bool getLedStateRest() {
  String payload;
  int code;

  DynamicJsonDocument doc(PAYLOAD_SIZE);
  DeserializationError error;
  HTTPClient http;

  // Get the led status from the server
  http.begin(LED_ENDPOINT);
  code = http.GET();

  // Check if the request was successful
  if (code != HTTP_CODE_OK) {
    Serial.printf("[REST] GET /led failed with code %d\n", code);
    http.end();
    return false;
  }

  // Get the response payload
  payload = http.getString();

  // Transform the payload into a JsonDocument
  error = deserializeJson(doc, payload.c_str());

  if (error) {
    Serial.println(error.f_str());
    http.end();
    return false;
  }

  http.end();

  Serial.printf("[REST] Got led status - %i\n", doc["status"] == true);

  return doc["status"];
}

// Enable or disable the led from the data given by the server
void updateLed() {
  if (is_wifi_connected == false)
    return;

  digitalWrite(LED_BUILTIN, getLedStateRest() ? HIGH : LOW);
}

// Get the temperature and humidity from the DHT11 sensor
// And send it to the server
void pushDHT() {
  if (is_wifi_connected == false)
    return;

  // Wait for the sensor to be ready
  delay(100);

  DHT.read(DHT11_PIN);

  float temp = DHT.temperature;
  float humid = DHT.humidity;

  // Push to the server
  pushWeatherData(TEMPERATURE_ENDPOINT, "temperature", temp);
  pushWeatherData(HUMIDITY_ENDPOINT, "humidity", humid);
}

// Send a post request to the server with the given endpoint and data
void pushWeatherData(const char *endpoint, const char *nameData,
                     float payload) {
  if (is_wifi_connected == false)
    return;

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

  // Add content type header to be accepted as JSON data by the server
  http.addHeader("Content-Type", "application/json");

  // Post the request
  code = http.POST((uint8_t *)output.c_str(), output.length());

  // Check the response code of the server
  if (code != HTTP_CODE_ACCEPTED) {
    Serial.printf("[REST] GET %s failed with code %d\n", endpoint, code);
    http.end();
    return;
  }

  Serial.printf("[REST] Pushed %s = %f !\n", nameData, payload);

  http.end();
}

// Update the led and the temperature and humidity from the DHT11 sensor
// Every DATA_REFRESH_INTERVAL_SECONDS seconds (see config-mcu.h)
void updateData() {
  updateLed();
  pushDHT();
}

// ---------------------------------------------------------------

// Arduino setup function
void setup() {
  Serial.begin(115200);

  // Register to the WiFiMulti
  Serial.println("Connecting to " WIFI_SSID " PWD:" WIFI_PASSWORD);
  wifiMulti.addAP(WIFI_SSID, WIFI_PASSWORD);

  for (uint8_t t = 4; t > 0; t--) {
    Serial.printf("[SETUP] WAIT %d...\n", t);
    Serial.flush();
    delay(1000);
  }

  // Enable builtin led
  Serial.println("[SETUP] Enable Led");
  pinMode(LED_BUILTIN, OUTPUT);
  digitalWrite(LED_BUILTIN, HIGH);

  // Add a timer to update the data every DATA_REFRESH_INTERVAL_SECONDS
  // This avoid to send the data every time the code is executed and
  // to free the CPU
  Serial.println("[SETUP] Set timer...");
  TimerLib.setInterval_s(updateData, DATA_REFRESH_INTERVAL_SECONDS);

  Serial.println("[SETUP] DONE! Waiting for connection...");

  delay(100);
}

// Wait until the wifi is connected
void loop() {
  // wait for WiFi connection
  if ((wifiMulti.run() == WL_CONNECTED) && (is_wifi_connected == false)) {
    Serial.println("[LOOP] Connected");
    Serial.flush();
    is_wifi_connected = true;
  }
}
