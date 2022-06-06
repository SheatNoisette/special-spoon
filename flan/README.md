# Flan MQTT REST Bridge

Test:
```
python -m venv venv
source venv/bin/activate
```

Supports:

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

Topic:
```
flan-bridge-spoonfull/temperature
```

Playload supported: float formated like: 32.0

## humidity

Topic:
```
flan-bridge-spoonfull/humidity
```

Playload supported: float formated like: 32.0