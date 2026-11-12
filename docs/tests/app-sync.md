# Test scenarios — App sync

Spec: [specs/04-app-sync.md](../specs/04-app-sync.md).

## First launch

| # | Given | When | Then |
| --- | --- | --- | --- |
| SY-01 | Freshly installed app, no network | it is launched | the home screen shows the bundled catalog |
| SY-02 | Freshly installed app, network available, newer remote catalog | it is launched | the home screen shows right away, then lists refresh |
| SY-03 | App updated from a version with `dataset_metadata.json` | it is launched | the file is deleted, a full sync happens, collection and favorites are intact |

## Manifest check

| # | Given | When | Then |
| --- | --- | --- | --- |
| SY-10 | Last check 2 h ago | the app is launched | no network request |
| SY-11 | Last check 25 h ago, unchanged catalog | the app is launched | one request, `304` answer, nothing downloaded |
| SY-12 | Last check 25 h ago, changed catalog | the app is launched | manifest then snapshot downloaded, catalog applied |
| SY-13 | Last check 2 h ago | manual sync | the manifest is checked despite the delay |
| SY-14 | Remote hash identical to the local hash | manual sync | the snapshot is not downloaded |
| SY-15 | Identical hash | sync with `reseed` | the snapshot is re-applied |

## Failures

| # | Given | When | Then |
| --- | --- | --- | --- |
| SY-20 | No network | automatic sync | `offline` state, no message, the app stays usable |
| SY-21 | No network | manual sync | error message |
| SY-22 | Network drops during the snapshot download | — | database unchanged, retried at the next check |
| SY-23 | Snapshot whose hash does not match the manifest | — | rejected before the JSON is read, database unchanged |
| SY-24 | Valid manifest, missing snapshot (404) | — | `error` state, database unchanged |
| SY-25 | App killed while the snapshot is being applied | it is relaunched | previous catalog intact, local hash unchanged |
| SY-26 | Remote `schema_version` higher than the app's | sync | `app_update_required` state, local catalog kept |
| SY-27 | Snapshot containing a field unknown to the app | sync | applied normally, field ignored |

## User data integrity

| # | Given | When | Then |
| --- | --- | --- | --- |
| SY-30 | An owned item is removed from the catalog | sync | it stays in the collection, flagged as deleted |
| SY-31 | A favorite act is removed from the catalog | sync | the favorite is not deleted |
| SY-32 | An item removed then reintroduced in the catalog | sync | it is no longer flagged as deleted; owned copies are still linked to it |
| SY-33 | A collection of several hundred copies | full sync | no copy lost or duplicated |

## Catalog relocation

| # | Given | When | Then |
| --- | --- | --- | --- |
| SY-40 | A manifest with an `https` `relocated_to` | sync | the new address is stored and used from then on |
| SY-41 | A manifest with an `http` `relocated_to` | sync | ignored |
| SY-42 | New address unreachable for less than 7 days | sync | the app does not fall back to the old address |
| SY-43 | New address unreachable for more than 7 days | sync | the app retries the address from its config |

## Interface

| # | Given | When | Then |
| --- | --- | --- | --- |
| SY-50 | A sync in progress | the user navigates | navigation is never blocked |
| SY-51 | A catalog updated in the background | — | notification, lists refreshed without losing filters |
| SY-52 | The Settings screen | — | shows the date of the last check and a short catalog identifier |
