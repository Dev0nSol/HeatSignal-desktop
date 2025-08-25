# Security — HeatSignal V1

## Threat model (summary)

- **Local app** with **strict allowlist** networking; no server-side session.
- Protect **API keys** (DPAPI) and **scanned data** (SQLCipher DB).
- **Manifest signing** prevents malicious updates; manual key rollover confirmation.

## Update trust

- `latest.json` is **Ed25519-signed** (minisign).  
- App ships **two embedded public keys** initially (primary + backup) *and* supports **manual rollover**:
  - If manifest is signed by an unknown key → block install and show a **Trust this key** prompt with fingerprints and links to:
    - `manifests/pubkeys.md` (human)
    - `manifests/pubkeys.json` (machine)
- **Downgrades blocked** (must install higher version).  
- Manifest pulled only on **Check for updates**.

## DB encryption

- SQLite with **SQLCipher**.  
- Passphrase → **Argon2id** (m=64MB, t=3, p=1) → key.  
- **No recovery**: if passphrase lost, data is irrecoverable (warned at setup).  
- Unlock required on sensitive actions; session unlock expires on idle/sleep.

## Local API hardening

- Runs on `127.0.0.1:<random>`.  
- **CSRF/nonce** header required; **same-origin** enforced; per-session token.  
- Swagger/OpenAPI **disabled**.  
- Max concurrency **4**; rate limiting/backoff on providers.

## Logging policy

- Keys never logged.  
- Wallets shortened (`abcd…wxyz`), tx signatures **hashed**.  
- Only counts/status codes; no PII.  
- Rotating files (20 × 50MB).  
- No UI export of logs in V1.

## Vulnerability reporting

Use **GitHub Security Advisories** on this repo. We’ll acknowledge within a reasonable timeframe and coordinate fixes via private advisories.
