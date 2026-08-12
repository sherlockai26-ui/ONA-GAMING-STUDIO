# ONA Game Runtime V1

Status: experimental

ONA Game Runtime V1 is the minimal launch contract that lets an external game discover the ONA Game Input Bridge when it is started by ONA.

## Environment Variables

ONA sets these variables only on the child game process:

```text
ONA_RUNTIME=1
ONA_PROTOCOL_VERSION=1
ONA_INPUT_HOST=127.0.0.1
ONA_INPUT_PORT=<real input bridge port>
```

ONA does not modify global Windows environment variables.

## Input Bridge Endpoint

Games should connect to:

```text
tcp://ONA_INPUT_HOST:ONA_INPUT_PORT
```

The port is owned by ONA. Games must not hardcode it.

## Current JSON Input Format

Joystick:

```json
{"kind":"joystick","playerId":1,"x":0.0,"y":0.0}
```

Button:

```json
{"kind":"button","playerId":1,"button":"A","state":"down"}
```

Button states are `down` and `up`.

## Lifecycle

1. ONA starts the Game Input Bridge.
2. ONA launches the game process.
3. ONA injects the `ONA_*` variables into that child process.
4. The game reads the variables and connects to the TCP input endpoint.
5. ONA streams newline-delimited JSON input events.

## Running Outside ONA

If a game is launched outside ONA, these variables may be missing.

In that case, the game should run normally without ONA input, or show its own controller configuration flow.

This document does not define a full SDK.
