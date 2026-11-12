# 0011 — Catalog fed by imports and review, not by hand

- **Status**: Proposed
- **Date**: 2026-11-12

## Context

The target is to track everything: albums and their editions, lightsticks, kits, merch, and every photocard including store benefits and event cards. Entering all of it by hand is not realistic for one maintainer.

No open, documented photocard API was found. Releases and their editions, on the other hand, are well covered by open music databases. None of the sources below has been tested, and their terms of use have not been read yet.

| Data | Candidate source | Expected quality | Caveat |
| --- | --- | --- | --- |
| Labels, groups, members, releases, editions, barcodes | **MusicBrainz** (open API) | Good on releases, uneven on K-pop version details | Rate limit and client identification to respect; terms to read |
| Covers | **Cover Art Archive** (linked to MusicBrainz) | Variable | Images remain protected by copyright |
| Physical edition details | **Discogs** (API with an account) | Good on pressings | Data terms of use to check |
| Photocards | Community databases: K-Collect, Bibliocards, Pocamarket | Very complete (K-Collect advertises over 100,000 cards) | **No public API.** Bibliocards publicly complains about its database being copied; Pocamarket is only reachable through an unofficial endpoint |
| Groups and idols | Hallyu API (open project) | Advertised coverage stops in 2020 | Old data |
| Lightsticks, kits, merch | No structured source identified | — | Manual entry or contributions |

## Decision

1. **Releases and editions are imported automatically** from MusicBrainz, as `draft`, with `external_refs` so an import can be re-run without duplicates. Manual work shrinks to reviewing and validating.
2. **Photocards start with sets, not cards.** Creating a set with its expected card count costs one row; visuals come later.
3. **User contributions** are the long-term path, once the API exists: a user proposes a card or a photo, a maintainer validates.
4. **Community databases are approached for a partnership**, never scraped.

Only verified data is exported to the apps.

## Consequences

- The schema carries provenance: `sources`, `external_refs`, and `review_status` on `items` and `photocard_sets` (see [specs/06](../specs/06-database-schema.md)).
- An import script per source has to be written and maintained.
- A review step sits between import and publication: the catalog grows at the pace of the review.
- Photocard coverage will stay partial until contributions exist.
- Lightsticks, kits and merch remain manual for now.

## Rejected alternatives

| Alternative | Why rejected |
| --- | --- |
| **Enter everything by hand** | Does not scale to the targeted scope |
| **Scrape a community photocard database** | Legal risk, risk of being blocked, and a bad way to enter a community the project will need |
| **Publish imported data without review** | Uneven source quality on K-pop versions would reach every app |
| **Wait for a photocard source before modeling photocards** | Sets with an expected count are already useful without visuals |

## Next steps

- Read the terms of use of MusicBrainz and Discogs.
- Test importing a whole group from MusicBrainz and measure what is missing (versions, formats, barcodes).
- Decide whether user contributions are in the scope of the API.

## Sources consulted

- [MusicBrainz API](https://musicbrainz.org/doc/MusicBrainz_API)
- [Cover Art Archive API](https://wiki.musicbrainz.org/Cover_Art_Archive/API)
- [K-Collect](https://kcollect.net)
- [Bibliocards](https://apps.apple.com/app/bibliocards/id6445831348)
- [Pocamarket — third-party scraper on Apify](https://apify.com/kdatafactory/pocamarket-scraper.md)
- [Hallyu API](https://gh.r-mj.com/elletownsend/hallyuapi)
