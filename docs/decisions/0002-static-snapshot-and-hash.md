# 0002 — Catalog distributed as a static snapshot, versioned by hash

- **Status**: Accepted
- **Date**: 2026-11-12

## Context

The app must receive catalog updates. Today it downloads the whole file at every startup and compares a semver version bumped by hand. Database credentials cannot be shipped in an APK, and we want to limit calls to the server, including to the future API.

## Decision

A job exports the catalog as an immutable JSON file, together with a `manifest.json` holding its SHA-256 hash. The app reads the manifest (a few bytes, at most once every 24 h) and downloads the snapshot only if the hash has changed. This distribution mode is kept even once the API exists.

## Consequences

- The export must be **deterministic**, otherwise the hash changes for no reason.
- The catalog is served by nginx without touching the database: a Postgres or API outage does not affect the apps.
- No more version to bump by hand.
- A catalog change is only visible after the next export (15 minutes at most).
- The snapshot is downloaded in full at every change; acceptable while it stays small, an incremental sync is possible later.

## Rejected alternatives

| Alternative | Why rejected |
| --- | --- |
| **The app queries an API on every screen** | No offline mode; server load proportional to usage |
| **The app connects directly to Postgres** | Credentials extractable from the APK; database exposed to the Internet |
| **Manual semver version** (current) | Forgetting it blocks every update |
| **Incremental sync right now** | Unjustified complexity for a catalog under 1 MB |
