# Decisions

One record per structuring decision. A record is never rewritten: if we change our mind, we create a new one and mark the old one "Superseded by …".

| Status | Meaning |
| --- | --- |
| **Accepted** | Settled, we build on it |
| **Proposed** | Recommendation awaiting validation |
| **Superseded** | Dropped in favour of another record |

## List

| # | Decision | Status |
| --- | --- | --- |
| [0001](0001-vps-over-managed-postgres.md) | VPS from the start rather than managed Postgres | Accepted |
| [0002](0002-static-snapshot-and-hash.md) | Catalog distributed as a static snapshot, versioned by hash | Accepted |
| [0003](0003-sqlite-offline-first.md) | Embedded SQLite, offline-first app | Accepted |
| [0004](0004-local-user-data.md) | Local user data, API later | Accepted |
| [0005](0005-docker-compose-everywhere.md) | Docker Compose locally and in production | Accepted |
| [0006](0006-unified-acts.md) | Groups, units and soloists in a single `acts` table | Proposed |
| [0007](0007-single-items-table.md) | One `items` table for every physical object | Proposed |
| [0008](0008-photocard-sets.md) | Photocards organized in sets | Proposed |
| [0009](0009-one-row-per-copy.md) | One row per owned copy | Accepted |
| [0010](0010-relative-image-keys.md) | Images referenced by relative key, served from the VPS | Proposed |
| [0011](0011-catalog-data-sourcing.md) | Catalog fed by imports and review, not by hand | Proposed |

## Template

```markdown
# NNNN — Title

- **Status**: Proposed | Accepted | Superseded by NNNN
- **Date**: YYYY-MM-DD

## Context
## Decision
## Consequences
## Rejected alternatives
```
