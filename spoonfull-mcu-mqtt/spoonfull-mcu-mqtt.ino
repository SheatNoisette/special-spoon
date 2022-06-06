#include <uTimerLib.h>
#include "EspMQTTClient.h"
#include <DFRobot_DHT11.h>

// Wifi password
#define WIFI_SSID "Hey"
#define WIFI_PASSWORD "z8ui9afj"

#define BROKERID "broker.hivemq.com"
#define BROKER_PORT 1883
#define DATA_RATE_S (60 * 1)
#define DHT11_PIN 23

#define TEMPERATURE_TOPIC "flan-bridge-spoonfull/temperature"
#define HUMIDITY_TOPIC "flan-bridge-spoonfull/humidity"
#define LED_TOPIC "flan-bridge-spoonfull/led"

DFRobot_DHT11 DHT;

bool led_status = false;
bool is_mqtt_available = false;

EspMQTTClient client(
  WIFI_SSID,
  WIFI_PASSWORD,
  BROKERID,
  "SpoonfullMCU",
  BROKER_PORT
);

void subscribe_led(const String payload){
   if (payload == "?") return;
   if (payload == "on") {
      led_status = true;
   } else if (payload == "off") {
      led_status = false;
   }
}

void dht_publish() {
  if (is_mqtt_available == false) return;

  DHT.read(DHT11_PIN);

  float temp = DHT.temperature;
  float humid = DHT.humidity;

  client.publish(TEMPERATURE_TOPIC, String(temp));
  client.publish(HUMIDITY_TOPIC, String(humid));
}

// This function is called once everything is connected (Wifi and MQTT)
// WARNING : YOU MUST IMPLEMENT IT IF YOU USE EspMQTTClient
void onConnectionEstablished()
{
  // Subscribe to LED
  client.subscribe(LED_TOPIC, subscribe_led);

  // Ask the status of the LED
  client.publish(LED_TOPIC, "?");

  is_mqtt_available = true;
}


void setup()
{
  Serial.begin(115200);
  client.enableDebuggingMessages(); // Enable debugging messages sent to serial output

  // Intialize the built-in LED
  pinMode(LED_BUILTIN, OUTPUT);

  // Set the callback function
  TimerLib.setInterval_s(dht_publish, DATA_RATE_S);
}

void loop()
{
  digitalWrite(LED_BUILTIN, led_status == true ? HIGH : LOW);
  client.loop();
}
