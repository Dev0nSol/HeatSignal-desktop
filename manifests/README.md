# Manifests Folder

This folder holds:
- `pubkeys.md` and `pubkeys.json` — your **public** minisign keys (never commit the secret key).
- `latest.json` — the **signed** update manifest that HeatSignal will fetch from Releases.

## Steps to publish a release

1. Build your Windows installer (e.g., `HeatSignal-0.1.0-setup.exe`).
2. Compute its **SHA-256** (Windows PowerShell):
   ```powershell
   Get-FileHash -Path .\HeatSignal-0.1.0-setup.exe -Algorithm SHA256
   ```
3. Create `latest.json` using `manifests/latest.template.json` as a starting point.
4. **Sign** `latest.json` with minisign (see `UPDATES.md`) and embed the Base64 signature in `signature_ed25519`.
5. Upload both the installer and `latest.json` to the GitHub Release.

> The app verifies both the signature and the SHA-256 before installing updates.
