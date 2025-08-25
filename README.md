# HeatSignal (V1, Windows)

Private, local-only Solana wallet analysis with **one personalized token recommendation** and a clean export you can share later for V2. No accounts. No telemetry.

- **Local-only**: all scans, scoring, similarity, and UI run on your machine.
- **One click**: connect Phantom (browser), scan, see your top pick + 2 alternates.
- **Strict safety**: verified badge when checks pass; clear “unknown” if not.
- **Clean export**: aggregated **NDJSON** (wallet, lookback, token, niche, spend…) saved to your disk.

## Requirements

- **Windows 10 (build ≥ 19045) or Windows 11**, 64-bit  
- Phantom browser extension (Chrome/Brave/Edge/Firefox)  
- **Helius** API key (required) + **Birdeye** API key (required)  
- No Python required (bundled). **WebView2** offline installer is bundled.

## Install

1. Download `HeatSignal-<version>-setup.exe` from Releases.  
2. Run the installer (**per-user** install; Start Menu + Desktop shortcuts).  
3. Launch HeatSignal (auto-launch on “Finish”).

> First run is skippable onboarding: Privacy → Enter API keys → Defaults → (optional) Phantom connect.

## Quick start

1. Open **Settings → API Keys** and paste Helius + Birdeye. Click **Test**.  
2. **Dashboard → Connect Phantom** (opens your default browser).  
   - You’ll sign a **local session challenge** (60 min validity).  
3. Choose **lookback** (7–84d presets; default 7) and thresholds (default **$75k liq / $20k vol**).  
4. Click **Scan** (or **Refresh** later).  
5. Go to **Recommendations** → see **Top pick** (with score bar, safety badge, “Why this token”), plus **Next best** (up to 2 alternates).

## Export

- **Export Data** → Preview (aggregated per token) → **Save**.  
- Saved at `%LOCALAPPDATA%\HeatSignal\exports\{YYYY-MM-DD}\{addressShort}_{YYYYMMDD-HHMMSS}.ndjson`  
- File format and fields: see **EXPORT_SCHEMA.md**.  
- Multi-wallet export supported (combined NDJSON; one line per `(wallet, mint)`).

> Export is **blocked** if the selected lookback has **< 5 buys**.

## Updates (local-only policy preserved)

- **Check for updates**: reads a signed `latest.json` from  
  `https://github.com/Dev0nSol/HeatSignal-desktop/releases/latest/download/latest.json`
- **Install from file**: verifies SHA-256 against the **signed** manifest (downgrades blocked).
- Update keys are **Ed25519**; rollover requires a **one-click Trust this key** confirmation.  
- See **UPDATES.md** for signing/verification details.

## What it does / doesn’t

- **Does**: Read-only on-chain via Helius; market data via Birdeye (fallback DexScreener if Birdeye missing).  
- **Does not**: Handle private keys, seed phrases, or any signing beyond Phantom’s connect message.  
- **Outbound allowlist** only: `api.helius.xyz`, `mainnet.helius-rpc.com`, `public-api.birdeye.so`, `api.dexscreener.com`, `github.com`, `objects.githubusercontent.com`.

## License

MIT — see `LICENSE`.
