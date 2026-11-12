# Test scenarios — Database schema

Spec: [specs/06-database-schema.md](../specs/06-database-schema.md).

Two families: **constraints** (the database rejects inconsistent data) and **real cases** (an existing situation can be entered without a workaround). Real cases also serve as the reference data set for the other tests.

## Constraints

| # | Given | When | Then |
| --- | --- | --- | --- |
| DB-01 | An act of kind `unit` | it is inserted without `parent_act_id` | the insert is rejected |
| DB-02 | An act of kind `group` | it is given a `parent_act_id` | rejected |
| DB-03 | An act of kind `soloist` | it is inserted without `person_id` | rejected |
| DB-04 | A person who already has a soloist act | a second one is created | rejected |
| DB-05 | An item of category `album` | it is inserted without `release_id` | rejected |
| DB-06 | An item of category `lightstick` | it is given a `format` | rejected |
| DB-07 | An item | `region` is `GLOBAL` (old value) | rejected: two letters expected |
| DB-08 | A set of kind `inclusion` | it is inserted without `item_id` | rejected |
| DB-09 | A set | `expected_count` is 0 | rejected |
| DB-10 | Two photocards of the same set | they have the same `position` | the second is rejected |
| DB-11 | A current member of an act | they are added again with no leave date | rejected |
| DB-12 | A member with a leave date | they are added again (back in the group) | accepted |
| DB-13 | An act that has releases | it is deleted | rejected |
| DB-14 | A user copy | both `item_id` and `photocard_id` are set | rejected |
| DB-15 | A user copy | neither is set | rejected |
| DB-16 | The app's SQLite database | an act referenced by a favorite is deleted | rejected: foreign keys are enforced on the connection |

## Real cases

Each case must be enterable and readable as described, without bending a column's meaning.

| # | Case | Check |
| --- | --- | --- |
| DB-20 | HYBE, BigHit Music, BELIFT LAB | Listing acts "under HYBE" returns BTS and ILLIT |
| DB-21 | Lisa: member of BLACKPINK and soloist | A single `people` row; two acts involve her |
| DB-22 | *LALISA* at YG, *Alter Ego* at LLOUD | Both releases have the same act and different labels |
| DB-23 | 3RACHA, unit of Stray Kids | The Stray Kids page can list its units; 3RACHA has three members |
| DB-24 | BORN PINK and its 9 editions | One release, 9 items; each of the 4 digipacks is linked to a member |
| DB-25 | Proof in 3 editions | One release of type `compilation`, 3 items |
| DB-26 | ARMY Bomb, 4 generations | 4 `lightstick` items attached to BTS, with no release |
| DB-27 | Version exclusive to a US store | Hidden when "exclusive items" is off |
| DB-28 | Japanese album of a Korean group | Separate release, `market` = `JP` |
| DB-29 | A store's pre-order benefit | Set linked to the release and the store, with no item |
| DB-30 | Lucky draw with unknown visuals | Set with `expected_count`, zero cards; the app shows the expected number |
| DB-31 | Cards from a pop-up store in Tokyo | `popup_store` event in `JP`; set linked to the event |
| DB-32 | Group card | Card with no member; counted for the act, absent from per-member filters |
| DB-33 | Mascot plushie | `merch` item, with no release and no format |

## Counts

| # | Given | When | Then |
| --- | --- | --- | --- |
| DB-40 | An act with 5 items and 20 photocards; the user owns 2 items and 3 cards | count without photocards | 2 / 5 |
| DB-41 | Same situation | count with photocards | 5 / 25 |
| DB-42 | The user owns 3 copies of the same item | count | the item counts as 1 owned |
| DB-43 | An item flagged as removed from the catalog, owned by the user | the collection is displayed | the item stays visible |
| DB-44 | An item in `draft` | catalog export | the item is absent from the snapshot |
| DB-45 | A set with `expected_count` = 8 and 3 cards entered | act total with photocards | counts 8, not 3 |
