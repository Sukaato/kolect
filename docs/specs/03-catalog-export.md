# 03 — Catalog export and distribution

Test scenarios: [tests/catalog-export.md](../tests/catalog-export.md).

## Principle

A script reads Postgres and produces two static files:

| File | Size | Role |
| --- | --- | --- |
| `manifest.json` | ~300 bytes | States the current version of the catalog (its hash) |
| `catalog.<hash>.json` | The whole catalog | The snapshot, immutable, named after its content |

The app reads the manifest; it downloads the snapshot only if the hash differs from the one it already applied.

## Published tree

```
/catalog/
└── v1/                                  # schema_version of the snapshot
    ├── manifest.json
    ├── catalog.3f9a1c0b7e2d4a51.json    # current snapshot
    └── catalog.a81e77c2d0934f6b.json    # previous snapshot (kept)
```

- The `v1/` prefix is the **format** version of the snapshot. An app only reads the folder of the format it understands.
- The last two snapshots are kept: an app that read the old manifest just before a publication can still download its file.

## Manifest

```json
{
  "schema_version": 1,
  "hash": "sha256:3f9a1c0b7e2d4a51c6e8…",
  "snapshot": "catalog.3f9a1c0b7e2d4a51.json",
  "size": 67572,
  "generated_at": "2026-11-12T03:00:14Z",
  "counts": { "acts": 12, "releases": 97, "items": 240, "photocards": 412 }
}
```

| Field | Description |
| --- | --- |
| `schema_version` | Format version of the snapshot. Equal to the folder number |
| `hash` | SHA-256 of the **bytes** of the snapshot file, prefixed with the algorithm |
| `snapshot` | File name, relative to the manifest. Contains the first 16 hex characters of the hash |
| `size` | Size in bytes, for verification and progress display |
| `generated_at` | Generation date, informative only |
| `counts` | Row count per table, informative (logs, settings screen) |

Optional fields:

| Field | Description |
| --- | --- |
| `relocated_to` | New base URL of the catalog. Absent in normal times; used to move the catalog to another address (domain or host change) without publishing a new app version, see [05](05-infrastructure-vps.md#domain-name) |
| `images_base_url` | Base URL that image keys are resolved against, see [decisions/0010](../decisions/0010-relative-image-keys.md) |

`generated_at` and `counts` are in the manifest and **not** in the snapshot: the snapshot contains only data, so its hash depends on nothing else.

## Snapshot

One array per catalog table at the root, plus `schema_version`. The content is **deterministic**.

Only published data is exported: rows with `review_status = 'draft'` are left out, and so are the provenance tables (`sources`, `external_refs`).

### Determinism rules

Two exports of an identical database must produce byte-identical files; otherwise the hash changes for no reason and every app re-downloads the catalog.

1. Tables in a fixed order (the foreign-key-safe order used by the seeder).
2. Rows sorted by primary key.
3. Object keys in a fixed order, defined by the script and not by the database.
4. Array columns serialized in their stored order (the order is data; it is not re-sorted).
5. Minified JSON, UTF-8, no BOM, explicit `null`s, no trailing newline.
6. No volatile data (`created_at`, `updated_at`, export date).

An automated test exports the same database twice and compares the hashes.

### Validation

Before any publication, the snapshot is validated against the JSON schema and the Vitest tests. An invalid snapshot **is not published** and the job fails; apps stay on the last valid snapshot.

Extra safeguard: if the row count of a table drops by more than 20 % compared with the published manifest, the job fails and requires a manual run with `--force`. This prevents an accidental deletion from reaching every app.

## Compatibility and `schema_version`

| Change to the snapshot | Action |
| --- | --- |
| Adding an **optional** column or table | Compatible. Same `schema_version`. Existing apps ignore unknown fields |
| Renaming, removing, changing a type or a meaning | Incompatible. Bump `schema_version`, publish under `v2/` |

When moving to `v2`, the export job keeps publishing `v1/` as long as app versions that read it are in circulation. The app's seeder must therefore **tolerate unknown fields** (no `deny_unknown_fields` on the serde side).

## Hosting

Files are served by **nginx on the VPS**, from a directory shared with the export job.

| Topic | nginx setting |
| --- | --- |
| HTTPS | Mandatory; the app refuses `http` in production |
| Compression | gzip (or brotli) enabled for `application/json` |
| Conditional requests | `ETag` enabled: an unchanged manifest answers `304` with no body |
| `manifest.json` | `Cache-Control: no-cache`: always revalidated, never served stale |
| `catalog.<hash>.json` | `Cache-Control: public, max-age=31536000, immutable`: the name changes with the content |
| Rate limiting | Per IP address on `/catalog/`, to protect the VPS bandwidth |

Base URL: `https://<domain>/catalog/` (see [05](05-infrastructure-vps.md#domain-name)).

Since snapshots are immutable and the manifest is tiny, a free CDN can be put in front of nginx later without changing the contract.

## Export job

An `exporter` container on the VPS, started by a timer.

| Trigger | Use |
| --- | --- |
| Timer: every 15 minutes | Automatic publication of changes |
| Manual: `docker compose run --rm exporter` (`--force` option) | Immediate publication after an edit, or to override the safeguard |
| End of a deployment | After migrations were applied |

The database being on the same machine, an export that finds no change costs a read of a few milliseconds and publishes nothing.

Steps:

1. Export from Postgres with the `catalog_exporter` role (read-only).
2. Validate the snapshot.
3. Compute the hash and compare it with the published `manifest.json`.
4. **Same hash: stop**, nothing is written.
5. Different hash: write the snapshot, then the manifest; delete snapshots beyond the two most recent.

Each file is written under a temporary name then renamed (atomic operation): nginx never serves a half-written file. The manifest is replaced **after** the snapshot, so it never points to a missing file.

A failure (validation, safeguard) is logged and must trigger an alert (see [05](05-infrastructure-vps.md#monitoring)).

## CI

### `catalog-check.yml` — check pull requests

On pull requests touching `catalog/**`: starts a Postgres service, applies all migrations, imports the seed, exports, validates, and checks determinism. Replaces `dataset-tests.yml`.

### `deploy.yml` — deploy to the VPS

Described in [05](05-infrastructure-vps.md#deployment): backup, migrations, service restart, export.

## Snapshot bundled in the app

At release build time, CI downloads the current snapshot and its manifest from the public URL and bundles them as Tauri resources (in place of `data/dataset.json`). At first launch the app seeds itself from this snapshot **without network** (see [04-app-sync.md](04-app-sync.md#first-launch)).
