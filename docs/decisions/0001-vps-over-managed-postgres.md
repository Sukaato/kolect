# 0001 — VPS from the start rather than managed Postgres

- **Status**: Accepted
- **Date**: 2026-11-12

## Context

The catalog must leave the `dataset.json` file for a real database. Neon (managed Postgres with a free tier) was considered as a host, at least as a transition. The long-term target is a VPS holding the API, the database, a cache and nginx.

## Decision

Host Postgres directly on a VPS (probably Hostinger), without a Neon step.

## Consequences

- A fixed cost as soon as the catalog goes to production, instead of zero.
- A single installation, no migration to plan.
- Backups, security, certificates and monitoring are ours from day one (see [specs/05](../specs/05-infrastructure-vps.md)).
- Phases 1 to 3 of the [roadmap](../roadmap.md) happen locally: the VPS is only required in phase 4.

## Rejected alternatives

| Alternative | Why rejected |
| --- | --- |
| **Neon as a transition step** | Usage-based billing that can climb; installation work done twice; the final architecture is the VPS anyway |
| **Neon as the final target, with its Data API** | Dependency on proprietary features; unpredictable cost with users |
| **Staying on `dataset.json` in the repo** | No referential integrity, hand-editing does not hold at the targeted scale |
