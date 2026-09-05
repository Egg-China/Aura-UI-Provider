# Aura UI Provider

Isolated native UI frontend for Aura Launcher implementing the frozen
`aura.ui.v1` process protocol (ABI 1).

## Status

- Milestone 1 (this repository): complete supervision-safe protocol host.
  Handshake, snapshot ingestion, readiness, launcher notifications, one
  launcher-state request, and clean shutdown all speak the exact wire bytes
  enforced by `UiFrontendProcessSession`.
- Milestone 2: Tauri 2 + Vue window attached above the protocol host.

## Protocol notes

- Frames are 4-byte big-endian length prefixes around canonical tagged
  MessagePack Bridge Values (`0x92, tag, payload` with `str32`/`bin32`/
  `array32` markers).
- Launcher requests use odd identifiers; this frontend uses even identifiers.
- The only accepted argv is `--stdio`; diagnostics go to stderr because stdout
  is the binary wire.

## Build

```powershell
cargo build --release
```

## Package

Build the release binary, then zip it with `package/plugin.json` as an
`Aura-Launcher` schema-v5 `.npl` UI-provider package:

The milestone-one distribution targets `windows-x64`, so the manifest
entrypoint is `bin/aura-ui-provider.exe` and the executable name inside the
archive must match it exactly:

```
dev.aura.modern-ui.npl
├── plugin.json
└── bin/aura-ui-provider.exe
```

The packaged archive has been verified end-to-end against Aura Launcher:
plugin discovery, SHA-256 verified extraction, permission gating, and a
complete supervised protocol session all succeed.
