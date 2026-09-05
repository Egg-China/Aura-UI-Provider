#!/usr/bin/env python3
"""Packages the Aura Modern UI provider as a schema-v5 .npl archive.

The archive embeds ``plugin.json`` plus the platform provider binary under
``bin/``. Entry timestamps are pinned to 1980-01-01 so repackaging the same
inputs is byte-stable, which keeps SHA-256 release manifests meaningful.
"""

import argparse
import hashlib
import json
import pathlib
import sys
import zipfile

ROOT = pathlib.Path(__file__).resolve().parents[1]
EPOCH = (1980, 1, 1, 0, 0, 0)

PLATFORM_ENTRYPOINTS = {
    "windows-x64": "bin/aura-ui-provider.exe",
    "linux-x64": "bin/aura-ui-provider",
    "macos-x64": "bin/aura-ui-provider",
    "macos-arm64": "bin/aura-ui-provider",
}

EXECUTABLE_NAMES = {
    "windows-x64": "aura-ui-provider.exe",
    "linux-x64": "aura-ui-provider",
    "macos-x64": "aura-ui-provider",
    "macos-arm64": "aura-ui-provider",
}


def sha256(path: pathlib.Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--platform", required=True, choices=sorted(PLATFORM_ENTRYPOINTS))
    parser.add_argument("--binary", type=pathlib.Path, default=None)
    parser.add_argument("--output", type=pathlib.Path, default=None)
    arguments = parser.parse_args()

    entrypoint = PLATFORM_ENTRYPOINTS[arguments.platform]
    executable_name = EXECUTABLE_NAMES[arguments.platform]
    binary = arguments.binary or ROOT / "target" / "release" / executable_name
    if not binary.is_file():
        print(f"missing provider binary: {binary}", file=sys.stderr)
        return 1

    manifest_path = ROOT / "package" / "plugin.json"
    manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
    manifest["entrypoint"] = entrypoint
    manifest["platforms"] = [arguments.platform]

    output = arguments.output or (ROOT / "dist" / f"dev.aura.modern-ui.{arguments.platform}.npl")
    output.parent.mkdir(parents=True, exist_ok=True)

    with zipfile.ZipFile(output, "w") as archive:
        info = zipfile.ZipInfo("plugin.json", date_time=EPOCH)
        info.compress_type = zipfile.ZIP_DEFLATED
        archive.writestr(info, json.dumps(manifest, indent=2, ensure_ascii=False) + "\n")
        info = zipfile.ZipInfo(entrypoint, date_time=EPOCH)
        info.compress_type = zipfile.ZIP_DEFLATED
        info.external_attr = 0o755 << 16
        archive.writestr(info, binary.read_bytes())

    digest = sha256(output)
    checksum_path = output.with_suffix(output.suffix + ".sha256")
    checksum_path.write_text(f"{digest}  {output.name}\n", encoding="utf-8")
    print(f"packaged {output} ({output.stat().st_size} bytes)")
    print(f"sha256 {digest}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
