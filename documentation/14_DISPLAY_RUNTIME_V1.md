# ONA Gaming Display Runtime V1

ONA does not render native games inside the Shell. ONA launches an external process and hands the visual experience to that process.

## ONA Gaming Display

The ONA Gaming Display is the display where the Shell is placed for the console experience. This target is selected by ONA, not by the game manifest and not by the Windows primary monitor.

On Windows, ONA uses the Win32 monitor device name, such as:

```text
\\.\DISPLAY2
```

That value comes from the `MONITORINFOEXW.szDevice` monitor associated with the selected ONA Gaming Display. Geometry and DPI metadata remain available for verification and for engines such as winit, but the device name is the primary display identity when Windows provides it.

If a native device name is unavailable, ONA falls back to a geometry identifier:

```text
{monitor-name}@{x},{y}:{width}x{height}
```

The fallback is paired with explicit geometry so a native game can match the correct `winit` monitor without relying only on display indexes.

## Runtime Environment

ONA Compatible games launched by ONA receive:

- `ONA_RUNTIME=1`
- `ONA_PROTOCOL_VERSION=1`
- `ONA_INPUT_HOST`
- `ONA_INPUT_PORT`
- `ONA_LIFECYCLE_HOST`
- `ONA_LIFECYCLE_PORT`
- `ONA_DISPLAY_MODE`
- `ONA_DISPLAY_ID`
- `ONA_DISPLAY_NAME` when available
- `ONA_DISPLAY_X`
- `ONA_DISPLAY_Y`
- `ONA_DISPLAY_WIDTH`
- `ONA_DISPLAY_HEIGHT`
- `ONA_DISPLAY_SCALE_FACTOR`
- `ONA_DISPLAY_TARGET`
- `ONA_PLAYER_ID` when available

`ONA_DISPLAY_MODE` is:

- `CONSOLE_FULLSCREEN` when the manifest requests fullscreen.
- `WINDOWED` when the manifest does not request fullscreen.

`ONA_DISPLAY_TARGET` is the authoritative ONA Gaming Display identifier for compatibility with early V1 games. `ONA_DISPLAY_ID` carries the same identifier with a clearer name.

## Lifecycle Handshake

The lifecycle bridge is a localhost TCP channel separate from the input bridge. Games connect to `ONA_LIFECYCLE_HOST:ONA_LIFECYCLE_PORT` and send newline-delimited signals:

```text
GAME_STARTED
GAME_WINDOW_READY
GAME_DISPLAY_READY
GAME_READY
GAME_EXITING
```

JSON is also accepted:

```json
{"event":"GAME_DISPLAY_READY"}
```

`GAME_DISPLAY_READY` means the game has created or moved its own window/render surface to the requested ONA Gaming Display and applied the expected display mode.

`GAME_READY` means the game has completed the console handoff contract: the window exists, the ONA Gaming Display is active, fullscreen or borderless presentation has been applied, the first frame is ready, and the game can receive player interaction.

## Handoff

ONA keeps the Shell visible during launch until one of these happens:

- The game sends `GAME_READY` and ONA verifies the game window is on the target display.
- Legacy fallback confirms a visible top-level game window is on the target display.

If a visible game window exists but is not on the ONA Gaming Display, ONA does not hide. The failed launch is rolled back: ONA terminates only the child process it owns, clears runtime state, restores the Shell, and leaves PLAY available.

## Input Bridge V1

The input bridge is newline-delimited JSON over localhost TCP. Each event is one JSON object followed by `\n`.

Canonical payloads:

```json
{"kind":"Joystick","playerId":1,"x":0.25,"y":-0.5}
{"kind":"Button","playerId":1,"button":"A","state":"down"}
{"kind":"Button","playerId":1,"button":"A","state":"up"}
{"kind":"Button","playerId":1,"button":"START","state":"down"}
{"kind":"Button","playerId":1,"button":"START","state":"up"}
```

Buttons are `A`, `B`, `X`, `Y`, `L1`, `L2`, `R1`, `R2`, `SELECT`, and `START`. Button states are `down` and `up` for normal gameplay.

## Process Ownership

ONA owns the process it launches. On Windows, ONA attempts to place the child process in a Job Object configured with kill-on-job-close. ONA also terminates the owned child explicitly during rollback, user shutdown, and launcher drop.

Shutdown order:

```text
ONA_SHUTDOWN lifecycle signal
short grace period
terminate owned child if still alive
clear runtime state
close ONA
```

## Responsibilities

ONA is responsible for launch, runtime environment, input bridge, lifecycle bridge, handoff timing, process monitoring, uninstall protection, and restoring the Shell to the ONA Gaming Display after game exit.

The game is responsible for selecting the target monitor, applying borderless fullscreen or its engine-specific equivalent, choosing resolution, and sending `GAME_READY`.

Windows remains the host operating system. ONA must not kill Explorer, force Alt+Enter, rewrite arbitrary game window styles, or simulate input to force display modes.
