# Test scenarios — Infrastructure

Spec: [specs/05-infrastructure-vps.md](../specs/05-infrastructure-vps.md).

Most of these scenarios are checked by hand, once at setup time (phase 4) then after any infrastructure change.

## Network exposure

| # | When | Then |
| --- | --- | --- |
| IN-01 | We scan the VPS ports from outside | only 22, 80 and 443 answer |
| IN-02 | We try to reach Postgres from the Internet | connection refused |
| IN-03 | We call the catalog over `http` | redirected to `https` |
| IN-04 | We try an SSH login with a password | refused |
| IN-05 | We request a directory listing of `/catalog/` | refused |

## Serving the catalog

| # | When | Then |
| --- | --- | --- |
| IN-10 | We request the manifest with its current `ETag` | `304` answer with no body |
| IN-11 | We request the manifest | the header forbids serving it without revalidation |
| IN-12 | We request a snapshot | long `immutable` cache; compressed response |
| IN-13 | Postgres is stopped | manifest and snapshot are still served |
| IN-14 | A single address chains requests | rate limiting applies |

## Deployment

| # | Given | When | Then |
| --- | --- | --- | --- |
| IN-20 | Pending migrations | deployment | a backup is taken before they are applied |
| IN-21 | A failing migration | deployment | it stops; services in place and the last snapshot keep being served |
| IN-22 | A deployment with no change | deployment | no service is restarted |
| IN-23 | A successful deployment | — | the public manifest answers and its hash matches the snapshot |

## Backups

| # | Given | When | Then |
| --- | --- | --- | --- |
| IN-30 | The backup job | it runs | an encrypted dump appears off the VPS |
| IN-31 | A dump | we restore it on an empty database, then export | the hash equals the catalog hash at dump time |
| IN-32 | More than 7 daily dumps | rotation | the oldest are deleted according to the retention rule |
| IN-33 | No backup for 48 h | — | an alert is raised |

## Rebuild

| # | Given | When | Then |
| --- | --- | --- | --- |
| IN-40 | A fresh VPS, the repo, the `.env` and the last dump | we follow the rebuild procedure | the catalog is served again in under one hour |
| IN-41 | The VPS is down | a user opens the app | the app works on its local catalog |

## Monitoring

| # | When | Then |
| --- | --- | --- |
| IN-50 | The export fails | a notification is received |
| IN-51 | The manifest becomes unreachable | the external probe alerts |
| IN-52 | The disk goes above 80 % | an alert is raised |
