# Updates & Manifest Security

## Where

Default URL (fixed):  
`https://github.com/Dev0nSol/HeatSignal-desktop/releases/latest/download/latest.json`

## Manifest format (example)

```json
{
  "version": "0.1.0",
  "published_at": "2025-08-24T00:00:00Z",
  "channel": "stable",
  "files": [{
    "os": "windows",
    "arch": "x64",
    "filename": "HeatSignal-0.1.0-setup.exe",
    "size": 12345678,
    "sha256": "PUT_SHA256_OF_EXE"
  }],
  "signature_ed25519": "PUT_MINISIGN_SIG_BASE64"
}
```

- **Embedded `signature_ed25519`** signs the manifest body.  
- App also verifies the **installer SHA-256** after you select the file.

## Key management

- App ships with **two public keys** (primary + backup).  
- **Rollover**: if manifest is signed by an **unknown** key → app shows fingerprints + links and requires **one-click “Trust this key”** to add it.

### Publish steps (you)

1) CI builds `HeatSignal-<version>-setup.exe` and drafts a GitHub Release, attaching the installer and printing its **SHA-256**.  
2) Locally create/update `latest.json` with the new version + SHA.  
3) **Sign** with minisign and embed Base64 signature in `signature_ed25519`.  
4) Attach `latest.json` to the Release; publish.

### minisign quickstart

```bash
# one-time: generate (keep secret key offline)
minisign -G -p pubkey.txt -s heatsignal-updates.key

# sign
minisign -S -s heatsignal-updates.key -m latest.json -x latest.json.minisig

# embed the signature's Base64 into latest.json.signature_ed25519
```

## Install from file

- **Requires** a manifest match: either freshly fetched or cached ≤ 7 days (**or** user can load a `latest.json` manually).  
- **Blocks downgrades** (must be higher version).  
- After verify → **confirm modal** → app closes → installer runs → app relaunches.

## Hosts allowlist

- `github.com` and `objects.githubusercontent.com` for Releases download.
