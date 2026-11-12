# 0009 — One row per owned copy

- **Status**: Accepted
- **Date**: 2026-11-12

## Context

The collection already stores one row per copy, but does nothing with it: the `notes` column is unused and the signature is assigned to "the first N rows", unrelated to a specific copy. The interface only handles two numbers (owned, signed). Manual entry and a barcode scanner are planned.

## Decision

Keep one row per copy in `user_copies`, and give it meaning: acquisition date, condition, price, signature and note specific to each copy. Identifiers are UUID v7 for a stable creation order.

## Consequences

- The signature is attached to a specific copy.
- The interface must evolve to edit a copy, not only counters.
- Counts are computed with `COUNT`, not read from a column.

## Rejected alternatives

| Alternative | Why rejected |
| --- | --- |
| **One row per object with `quantity` and `signed_quantity`** | Simpler and faithful to the current interface, but forbids any per-copy detail |
| **Wishlist right now** | Not requested; can be added later without touching `user_copies` |
