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


## Release matrix

Supported packages are built for `windows-x64`, `linux-x64`, `macos-x64`,
and `macos-arm64`. Tag a release (`v*`) or run the *Release Packages*
workflow manually; every platform uploads its `.npl` plus a SHA-256 sidecar,
and a combined `SHA256SUMS.txt` manifest is attached as the
`release-checksums` artifact.

Platforms without maintained builders — arm32 Linux, riscv64, loongarch64,
FreeBSD, and HarmonyOS — are intentionally not distributed. Build them
yourself with the toolchain below; the protocol is pure data, so any
platform that can compile Rust and Qt can host it.

## Self-build guide

```bash
# 1. Frontend (Node 20+)
cd frontend
npm ci
npm run build

# 2. Native provider (Rust stable)
cd ..
cargo build --release --manifest-path provider/Cargo.toml

# 3. Package the launcher plugin
python scripts/package_npl.py --platform windows-x64
# on Linux/macOS: --platform linux-x64 | macos-x64 | macos-arm64
```

Linux additionally needs `libwebkit2gtk-4.1-dev`, `libgtk-3-dev`,
`libayatana-appindicator3-dev`, and `librsvg2-dev`. The packaged
`.npl` installs through Aura Launcher's plugin store or the plugins
directory; restart the launcher and select *Aura Modern UI* under
Settings → UI Frontend.
