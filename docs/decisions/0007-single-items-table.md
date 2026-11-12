# 0007 — One `items` table for every physical object

- **Status**: Proposed
- **Date**: 2026-11-12

## Context

The current schema has one table per object type: `album_versions`, `digipacks`, `lightsticks`, `fanclub_kits`. We want to be able to add season's greetings, photobooks and merch. Each new table adds a foreign key to the collection and a branch to every counting query. Besides, `album_versions.format` mixes the medium (`cd`, `vinyl`) and the release type (`ep`, `mini_album`).

## Decision

- `releases` represents the work, with its type (`full`, `mini`, `single`…).
- `items` represents any physical object, with a `category` and, for albums, a `format`.
- A digipack is an item with format `digipack`, optionally linked to a member.
- Store exclusives and event sales are carried by `vendor_id` and `event_id`.

## Consequences

- Adding an object category no longer requires a structural migration.
- A single foreign key to objects in the collection.
- Some columns only apply to one category; `CHECK` constraints hold consistency.
- Dataset, seeder, DTOs and screens are redesigned.

## Rejected alternatives

| Alternative | Why rejected |
| --- | --- |
| **One table per category** (current) | Does not scale to the targeted number of categories |
| **`items` table + one detail table per category** | More rigorous, but category-specific attributes are too few to justify it |
| **Photocards inside `items`** | Columns and volumes too different; see [0008](0008-photocard-sets.md) |
