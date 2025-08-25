# Architecture — HeatSignal V1

## Stack

- **Shell/UI**: **Tauri** (Rust) + WebView2 (offline installer bundled).
- **Backend**: **FastAPI** (Python **3.11**), bundled via **PyInstaller** into a self-contained EXE.
- **DB**: SQLite (**SQLCipher** encryption).
- **Installer**: NSIS `.exe` (per-user install).
- **Single-instance**: yes (focus existing window on second launch).

## Processes & ports

- Tauri launches the FastAPI EXE on a **random local port** (`127.0.0.1:<port>`).
- UI talks to FastAPI with a per-session **nonce** header; FastAPI enforces **same-origin + nonce**.
- No external listeners; no fixed ports.

## Core flows

### Phantom connect
1. UI opens default browser for Phantom.
2. User signs a **session challenge** (UUID + nonce + issued-at), valid **60 minutes**, stored **in memory** only.
3. Phantom redirects to `http://127.0.0.1:<port>/phantom/callback?...` with strict **state** (single-use, 2-minute TTL).
4. Backend verifies state + signature; saves wallet (nickname-able).  
   - Only **one active wallet per session**; up to **10** stored total.

### Scan / Refresh
- **Refresh behavior**: **incremental rescan + re-rank** (fetch only new txs since `last_scanned_slot`).
- **Providers**:  
  - **Helius** (tx parse + labels) and **Helius RPC** (balances).  
  - **Birdeye** (prices, liquidity, 24h vol, market cap, holders growth).  
  - **DexScreener** used **only** when Birdeye data is missing.
- **What counts as a buy**  
  - AMM swaps **and** orderbook fills.  
  - Token→token swaps allowed (we price the **source** asset).  
  - Aggregator routes (e.g., Jupiter) **only if** the **final leg** is on a **whitelisted DEX**.
- **Whitelist** (initial): Raydium AMM, Orca Whirlpool, Phoenix, Meteora, Lifinity, Crema, OpenBook.
- **Exclusions**: NFTs, **LP tokens** (excluded from recs **and** export), stablecoins & SOL wrappers from recs (still appear in export).
- **USD at tx time**: **5-minute OHLC** nearest bucket; if unavailable → **current price (estimated)**.
- **Owned detection**: current balance > **$5** USD (dust threshold); stables/wrappers ignored.
- **Caching**: Birdeye TTL ~5 minutes; market cache purged after **7 days**.
- **Caps**: candidate pool **500** (see below); scan fetch cap **5,000 tx** per run.

### Candidate pool (500)
- **Hybrid** union:  
  - Top by **24h volume** and **7d volume gain** (pre-filtered), plus  
  - Tokens with **recent swaps in last 48h**.  
- De-dup and apply safety/thresholds; cap at **500** total.  
- Split: **350 volume-based / 150 recent-activity**.

## Scoring & recommendation

### HEAT score (0–100)
- 28% **7d volume trend** (vs prior 7d)  
- 22% **liquidity**  
- 18% **market cap** (prefer **circulating**; fallback **FDV**; **min $1M**; **unknown ⇒ exclude**)  
- 14% **(1 − volatility)** using 7d high–low ÷ mean (buckets: **low <0.15**, **0.15–0.35**, **>0.35**)  
- 10% **DEX count**  
- 8% **holders growth 7d**  
- **New token penalty**: age <14 days ⇒ up to **−30** linear (day 0 → day 14)

### Safety gates
- Exclude if aggregator flags (**scam/honeypot/trading-disabled**) or suspicious LP if detectable.
- **Verified ✓** badge requires:  
  **no flags AND mint authority renounced/immutable AND freeze authority null AND DEX count ≥ 2**.  
  Otherwise show **Safety unknown ⚠️** (still eligible if it passes filters).

### Eligibility filters (V1 defaults)
- **Lookback**: presets **7–84 days**, **rolling window** to “now”.
- **Liquidity ≥ $75k** (adjustable).  
- **24h volume ≥ $20k** (adjustable).  
- **DEX count ≥ 2** (**hard**, unknown ⇒ exclude).  
- **Market cap ≥ $1M** (Birdeye; fallback DexScreener; unknown ⇒ exclude).  
- Exclude tokens the user **already owns** (>$5).  
- **HEAT score ≥ 60** and **similarity ≥ 0.40**.  
- Tie-break by **higher 7d volume**.

### Similarity (profile-based)
- Build a **user vector** with **45-day half-life** recency decay over buys:
  - Niche share + normalized liquidity & volatility.
- Token vector: niche one-hot + normalized liquidity & volatility.
- Normalize (liquidity, volatility) via **z-score → logistic**.
- **Cosine similarity** with weights: **niche 0.7**, **liquidity 0.2**, **volatility 0.1**.
- Related-niche map (0.6 match) approved (e.g., DeFi↔Infra/Tools, Meme↔SocialFi/Gaming…).

### Final rank
`0.6 × similarity + 0.4 × normalized HEAT score`  
Show **Top 1** and **Next best (up to 2)** (condensed; expand for details).  
If **no match** → empty state + **Widen search** (temporary: 84d, liq ≥ $50k, vol ≥ $10k, score ≥ 55, sim ≥ 0.35, same safety/DEX rules).

## Export

- **Aggregated NDJSON**, one line per **(wallet, mint)** for the current lookback.  
- **USD only** in file.  
- Path: `%LOCALAPPDATA%\HeatSignal\exports\{YYYY-MM-DD}\{addrShort}_{YYYYMMDD-HHMMSS}.ndjson`  
- Block export if **< 5 buys** in current lookback.  
- Multi-wallet combined export supported (still one line per `(wallet, mint)`).

## Settings & security

- **Passphrase** protects the DB (SQLCipher; **Argon2id m=64MB, t=3, p=1**).  
- Unlock on sensitive actions; **auto-lock after 15 minutes idle**; lock on OS sleep/lock.  
- **Manual FX rate** for EUR (blank by default; link to sources).  
- Outbound allowlist fixed:
  - `api.helius.xyz`, `mainnet.helius-rpc.com`
  - `public-api.birdeye.so`
  - `api.dexscreener.com`
  - `github.com`, `objects.githubusercontent.com`

## Updates

- **Signed manifest** (`latest.json`) with embedded **Ed25519** signature (minisign).  
- Two embedded public keys + **manual key-trust** prompt for rollover.  
- **Install from file** verifies SHA-256 against manifest; **downgrades blocked**.  
- If offline: use **cached manifest ≤7 days**, else allow **manual `latest.json`** load.

## Files & paths

- DB: `%LOCALAPPDATA%\HeatSignal\data\heatsignal.db`  
- Backups (migrations): `%LOCALAPPDATA%\HeatSignal\backups\{YYYYMMDD-HHMMSS}.heatsignal.db` (keep **last 5**)  
- Exports: `%LOCALAPPDATA%\HeatSignal\exports\...` (history list keeps **20** entries; never auto-delete)  
- Logs: `%LOCALAPPDATA%\HeatSignal\logs\` (rotate **20 × 50MB**)

## Build & release

- CI: GitHub Actions builds Windows installer; drafts release and prints **SHA-256**.
- You sign `latest.json` **locally** with **minisign** and attach it to the release.
- Installer filename: `HeatSignal-<version>-setup.exe`.
