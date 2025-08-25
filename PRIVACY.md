# Privacy — HeatSignal V1

**Local-first, privacy-by-default.** All heavy lifting happens on your machine.

## Data flows

- **Local compute**: wallet scan, buy parsing, market snapshots, scoring, similarity, recommendations.
- **Outbound requests (allowlisted only)**:
  - **Helius**: read-only transactions + labels (required)
  - **Helius RPC**: balances for “owned” detection ($5 dust threshold)
  - **Birdeye**: prices/liquidity/volume, 5-min TTL cache (required)
  - **DexScreener**: only if Birdeye is missing for a token
  - **GitHub Releases**: update manifest (`latest.json`)
- **No telemetry**. No background sync. No remote user database in V1.

## Local storage

- **DB**: `%LOCALAPPDATA%\HeatSignal\data\heatsignal.db`
  - **Encrypted** with your **passphrase** (SQLCipher; KDF **Argon2id m=64MB, t=3, p=1**).
  - Passphrase requested on sensitive actions (Scan/Refresh/Export), remembered for the session, **auto-lock after 15 min idle**, lock on OS sleep/lock.
- **API keys**: stored via **Windows DPAPI** (per-user encryption).
- **Caches**: market snapshots purge after **7 days**.
- **Logs**: local-only, **redacted** (addresses shortened, tx sigs hashed). Rotate **20×50MB** (≈1GB max). No UI buttons to expose logs.

## Exports (user-initiated only)

- NDJSON written to: `%LOCALAPPDATA%\HeatSignal\exports\{YYYY-MM-DD}\{addrShort}_{YYYYMMDD-HHMMSS}.ndjson`
- First line meta + per-token aggregates. Always **USD** amounts in file (2-decimal strings).
- Export includes **wallet base58 address** (you chose plain).
- You can **blocklist** tokens locally (won’t be recommended; still appear in export).

## Phantom & signing

- Phantom connect opens in browser; returns to `http://127.0.0.1:<port>/phantom/callback?...` with strict **state** and 2-min expiry.
- One-time **ownership signature**: session challenge (UUID + nonce + issued-at), valid **60 minutes**; stored in memory only.

## Wipe & uninstall

- **Wipe** requires typing **HEATSIGNAL** **and** your DB passphrase.  
- Uninstall offers “Remove all app data” (**checked by default**) — deletes DB/caches/logs/settings. Exports are **never auto-deleted**.

## Demo mode

- Optional, randomized sample data. **Export disabled**; **DEMO** banner displayed. Real wallets/data hidden.
