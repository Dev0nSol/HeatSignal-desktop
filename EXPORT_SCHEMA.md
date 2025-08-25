# Export Schema — NDJSON (V1)

**Encoding**: UTF-8  
**Currency**: **USD** in file (strings with two decimals)  
**Timestamps**: Unix epoch seconds (UTC)

## File layout

1. **First line (meta)**:
```json
{"type":"meta","app_version":"0.1.0","export_ts":1692278400,"lookback_days":7,"terms_version":"v1"}
```

2. **Then one line per (wallet, mint) aggregate:**
```json
{
  "wallet_address": "Base58",
  "lookback_days": 7,
  "mint": "Base58",
  "symbol": "TKN",
  "niche": "DeFi",
  "buys_count": 3,
  "total_source_usd": "1234.56",
  "avg_source_usd": "411.52",
  "median_source_usd": "350.00",
  "first_buy_ts": 1723776000,
  "last_buy_ts": 1723948800,
  "niche_overridden": true
}
```

### Field order & types (locked)

1) `wallet_address` (string)  
2) `lookback_days` (int)  
3) `mint` (string)  
4) `symbol` (string; empty if unresolved)  
5) `niche` (string; user override if set)  
6) `buys_count` (int)  
7) `total_source_usd` (string, 2 decimals)  
8) `avg_source_usd` (string, 2 decimals)  
9) `median_source_usd` (string, 2 decimals)  
10) `first_buy_ts` (int)  
11) `last_buy_ts` (int)  
12) `niche_overridden` (boolean)

## Rules

- **Aggregated** over the active lookback (**rolling window**).  
- **NFTs** and **LP tokens** are **excluded** entirely.  
- **Stablecoins & SOL wrappers** are included in export aggregates (but never recommended).  
- **Owned** means current balance > **$5** USD (stables/wrappers ignored).  
- **Multi-wallet combined export**: one NDJSON with **one line per (wallet, mint)**.  
- If there are **zero** aggregate lines, export is **blocked** with guidance to widen lookback.
