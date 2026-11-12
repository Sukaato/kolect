# 0004 — Local user data, API later

- **Status**: Accepted
- **Date**: 2026-11-12

## Context

The collection and favorites live in the device's SQLite database. Syncing them across devices requires accounts, authentication and an API, none of which exist yet.

## Decision

At this stage only the catalog is hosted. Collection and favorites stay local. The API, accounts and cache will be added on the same VPS in a later phase.

## Consequences

- No personal data on the server for now.
- Changing phone loses the collection: a known limit, to be addressed with the API or a manual export.
- The schema prepares what comes next: identifiers generated client-side, catalog isolated in its own Postgres schema.

## Rejected alternatives

| Alternative | Why rejected |
| --- | --- |
| **Accounts and sync right now** | Delays the catalog redesign; adds auth, API and personal-data compliance all at once |
