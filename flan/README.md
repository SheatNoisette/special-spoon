# Flan MQTT REST Bridge

Flan is a MQTT REST bridge. It makes it easy to use MQTT to send and receive data from a REST API.

Make it running:
```
python -m venv venv
source venv/bin/activate
pip install -r requirements.txt
python flan-bridge.py
```

Supports the following topic:
- LED
- temperature
- humidity

You can edit the toml file to change the configuration of:
- MQTT server broker (default: HiveMQ)
- Prefix of the topic (default: "flan-bridge-spoonfull")
- The REST API server (default: "http://localhost:8000")

## LED

Topic:
```
flan-bridge-spoonfull/led
```

Payloads supported:
- on : Led is on
- off : Led is off
- ? : Ask the status of the led

## Temperature

Get the temperature of the DHT11 sensor.

Topic:
```
flan-bridge-spoonfull/temperature
```

Playload supported: float formated like: 32.0

## Humidity

Get the humidity of the DHT11 sensor.

Topic:
```
flan-bridge-spoonfull/humidity
```

Playload supported: float formated like: 32.0
