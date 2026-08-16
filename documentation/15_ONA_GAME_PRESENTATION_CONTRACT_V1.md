# ONA Game Presentation Contract V1

## Principle

ONA does not render a native game inside ONA Shell. ONA launches a native process, validates its Runtime V1 lifecycle signals, grants presentation ownership to the game's own window, and then steps out of the foreground.

The platform owns launch, validation, controller routing, system overlays, background sessions, minimize/restore coordination, and graceful shutdown. The game owns its renderer, window contents, frame pacing, engine, assets, menus, and gameplay.

## Runtime V1 State Boundary

The initial handoff is a one-time flow:

1. `Launching`
2. `WaitingForReady`
3. `GAME_STARTED`
4. `GAME_WINDOW_READY`
5. `GAME_DISPLAY_READY`
6. `GAME_READY`
7. SAFE HANDOFF accepted
8. `Running`

Once ONA reaches `Running`, the initial handoff overlay is cleared. System features such as Quick Menu, Home, Continue, Minimize, Restore, and Close must not restart launch validation unless a new game launch is explicitly requested.

## Presentation Owners

ONA Runtime V1 uses explicit presentation owners:

- `ONA_SHELL`: ONA Home, Library, Settings, Store, controller screens.
- `ONA_TRANSITION_GUARD`: opaque guard during launch, return, stop, and failure transitions.
- `GAME`: the native game window owns the foreground experience.
- `ONA_SYSTEM_OVERLAY`: ONA Quick Menu is visible over a running game.
- `ONA_MINIMIZED`: the complete console experience is minimized.

There is no implicit `None` owner during an active native game session.

## Host Presentation Adapter

The Host Presentation Adapter is the platform-facing contract for window ownership:

- discover and remember native game process/window identity,
- focus the game window after SAFE HANDOFF,
- minimize/restore the game window when the console experience is minimized/restored,
- show/hide ONA system overlay without relaunching the game,
- keep cursor state explicit per owner,
- keep Shell, transition guard, Quick Menu, and game presentation as separate concepts.

Windows currently uses native HWND operations. Other hosts must provide equivalent process-window operations without changing the Game Manifest, Game Manager, Launcher, Lifecycle, Input Bridge, or Native Game contracts.

## Quick Menu

HOLD START is an ONA system command while a native game is running.

Quick Menu flow:

1. `Running`
2. pause game input forwarding
3. show `ONA_SYSTEM_OVERLAY`
4. open Quick Menu
5. close Quick Menu
6. hide system overlay
7. restore cursor/game focus
8. resume game input forwarding
9. return directly to `Running`

Quick Menu must not show `Launching`, `WaitingForReady`, or `GAME_READY`.

## START Input

Short START remains a game input. ONA may observe START down/up to detect HOLD START, but a short press must continue to flow:

Controller -> WebSocket -> ONA Core -> Input Dispatcher -> Input Bridge TCP -> Native Game

When HOLD START is detected, ONA opens Quick Menu and consumes the system command. The initial START down may already have reached the game; ONA must avoid generating duplicate short actions when the hold is released.

## Minimize / Restore

Minimize is a console presentation operation, not a Shell-only operation.

If a game is active, ONA requests minimization of the native game window and then minimizes ONA Shell. When ONA is restored, the previous presentation owner is restored directly:

- previous `GAME` -> focus/restore game, hide Shell, resume input forwarding.
- previous `ONA_SYSTEM_OVERLAY` -> restore game, show Quick Menu overlay, keep input forwarding paused.
- previous `ONA_SHELL` or `Background` -> restore Shell state.

Restore must not run a new handoff, launch a new process, or terminate the existing game.

## Local Controller Connectivity

Runtime V1 does not require Internet connectivity. The controller should work over the local network using the QR URL served by ONA Core.

Direct Local hotspot mode is a future host capability. ONA may detect/report host support, but Runtime V1 does not automatically enable a Windows hotspot.

## Game Independence

A compatible game must not depend on repository paths or ONA Shell internals. It should depend only on:

- files copied into its installed local game directory,
- manifest-declared executable, working directory, and arguments,
- Runtime V1 environment variables,
- lifecycle bridge TCP messages,
- input bridge TCP messages,
- documented presentation requirements.

This keeps the same infrastructure usable for small native games, ONA first-party titles, and future AAA engines.
