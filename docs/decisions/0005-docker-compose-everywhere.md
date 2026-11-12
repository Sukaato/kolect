# 0005 — Docker Compose locally and in production

- **Status**: Accepted
- **Date**: 2026-11-12

## Context

We need a Postgres database to develop against, and a set of services on the VPS (Postgres, nginx, export, backup, later API and cache).

## Decision

Services are described in shared Docker Compose files: a common file, an override for development, one for production. The development Postgres is a local container.

## Consequences

- Same Postgres version and same service definitions everywhere.
- Development works offline and cannot touch production.
- Docker becomes a prerequisite for working on the catalog.

## Rejected alternatives

| Alternative | Why rejected |
| --- | --- |
| **Hosted development database** | Requires a connection; risk of confusion with production |
| **Direct installation on the VPS, no containers** | Rebuilding the server is slower and less reproducible |
