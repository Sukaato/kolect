# 02 — Catalog database (Postgres)

## Scope

The Postgres database holds **the catalog only**. Its tables are defined in [06-database-schema.md](06-database-schema.md); this document covers everything around them: type mapping with the app, repo layout, migrations, environments and roles.

User tables (`user_copies`, `user_favorites`) exist **only** in the app's SQLite database. They will join Postgres with the API, in a separate schema.

Catalog tables live in a dedicated Postgres schema, `catalog`, so the API can later be given distinct rights on the catalog (read) and on user data (write).

## Relationship with the app's SQLite database

The app's SQLite database mirrors the catalog tables. The export converts Postgres types to what the app's seeder expects.

### Type mapping

| Postgres (catalog) | SQLite (app) | Serialization in the snapshot |
| --- | --- | --- |
| `uuid` | `TEXT` | lowercase string |
| `date` | `TEXT` | `"YYYY-MM-DD"` |
| `boolean` | `INTEGER` 0/1 | `true` / `false` |
| `text` + `CHECK` | `TEXT` + `CHECK` | string |
| `text[]` | `TEXT` holding a JSON array | array of strings, stored order preserved |
| `char(2)` | `TEXT` | string |
| `timestamptz` | not mirrored | not exported |

### Soft delete

`is_deleted` **does not exist** in Postgres. A row deleted in Postgres disappears from the snapshot, and the app's seeder flags it `is_deleted = 1` locally, so the collection of a user who owns the item does not break. Every mirrored catalog table in SQLite therefore has an extra `is_deleted` column.

### Audit columns

Every Postgres catalog table has:

| Column | Type | Use |
| --- | --- | --- |
| `created_at` | `timestamptz NOT NULL DEFAULT now()` | Audit |
| `updated_at` | `timestamptz NOT NULL DEFAULT now()` | Audit; basis for a future incremental sync. Maintained by a trigger |

These columns are **not** exported in the snapshot (they would make the hash unstable for no useful change).

## Repo layout

```
catalog/
├── migrations/               # Versioned SQL, applied locally and on the VPS
│   └── 0001_init.sql
├── seed/
│   └── dataset.json          # Former public/dataset.json, used as initial seed
└── scripts/
    ├── migrate.ts            # Applies pending migrations
    ├── import.ts             # dataset.json → Postgres (initial seed / local reset)
    └── export.ts             # Postgres → snapshot + manifest (see 03)
infra/
├── compose.yml               # Shared services (Postgres, nginx, exporter)
├── compose.dev.yml           # Local overrides
└── compose.prod.yml          # VPS overrides (see 05)
```

Scripts are written in TypeScript and run by Bun (already in the project), using the built-in Postgres client `Bun.sql`: no extra dependency.

| Command | Effect |
| --- | --- |
| `bun catalog:up` | Starts the local Docker Postgres |
| `bun catalog:migrate` | Applies migrations on `DATABASE_URL` |
| `bun catalog:import` | Loads `catalog/seed/dataset.json` (upsert) |
| `bun catalog:export` | Generates snapshot + manifest in the output directory |
| `bun catalog:reset` | Recreates the local database: drop + migrate + import |

Scripts only know a `DATABASE_URL` and an output directory: they run identically locally, in CI and on the VPS.

### Migrations

- Numbered SQL files, applied in order, recorded in a `schema_migrations` table.
- **Forward only**: no `down.sql`. A fix is a new migration.
- Any migration that changes the shape of the snapshot must follow the compatibility rule described in [03-catalog-export.md](03-catalog-export.md#compatibility-and-schema_version).
- On the VPS, migrations are applied by the deployment (see [05](05-infrastructure-vps.md#deployment)), after a backup.

## Environments

| Environment | Database | Access | Use |
| --- | --- | --- | --- |
| **local** | Docker Postgres | `postgres://kolect:kolect@localhost:5432/kolect` (in `.env.local`, git-ignored) | Daily development, tests |
| **CI (PR)** | Runner's Postgres service | same, ephemeral | Check migrations + import + export |
| **prod** | Docker Postgres on the VPS | Internal Docker network only; SSH tunnel for the maintainer | Source of truth |

Same Postgres image (same major version) in all three environments.

### Roles (prod)

| Role | Rights | Used by |
| --- | --- | --- |
| `catalog_admin` | Owner of the `catalog` schema | Maintainer, migrations |
| `catalog_exporter` | `SELECT` on `catalog` | Export job |

Passwords live in a `.env` file on the VPS, outside the repo. No connection string in the repo, in the app or in its config files. The Postgres port is never published on the Internet.

## Editing the catalog

At this stage the catalog is edited directly in SQL or with a GUI client (DBeaver, TablePlus…), connected as `catalog_admin` through an SSH tunnel:

```bash
ssh -L 5433:localhost:5432 kolect-vps
```

A consequence to be aware of: data changes **no longer go through a pull request**. The safeguards become the export job, which refuses to publish an invalid snapshot (see [03](03-catalog-export.md#validation)), and the daily backups in case of an entry mistake (see [05](05-infrastructure-vps.md#backups)).

## Initial seed and cutover

1. Move `public/dataset.json` to `catalog/seed/dataset.json`, converted to the new schema.
2. Run `bun catalog:import` on the local Postgres, then check that `bun catalog:export` reproduces equivalent content (same rows, same values).
3. On the VPS: apply migrations, import the seed.
4. From then on the VPS Postgres is authoritative and `catalog/seed/dataset.json` only serves to initialize local databases. It can be regenerated from production to stay representative.
