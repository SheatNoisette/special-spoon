"""
Bridge MQTT to HTTP REST API
"""

from requests import get, post
import paho.mqtt.client as mqtt
import threading
import time

MQTT_BROKER = "broker.hivemq.com"
MQTT_PORT = 1883

REST_SERVER = "localhost"
REST_PORT = 8000

PREFIX_TOPIC = "flan-bridge-spoonfull"
LED_TOPIC = PREFIX_TOPIC + "/led"
HUMIDITY_TOPIC = PREFIX_TOPIC + "/humidity"
TEMPERATURE_TOPIC = PREFIX_TOPIC + "/temperature"

last_led_status = None

def get_led_status() -> bool:
    """
    Get LED status from REST API
    """
    print("<- LED STATUS")
    response = get(f"http://{REST_SERVER}:{REST_PORT}/led")
    return response.json()["status"]

def set_led_status(status:bool):
    """
    Set LED status on REST API
    """
    print("-> LED STATUS:", status)
    led_status = "on" if status else "off"
    response = post(f"http://{REST_SERVER}:{REST_PORT}/led/{led_status}")
    return response

def publish_led_status(client):
    """
    Publish LED status to MQTT broker
    """
    print("<= LED STATUS")
    client.publish(LED_TOPIC, "on" if get_led_status() else "off", 2)

def format_json(payload: str, payload_value) -> str:
    return {"ip": "flan-bridge", str(payload): payload_value}

def push_humidity(humidity:float):
    """
    Push humidity to REST API
    """
    print("-> HUMIDITY:", humidity)
    return post(f"http://{REST_SERVER}:{REST_PORT}/humidity", json=format_json("humidity", humidity))

def push_temperature(temperature:float):
    """
    Push temperature to REST API
    """
    print("-> TEMPERATURE:", temperature)
    return post(f"http://{REST_SERVER}:{REST_PORT}/temperature", json=format_json("temperature", temperature))

def on_connect(client, userdata, flags, rc):
    print("=> MQTT BROKER CONNECTED - CODE:", str(rc))

    print("=> SUBSCRIBE TO TOPICS")
    client.subscribe(LED_TOPIC, 2)
    client.subscribe(HUMIDITY_TOPIC, 2)
    client.subscribe(TEMPERATURE_TOPIC, 2)
    print("=> SUBSCRIBE TO TOPICS - DONE")

def on_publish(client, userdata, mid):
    print("=> MQTT PUBLISHED - MESSAGE ID:", str(mid))

def on_message(client, userdata, msg):
    print("=> MQTT MESSAGE:", msg.topic, msg.payload)

    if msg.topic == LED_TOPIC:
        # Client is requesting LED status
        if msg.payload == b"?":
            publish_led_status(client)
        else:
            set_led_status(msg.payload == b"on")
        return

    payload_float = 0.0
    try:
        payload_float = float(msg.payload)
    except ValueError:
        print("=> ERROR:", msg.payload, "is not a float")
        return

    if msg.topic == HUMIDITY_TOPIC:
        push_humidity(payload_float)
    elif msg.topic == TEMPERATURE_TOPIC:
        push_temperature(payload_float)

def on_connect_failed(client, userdata, flags, rc):
    print("=> MQTT CONNECT FAILED - CODE:", str(rc))

class MonitorLedThread(threading.Thread):
    def __init__(self, client):
        threading.Thread.__init__(self)
        self.client = client

    def run(self):
        global last_led_status
        while True:
            led_status = get_led_status()
            if led_status != last_led_status:
                print("=> LED STATUS CHANGED:", led_status)
                last_led_status = led_status
                publish_led_status(self.client)
            time.sleep(4)

if __name__ == "__main__":
    print("=> BRIDGE STARTED")

    print("-> TESTING REST API")
    try:
        get(f"http://{REST_SERVER}:{REST_PORT}/")
    except:
        print("-> REST API NOT WORKING")
        exit(1)
    print("-> REST API SEEMS TO BE WORKING")

    print("=>  INIT MQTT CLIENT")

    client = mqtt.Client()
    client.on_connect = on_connect
    client.on_message = on_message
    client.on_connect_fail = on_connect_failed
    client.on_publish = on_publish

    client.connect(MQTT_BROKER, MQTT_PORT, 60)

    # Start thread
    print("=> STARTING THREAD")
    thread = MonitorLedThread(client)
    thread.start()

    client.loop_forever()