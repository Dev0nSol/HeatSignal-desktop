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
- **No telemetry.** No background sync. No remote user database in V1.

## Local storage

- **Database**: `%LOCALAPPDATA%\HeatSignal\data\heatsignal.db`  
  - Encrypted with your **passphrase** (SQLCipher).  
  - Passphrase → **Argon2id** (m=64 MB, t=3, p=1) → key.  
  - Passphrase is requested for sensitive actions (Scan/Refresh/Export), remembered for the session, **auto-locks after 15 min idle**, and locks on OS sleep/lock.
- **API keys**: stored with **Windows DPAPI** (per-user encryption).
- **Caches**: market snapshots purge after **7 days**.
- **Logs**: local-only, **redacted** (wallets shortened, tx sigs hashed). Rotate **20 × 50 MB** (≈1 GB max). No UI buttons to expose logs in V1.

## Exports (user-initiated only)

- NDJSON written to:  
  `%LOCALAPPDATA%\HeatSignal\exports\{YYYY-MM-DD}\{addressShort}_{YYYYMMDD-HHMMSS}.ndjson`
- First line is meta; then **one line per (wallet, mint)** aggregate.  
- Amounts are **USD** (strings with two decimals).  
- Export includes the **wallet address** (base58).  
- Export is **blocked** if the selected lookback has **< 5 buys**.

## Phantom & signing

- Phantom connect opens in your default browser; returns to  
  `http://127.0.0.1:<port>/phantom/callback?...` with strict **state** and 2-minute expiry.
- One-time **ownership signature**: session challenge (UUID + nonce + issued-at), valid **60 minutes**; stored **in memory only**.

## Wipe & uninstall

- **Wipe Data** requires typing **HEATSIGNAL** **and** entering your DB passphrase.  
- Uninstall prompts **“Remove all app data”** (checked by default). If checked, DB/caches/logs/settings are removed. **Exports are never auto-deleted.**

## Demo mode

- Optional, randomized sample data. **Export disabled**; **DEMO** banner displayed. Real wallets/data are hidden while Demo is on.

## Network allowlist (fixed in V1)

- `api.helius.xyz`, `mainnet.helius-rpc.com`  
- `public-api.birdeye.so`  
- `api.dexscreener.com`  
- `github.com`, `objects.githubusercontent.com`
