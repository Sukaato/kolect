# 0008 — Photocards organized in sets

- **Status**: Proposed
- **Date**: 2026-11-12

## Context

Today a photocard is attached to an album, a version or a digipack. Yet we want to track **every** card: those from lightsticks, kits and season's greetings, per-store pre-order benefits, lucky draws, pop-up stores per country, concerts. It also happens that the number of cards in a batch is known while their visuals are not.

## Decision

Cards are grouped into `photocard_sets`. A set describes **where the cards come from**: the object containing them, the store, the event, and its kind (`inclusion`, `preorder_benefit`, `lucky_draw`, `event`…). It carries an optional `expected_count`. `photocards` belong to a set, with a position, an optional member and an optional image.

## Consequences

- Any card origin can be represented without a new column.
- A set can exist before its cards: "8 cards, visuals coming".
- A card without an image can already be ticked by the user.
- Photocards stay outside `items`: the collection keeps two foreign keys (`item_id`, `photocard_id`).
- The entry volume is considerable: it depends on a data source (see [0011](0011-catalog-data-sourcing.md)).

## Rejected alternatives

| Alternative | Why rejected |
| --- | --- |
| **Direct link to album / version / digipack** (current) | Covers neither other objects nor benefits and events |
| **Origin columns directly on each card** | Repeats the same information on every card of a batch; cannot describe a batch whose cards are unknown |
