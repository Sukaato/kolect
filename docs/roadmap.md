# Roadmap

## Phases

Each phase can ship on its own and leaves the app in a working state. Phases 1 to 3 happen entirely locally, without a VPS: the server is only needed from phase 4.

### Phase 1 — Local catalog database

- [ ] `infra/compose.yml` + `compose.dev.yml` (Postgres)
- [ ] Migration `0001_init.sql`: `catalog` schema as described in [specs/06](specs/06-database-schema.md), constraints, `updated_at` trigger
- [ ] `catalog:migrate`, `catalog:import`, `catalog:reset`
- [ ] Move `public/dataset.json` to `catalog/seed/dataset.json`, converted to the new schema
- [ ] Fix in the current app, independently of the redesign: enable SQLite foreign keys (`PRAGMA foreign_keys = ON` when the connection is opened; today every `ON DELETE` clause is inactive) and add `NOT NULL` to the favorites primary keys
- [ ] Fix stale paths in `package.json`, `diesel.toml` and `dataset-tests.yml` (`src/infrastructure/db/…` → `src/db/…`)

**Done when**: `bun catalog:reset` produces a local database holding the whole current dataset, with no constraint violation.

### Phase 2 — Export and hash

- [ ] Deterministic `catalog:export` (snapshot + manifest, atomic writes)
- [ ] Update the JSON schema and the Vitest tests to the new format
- [ ] Determinism test (two exports = same hash)
- [ ] `catalog-check.yml` workflow on pull requests

**Done when**: the local export produces a valid snapshot whose content matches the current dataset.

### Phase 3 — Hash-based sync and new schema in the app

- [ ] SQLite migrations for the new schema and `catalog_meta`
- [ ] Repositories, services, commands and screens moved to acts / releases / items
- [ ] New `sync` algorithm (manifest, ETag, SHA-256 check, 24 h throttle)
- [ ] Bundled snapshot and first-launch seeding
- [ ] Non-blocking startup, sync states in the UI

**Done when**: in dev, the app syncs against the catalog generated from Docker, starts without network, and downloads nothing when the hash has not changed.

### Phase 4 — VPS and production

- [ ] Order the VPS, baseline hardening (see [specs/05](specs/05-infrastructure-vps.md#security))
- [ ] Domain name and TLS certificate
- [ ] `compose.prod.yml`: Postgres, nginx, exporter
- [ ] Daily off-site backups, **with a restore test**
- [ ] `deploy.yml` workflow
- [ ] Import the seed, first published export
- [ ] Point `prod.json` to the VPS; bundle the snapshot at release build time
- [ ] Remove `dataset-tests.yml`

**Done when**: a change made in the VPS Postgres reaches a device within fifteen minutes without a new app version, and a backup has been restored successfully on an empty database.

The old `public/dataset.json` must stay online on `main` as long as app versions that read it are installed.

### Phase 5 — API and user data

Not specified in detail. Adds on the same VPS the API (behind nginx), the Postgres schema for user data, then a cache if a measurement justifies it.

## Future work

Listed to check that the chosen architecture closes no door.

| Evolution | What it would take | What is already in place |
| --- | --- | --- |
| **Accounts and collection sync** | API + auth + user tables in Postgres | UUIDs are already generated client-side; the catalog shares its identifiers between Postgres and SQLite; isolated `catalog` schema |
| **Cache** (Redis / Valkey) | One more Compose service | Nothing to plan: the catalog does not need it, nginx serves files |
| **Incremental sync** (if the snapshot grows large) | Publish diff files between two hashes | `updated_at` on every table; extensible manifest |
| **Catalog imports** | Import scripts per source | `sources`, `external_refs`, `review_status` (see [decisions/0011](decisions/0011-catalog-data-sourcing.md)) |
| **User contributions** | Proposal and review flow in the API | `review_status` separates drafts from published data |
| **Admin interface** | A web app behind the API | Constraints live in the database, not in the entry tool |
| **Images** | See [decisions/0010](decisions/0010-relative-image-keys.md) | Relative image keys |
| **CDN in front of nginx** | A free CDN as a front | Immutable snapshots, manifest revalidated with `ETag` |

## Open questions

| # | Question | Default proposal |
| --- | --- | --- |
| Q1 | Postgres migration tool: home-made Bun script or a dedicated tool (dbmate, Drizzle Kit)? | Home-made Bun script: nothing to install, very simple need |
| Q2 | Export job frequency | Every 15 minutes, plus manual runs |
| Q3 | Minimum delay between two manifest checks by the app | 24 h |
| Q4 | Domain name: buy one, or use the hostname provided by the host? | Buy one before the first public release (see [specs/05](specs/05-infrastructure-vps.md#domain-name)) |
| Q5 | Threshold of the "row count drop" safeguard | 20 % per table |
| Q6 | Off-site backup destination | Object storage with a free tier, encrypted dump |
| Q7 | Postgres major version | Latest stable at the time of phase 1, identical everywhere |
| Q8 | VPS size | Smallest plan: Postgres + nginx fit in 1 to 2 GB of RAM; to revisit with the API |
| Q9 | API stack (phase 5) | Rust (Axum), consistent with the Tauri backend and Diesel — to confirm |
| Q10 | Authentication (phase 5) | To study: email magic link, OAuth, or anonymous device-bound account |
| Q11 | Migrate the existing SQLite data or start from scratch? | Start from scratch if no version was distributed; otherwise migrate, keeping identifiers |

Schema-specific open questions are listed in [specs/06](specs/06-database-schema.md#open-questions).
