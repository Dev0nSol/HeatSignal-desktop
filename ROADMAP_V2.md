# Roadmap — V2 (Connected mode & $HEAT)

## High-level

- **Optional connected mode** (opt-in) with a tiny public endpoint where users can **upload their aggregated NDJSON** for research & improved models.
- **$HEAT token utility** integrated with perks and rewards for contributors.

## Planned features

1. **Data upload (opt-in)**
   - Endpoint receives **NDJSON**; verifies a static **export key** or **SIWS** (Sign-In With Solana).
   - Storage-first (e.g., R2/S3) → batch ETL to analytics DB.
   - Terms & transparency around exactly what’s received and how it’s used.

2. **Better recs**
   - Model retraining from uploaded aggregates.
   - Expand similarity features (text embeddings, graph features, wallet clusters).

3. **User benefits via $HEAT**
   - **Rewards** (airdrop or points) for verified contributions.
   - **Access** tiers / boosts (e.g., broader candidate pool, faster refresh).
   - **Governance** on niche taxonomy, safety thresholds, and scoring weights.

4. **Pro UX**
   - Multiple recommendations at once; category filters.
   - Rich token pages, quick swap deeplinks, watchlists.
   - Cross-platform (macOS/Linux) and portable mode.

5. **Security & trust**
   - Add **OV/EV** code signing for Windows installers.
   - Optional **beta** channel.
   - Automatic update checks (user-controlled) if privacy policy allows.

6. **Community**
   - Public dashboards (aggregate stats) without exposing wallet identities.
   - Open taxonomy discussions; curated niche mapping repo.

> Until V2 ships, HeatSignal V1 remains **local-only** and self-contained.
