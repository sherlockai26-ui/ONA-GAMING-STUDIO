# ONA Gaming Display Runtime V1

ONA does not render native games inside the Shell. ONA launches an external process and hands the visual experience to that process.

## ONA Gaming Display

The ONA Gaming Display is the display where the Shell is placed for the console experience. This target is selected by ONA, not by the game manifest and not by the Windows primary monitor.

ONA identifies the target display from the active monitor layout using the monitor name, virtual desktop position, physical size, scale factor, and primary role. The public runtime identifier is:

```text
{monitor-name}@{x},{y}:{width}x{height}
```

This identifier is stable enough for Runtime V1 and is paired with explicit geometry so a native game can match the correct `winit` monitor without relying only on display indexes.

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
```

JSON is also accepted:

```json
{"event":"GAME_DISPLAY_READY"}
```

`GAME_DISPLAY_READY` means the game has created or moved its own window/render surface to the requested ONA Gaming Display and applied the expected display mode.

## Handoff

ONA keeps the Shell visible during launch until one of these happens:

- The game sends `GAME_DISPLAY_READY`.
- Legacy fallback confirms a visible top-level game window is on the target display.

If a visible game window exists but is not on the ONA Gaming Display, ONA does not hide immediately.

## Responsibilities

ONA is responsible for launch, runtime environment, input bridge, lifecycle bridge, handoff timing, process monitoring, uninstall protection, and restoring the Shell to the ONA Gaming Display after game exit.

The game is responsible for selecting the target monitor, applying borderless fullscreen or its engine-specific equivalent, choosing resolution, and sending `GAME_DISPLAY_READY`.

Windows remains the host operating system. ONA must not kill Explorer, force Alt+Enter, rewrite arbitrary game window styles, or simulate input to force display modes.
