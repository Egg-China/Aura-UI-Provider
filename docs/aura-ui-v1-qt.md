# aura.ui.v1 Qt Host Interface

Aura Launcher keeps the UI protocol as a pure data contract. The default
Modern UI ships as a Tauri 2 + Vue provider; this document defines what a
Qt-based host must implement to register as an alternative `ui-provider`
plugin. Aura ships no Qt implementation — the launcher only freezes the wire
contract below.

## Process contract

- Launch argv is exactly `--stdio`.
- stdout carries only the binary wire; diagnostics go to stderr.
- Exit `0` after `ui.shutdown`; exit non-zero on any transport failure so the
  JavaFX supervision host can fall back.
- Launcher-originated requests use odd identifiers; frontend-originated
  requests use even identifiers starting at `2`.

## Frame format

Every message is a 4-byte big-endian length prefix followed by one encoded
Bridge Value envelope. Maximum frame size is 16 MiB.

## Value encoding (Bridge Value v1)

Each value is `[0x92, tag, payload]`:

| Tag | Type    | Payload                              |
|-----|---------|--------------------------------------|
| 0   | null    | `0xc0`                               |
| 1   | boolean | `0xc2` / `0xc3`                      |
| 2   | integer | `int64` (`0xd3`)                     |
| 3   | float   | `f64` (`0xcb`)                       |
| 4   | string  | `str32` (`0xdb`)                     |
| 5   | bytes   | `bin32` (`0xc6`)                     |
| 6   | array   | `array32` (`0xdd`) of full values    |
| 7   | map     | `array32` of `[bare key, full value]`|

Map keys are bare `str32` entries, never full tagged values.

## Envelope messages

- Request: `map{type:"request", requestId, method, params}`
- Result: `map{type:"result", requestId, value}`
- Error: `map{type:"error", requestId, code, message}`

## Session sequence

1. Launcher sends `ui.hello` (id 1) with `{protocol:"aura.ui.v1", abi:1}`;
   the host echoes the same object back as the result.
2. Launcher sends `ui.snapshot.replace` (id 3); the host answers null.
3. Host sends `ui.ready` (id 2) only after its window is interactive.
4. Host sends `core.snapshot.get` (id 4) and stores the result.
5. Launcher may push `ui.navigate` and `ui.notify` requests at any time.
6. Launcher sends `ui.shutdown`; the host replies, then exits `0`.

## Launcher command surface (host → launcher)

- `core.snapshot.get` — full state: `instances`, `accounts`, `settings`,
  `pluginContributions`.
- `core.instance.select` / `core.instance.launch` — `{id}` of a game instance.
- `core.plugin.action` — `{id}` of a registered contribution.
- `core.settings.get` / `core.settings.set` — typed settings allowlist.
- `core.app.shutdown` — terminate the launcher process tree.

## Package manifest

A Qt provider packages as a schema-v5 `.npl` with `pluginKind:"ui-provider"`,
`runtime:"aura-ui"`, `abi:1`, and the `launcher-ui-provider`, `native-code`,
and `process` permissions. The `entrypoint` must match the archived binary
path exactly (for example `bin/aura-ui-qt`).

## Qt implementation sketch

- Read stdin on a dedicated `QThread`; parse frames into `QVariant` trees.
- Emit parsed messages through a queued signal into the GUI thread.
- Write responses from a single serialized writer object.
- Treat `ui.navigate` as a route change and `ui.notify` as a toast.
- Exit via `qApp->quit()` after the shutdown reply is flushed.
