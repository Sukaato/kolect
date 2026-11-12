# 0003 — Embedded SQLite, offline-first app

- **Status**: Accepted
- **Date**: 2026-11-12

## Context

The app is used on mobile, sometimes without network (in a store, at a concert). It already embeds a SQLite database holding the catalog and the collection. With a remote database, reading everything online was tempting.

## Decision

The app **always** reads its local SQLite database. The network is only used to update the catalog, in the background, without ever blocking navigation. A snapshot is bundled in the app for the first launch.

## Consequences

- The app works right after installation and without network.
- The catalog exists twice (Postgres and SQLite): two schemas to maintain side by side, and a type conversion at export time.
- The startup screen must no longer wait for the sync.

## Rejected alternatives

| Alternative | Why rejected |
| --- | --- |
| **Everything online, no local database** | Unusable offline; every screen depends on the server |
| **HTTP cache only, no SQLite** | No relational queries (counts, filters) offline |
