# Kolect documentation

Each folder has one job. Before writing, pick the folder by the question the document answers.

| Folder | Answers | Content | Lifetime |
| --- | --- | --- | --- |
| [overview/](overview/) | "How does it work today?" | Description of the existing app | Updated when the code changes |
| [specs/](specs/) | "What do we have to build?" | Expected behaviour, formats, schemas. No long justification | Updated until shipped, then becomes the reference |
| [decisions/](decisions/) | "Why this choice?" | One record per decision: context, choice, rejected alternatives. A choice that is not settled yet is a record with status `Proposed` | Never rewritten once accepted; a new record supersedes the old one |
| [tests/](tests/) | "How do we know it works?" | Test scenarios, per feature | Updated together with the matching spec |
| [roadmap.md](roadmap.md) | "In what order, and what is still undecided?" | Phases, future work, open questions | Living |

## Index

### Overview

- [The app today](overview/existing-app.md): features, frontend, backend, data model

### Specs

- [01 — Architecture](specs/01-architecture.md)
- [02 — Catalog database (Postgres)](specs/02-catalog-database.md)
- [03 — Catalog export and distribution](specs/03-catalog-export.md)
- [04 — App-side sync](specs/04-app-sync.md)
- [05 — VPS infrastructure](specs/05-infrastructure-vps.md)
- [06 — New database schema](specs/06-database-schema.md)

### Decisions

See [decisions/README.md](decisions/README.md) for the list and the status of each record.

### Test scenarios

- [Database schema](tests/database-schema.md)
- [Catalog export](tests/catalog-export.md)
- [App sync](tests/app-sync.md)
- [Infrastructure](tests/infrastructure.md)

## The project in five points

1. **A Postgres database on a VPS is the source of truth for the catalog.** The app never connects to it.
2. **An export job** produces a JSON snapshot and a `manifest.json` holding its SHA-256 hash, served as static files by nginx.
3. **The app stays offline-first**: it reads its local SQLite database and downloads the snapshot only when the hash has changed.
4. **User data stays local** for now. The API, accounts and cache will be added later on the same VPS.
5. **The schema is redesigned** around three concepts: *acts* (groups, units, soloists), *items* (any physical object) and photocard *sets*.

## Status

Documentation written, pending validation. Implementation not started.
