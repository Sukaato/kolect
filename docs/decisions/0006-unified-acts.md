# 0006 — Groups, units and soloists in a single `acts` table

- **Status**: Proposed
- **Date**: 2026-11-12

## Context

The current schema separates `groups` and `artists`. Albums, lightsticks, kits and favorites therefore carry two columns (`group_id`, `artist_id`) of which only one is set, a rule that exists only as a comment. Home lists merge two queries in memory. The `artists` table stands for both a person and a solo career, and sub-units (3RACHA within Stray Kids) cannot be represented.

## Decision

One `acts` table with a kind: `group`, `unit` or `soloist`. People are in `people`; a soloist is an act linked to a person. A unit references its group. Labels become hierarchical and are carried both by the act (current label) and by each release.

## Consequences

- A single `act_id` foreign key everywhere; favorites in one table.
- Lists can be paginated in SQL.
- Group and Artist screens can merge; so can the `group_*` and `artist_*` commands.
- A soloist takes two rows (`people` and `acts`).
- The `artist_aliases` table goes away.

## Rejected alternatives

| Alternative | Why rejected |
| --- | --- |
| **Keep `groups` and `artists`, add `CHECK`s** | Fixes integrity, not the duplicated queries nor sub-units |
| **Sub-units as plain unrelated groups** | Loses the relation with the parent group |
