# Test scenarios — Catalog export

Spec: [specs/03-catalog-export.md](../specs/03-catalog-export.md).

## Determinism and hash

| # | Given | When | Then |
| --- | --- | --- | --- |
| EX-01 | An unchanged database | we export twice | both snapshots are byte-identical, same hash |
| EX-02 | The same data inserted in a different order | we export | same hash |
| EX-03 | A row modified then set back to its original value | we export | same hash as before (`updated_at` has no effect) |
| EX-04 | A modified value | we export | the hash changes |
| EX-05 | A produced snapshot | we compute the SHA-256 of the file | it equals the manifest `hash` |
| EX-06 | A produced snapshot | we read its file name | it contains the first 16 characters of the hash |

## Publication

| # | Given | When | Then |
| --- | --- | --- | --- |
| EX-10 | A hash identical to the published one | the export runs | no file is written or modified |
| EX-11 | A different hash | the export runs | the new snapshot exists **before** the manifest points to it |
| EX-12 | Three successive publications | we list the directory | only the two most recent snapshots remain |
| EX-13 | An export being written | a client reads the manifest | it gets the old or the new one, never a truncated file |
| EX-14 | No published manifest (first time) | the export runs | snapshot and manifest are created |

## Validation and safeguards

| # | Given | When | Then |
| --- | --- | --- | --- |
| EX-20 | A snapshot that violates the JSON schema | the export runs | nothing is published, the job fails, the old manifest stays in place |
| EX-21 | A table loses more than 20 % of its rows | the export runs without `--force` | nothing is published, the job fails |
| EX-22 | Same situation | the export runs with `--force` | the snapshot is published |
| EX-23 | A table loses 10 % of its rows | the export runs | published normally |
| EX-24 | Rows in `draft` | the export runs | they are absent from the snapshot |
| EX-25 | Populated `sources` and `external_refs` tables | the export runs | they are absent from the snapshot |

## Format

| # | Given | When | Then |
| --- | --- | --- | --- |
| EX-30 | A snapshot | we inspect it | no `created_at`, `updated_at`, nor export date |
| EX-31 | An array column (`roles`) | we export | the stored order is preserved |
| EX-32 | An empty optional field | we export | it is `null`, not omitted |
| EX-33 | The manifest | we inspect it | `schema_version` matches the `vN/` folder |

## Import

| # | Given | When | Then |
| --- | --- | --- | --- |
| EX-40 | An empty database and the seed file | `catalog:import` | every row is inserted with no constraint violation |
| EX-41 | An already imported database | `catalog:import` again | no duplicate, no error |
| EX-42 | An exported snapshot | we import it into an empty database then re-export | same hash (lossless round trip) |
