# Security — HeatSignal V1

## Threat model (summary)
- Local desktop app with a **fixed outbound allowlist**; no server-side session.
- Protect **API keys** (DPAPI) and **scanned data** (SQLCipher).
- **Signed update manifest** prevents malicious updates; manual key rollover confirmation.

## Update trust
- `latest.json` is **Ed25519-signed** with **minisign**.
- The app ships **two embedded public keys** (primary + backup) and supports **manual rollover**:
  - If the manifest is signed by an unknown key → block install and show fingerprints + links to:
    - `manifests/pubkeys.md` (human-readable)
    - `manifests/pubkeys.json` (machine-readable)
  - User must click **“Trust this key”** to add it.
- **Downgrades blocked** (only higher versions allowed).

## DB encryption
- SQLite with **SQLCipher**.
- Passphrase → **Argon2id** (`m=64MB, t=3, p=1`) → encryption key.
- Passphrase required for sensitive actions (Scan/Refresh/Export); session auto-lock after **15 min idle** and on **OS sleep/lock**.

## Local API hardening
- Backend listens on `127.0.0.1:<random>` (ephemeral).
- UI must send a per-session **nonce** header; FastAPI enforces **same-origin** + nonce.
- Swagger/OpenAPI endpoints are **disabled** in V1.

## Logging policy
- API keys are **never** logged.
- Wallet addresses **shortened** (`abcd…wxyz`); tx signatures **hashed**.
- Only counts/status codes and high-level events.
- Log rotation: **20 files × 50 MB** max (≈1 GB). No UI buttons to expose logs.

## Vulnerability reporting
Use **GitHub Security Advisories** on this repository.
