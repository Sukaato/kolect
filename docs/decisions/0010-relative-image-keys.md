# 0010 — Images referenced by relative key, served from the VPS

- **Status**: Proposed
- **Date**: 2026-11-12

## Context

Every catalog row currently carries a full `image_url`, pointing to a placeholder domain: no image is actually hosted. `todo.md` lists the topic without settling it. The final host is not chosen, and it may change.

## Decision

Two parts: how images are referenced, and where they are hosted first.

**Reference.** The database stores a relative key (`image_key`, for example `releases/<uuid>.webp`). The base URL is given to the app by the catalog manifest (`images_base_url`). Files are immutable: a modified image gets a new key.

**Hosting.** Start with nginx on the VPS, keeping the ability to switch to object storage by changing the base URL only.

| Topic | Proposal |
| --- | --- |
| Sizes | Two variants generated at import time: thumbnail (~300 px) for grids, large (~1000 px) for detail screens |
| Format | WebP |
| Cache | Long cache on the server and in the app; no invalidation since files are immutable |
| In the app | A single image component, with a placeholder when the key is null or the image is unavailable |
| Offline | Images already displayed stay available through the WebView cache; no pre-download |

## Consequences

- Changing image host requires neither a data migration nor a new app version.
- The app must build the URL and handle a missing image.
- Images use the VPS disk and bandwidth, and must be included in backups.
- An import script has to generate the two variants.

## Rejected alternatives

| Alternative | Why rejected |
| --- | --- |
| **Full URL in the database** (current) | Ties the data to a host |
| **Images bundled in the app or stored in the repo** | App and repo size; cannot be updated without a release |
| **Object storage with a free tier** (e.g. Cloudflare R2) from the start | Outbound bandwidth not billed and built-in CDN, but one more service and account while the VPS is already paid for. Kept as the fallback if VPS bandwidth becomes a problem |
| **Managed image service** (on-the-fly resizing) | Paid beyond a small volume |

## Open points

- **Where images come from** and who imports them: see [0011](0011-catalog-data-sourcing.md).
- **Rights**: covers and photocards are protected visuals. Low risk for personal use, to be reassessed before publishing on a store.
- **Photos taken by users**: possible with the API, raises moderation questions.
- **Volume**: to be estimated. Order of magnitude: a few tens of KB per thumbnail; 100,000 photocards would amount to several GB.
