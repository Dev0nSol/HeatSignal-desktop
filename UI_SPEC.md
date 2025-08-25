# UI Spec — HeatSignal V1 (Windows, fixed 1280×800)

## Theme

- Dark only.  
- Primary accent: **#FF6A00** (“heat-map orange”).  
- Score gradient: **#FF3B30 → #FF6A00 → #FFD60A** (continuous).  
- Font: **Inter** (400 body / 500 buttons / 600 headings).  
- Colors:  
  - app bg `#1E1F24`, surface `#121316`, hover `#16181C`, borders `#2A2D32`  
  - text primary `#EDEFF2`, secondary `#A5A9B1`, muted `#6C7078`  
  - success dot `#22C55E`, error `#FF3B30`, warn `#FFD60A`

## Window & chrome

- Fixed size **1280×800**. Non-resizable.  
- Custom title bar: **HeatSignal** (buttons: **Minimize** + **Close** only).  
- Double-click title bar: **no-op**.  
- Close behavior: on first time, prompt **“Minimize to tray or Exit?”** (default Minimize). Remember choice.  
- Tray icon (if chosen): click restores; **no context menu**.

## Navigation (collapsible; default open, remember state)

1) **Dashboard**  
2) **Recommendations**  
3) **Export Data**  
4) **Data Transparency**  
5) **Updates**  
6) **Settings**  
7) **About & Social**

## Onboarding (skippable)

1. **Privacy & Local-Only** (includes *Not financial advice*).  
2. **Enter API Keys** (Helius + Birdeye; validate).  
3. **Set Defaults** (lookback 7d; thresholds $75k/$20k).  
4. **Connect Phantom** (optional here).

## Dashboard (pre-scan widgets)
- **Phantom status tile** (nickname + short addr + **Copy address** + Switch wallet).  
- **API keys tile** (green checks + Edit keys).  
- **Scan/Refresh tile** (big **Scan** button; later shows last scan time + buys count).  
- **V2 Coming Soon** banner → About & Social.  
- **Data Transparency** quick link.  
- If onboarding skipped: persistent **“API keys required”** banner.

## Scan/Refresh progress
- **Stepper**: 1) Fetch txs 2) Parse buys 3) Fetch market 4) Score 5) Rank  
- No Cancel (block closing with modal).  
- Partial data banner + **Retry** when providers hiccup.

## Recommendations
- **Top pick card**:
  - **Name + symbol**, **HEAT score** (0–100 + gradient bar; tooltip shows breakdown), **Safety badge** (“Verified ✓” / “Safety unknown ⚠️” clickable → modal).  
  - **Reason** (template): “Matches your **{niche}** buys; **7-day volume ↑ {pct}%**, **liquidity {tier}**, volatility **{low/med/high}**.” *(fallbacks approved)*  
  - Buttons: **View on Birdeye**, 3-dot menu: **View on Solscan**, **View on Jupiter**, **Copy mint**.
- **Next best** list (up to 2): condensed rows; click expands into full details.  
- If **<5 buys** in lookback: toast suggests **Increase lookback** (one-click).  
- If **no match**: show “No matches right now. Your thresholds and filters are too strict for current market data.” + **Widen search?** (temporary relax).

## Export Data
- Shows **current lookback** (always exports the current selection).  
- **All-or-nothing** export (no per-token deselect).  
- Table: **aggregated per token**; **USD only** in preview; **fixed default sort** by `total_source_usd` (desc).  
- Save = **auto-save to default folder**; toast: **Open folder** / **Copy path**.  
- Multi-wallet export: user selects wallets → one combined NDJSON (one line per `(wallet, mint)`).  
- Keeps a **history (20 files)**; **Clear history** available.

## Data Transparency
- Exactly the list you approved (local-only, stored, exported, not collected).

## Updates
- **Check for updates**: if new version → show version + **View Release Notes on GitHub**.  
- **Install from file**: restrict file dialog to `HeatSignal-*.exe`; verify SHA-256 vs signed manifest; **block downgrades**; final confirm modal; app closes → installer → relaunch.  
- If manifest fetch fails: validate against **cached ≤7 days**; otherwise allow **manual `latest.json`** load.  
- **Trusted update keys** manager: list + **revoke**.

## Settings
- **General**: currency toggle (USD/EUR; **blank FX until user sets**), timezone, 24-hour time; **Demo Mode** toggle (in Settings only); sidebar collapse toggle; Reset to defaults.  
- **API Keys**: masked fields + **Reveal** + **Test** (no DB re-prompt).  
- **Scan**: lookback presets, thresholds sliders (liq $25k–$2M; vol $10k–$1M), DEX whitelist fixed, market cap ≥ $1M fixed (hidden), DEX count ≥ 2 fixed. Changes **auto-refresh**.  
- **Wallets**: list, edit **nicknames** (also inline in picker), remove (prompt to delete data or just remove reference).  
- **Token Niches**: show seen tokens with auto/override; per-wallet overrides; **Reset all overrides** button.  
- **Security**: set/change DB passphrase; auto-lock 15 min; **Wipe Data** (type HEATSIGNAL + passphrase).  
- **Advanced**: outbound allowlist (fixed), hardware accel always on, no docs UI, no logs UI.

## Copy & legal

- Persistent footer (Recommendations): “**Not financial advice.** HeatSignal provides informational suggestions…”  
- About & Social: X links, GitHub link, **copy-on-click CA (hardcoded)**, and **version only**.
