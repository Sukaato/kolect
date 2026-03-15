// src/tests/dataset.test.ts
//
// Tests de cohérence du dataset v2.
// On charge le fichier et on vérifie que toutes les FK pointent vers des IDs existants,
// que les contraintes métier sont respectées, et que les patterns de base sont valides.
// Aucun ID hardcodé — les tests s'adaptent automatiquement si le dataset évolue.

import { describe, it, expect, beforeAll } from 'vitest'
import datasetRaw from '../../public/dataset.json' with { type: 'json '}

// ─── Types locaux ─────────────────────────────────────────────────────────────

interface Dataset {
  version: string
  agencies:       Agency[]
  groups:         Group[]
  artists:        Artist[]
  artist_aliases: ArtistAlias[]
  group_members:  GroupMember[]
  albums:         Album[]
  album_versions: AlbumVersion[]
  digipacks:      Digipack[]
  lightsticks:    Lightstick[]
  fanclub_kits:   FanclubKit[]
  photocards:     Photocard[]
}

interface Agency       { id: string; name: string; country: string; image_url: string | null }
interface Group        { id: string; name: string; debut_date: string; agency_id: string; fandom_name: string | null; image_url: string | null }
interface Artist       { id: string; real_name: string; birth_date: string | null; image_url: string | null; solo_debut_date: string | null; solo_agency_id: string | null }
interface ArtistAlias  { id: string; artist_id: string; name: string; kind: string; is_primary: boolean }
interface GroupMember  { artist_id: string; group_id: string; roles: string | null; join_date: string | null; leave_date: string | null }
interface Album        { id: string; name: string; release_date: string; region: string; image_url: string | null; group_id: string | null; artist_id: string | null }
interface AlbumVersion { id: string; album_id: string; name: string; format: string; release_date: string; region: string; image_url: string | null }
interface Digipack     { id: string; album_id: string; artist_id: string | null; name: string; release_date: string; region: string; image_url: string | null }
interface Lightstick   { id: string; name: string; version: string; release_date: string; image_url: string | null; group_id: string | null; artist_id: string | null }
interface FanclubKit   { id: string; name: string; release_date: string; region: string; image_url: string | null; group_id: string | null; artist_id: string | null }
interface Photocard    { id: string; name: string; artist_id: string | null; album_id: string | null; album_version_id: string | null; digipack_id: string | null; release_date: string; region: string; image_url: string | null }

// ─── Setup ────────────────────────────────────────────────────────────────────

const dataset = datasetRaw as Dataset

// Index sets — construits une seule fois pour tous les tests
let agencyIds:       Set<string>
let groupIds:        Set<string>
let artistIds:       Set<string>
let albumIds:        Set<string>
let albumVersionIds: Set<string>
let digipackIds:     Set<string>

const UUID_RE  = /^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/
const DATE_RE  = /^\d{4}-(0[1-9]|1[0-2])-(0[1-9]|[12]\d|3[01])$/
const URI_RE   = /^https?:\/\/.+/

beforeAll(() => {
  agencyIds       = new Set(dataset.agencies.map(a => a.id))
  groupIds        = new Set(dataset.groups.map(g => g.id))
  artistIds       = new Set(dataset.artists.map(a => a.id))
  albumIds        = new Set(dataset.albums.map(a => a.id))
  albumVersionIds = new Set(dataset.album_versions.map(v => v.id))
  digipackIds     = new Set(dataset.digipacks.map(d => d.id))
})

// ─── Structure du dataset ─────────────────────────────────────────────────────

describe('dataset structure', () => {
  it('has a valid semver version', () => {
    expect(dataset.version).toMatch(/^\d+\.\d+\.\d+$/)
  })

  it('has all required tables', () => {
    const tables = ['agencies','groups','artists','artist_aliases','group_members',
                    'albums','album_versions','digipacks','lightsticks','fanclub_kits','photocards']
    for (const t of tables) {
      expect(dataset).toHaveProperty(t)
      expect(Array.isArray((dataset as any)[t])).toBe(true)
    }
  })

  it('all tables are non-empty', () => {
    expect(dataset.agencies.length).toBeGreaterThan(0)
    expect(dataset.groups.length).toBeGreaterThan(0)
    expect(dataset.artists.length).toBeGreaterThan(0)
    expect(dataset.artist_aliases.length).toBeGreaterThan(0)
    expect(dataset.group_members.length).toBeGreaterThan(0)
    expect(dataset.albums.length).toBeGreaterThan(0)
    expect(dataset.album_versions.length).toBeGreaterThan(0)
    expect(dataset.photocards.length).toBeGreaterThan(0)
  })
})

// ─── Patterns UUID / date / URI ───────────────────────────────────────────────

describe('uuid patterns', () => {
  it('all agency ids are valid uuids', () => {
    for (const r of dataset.agencies)
      expect(r.id, `agency "${r.name}"`).toMatch(UUID_RE)
  })

  it('all group ids are valid uuids', () => {
    for (const r of dataset.groups)
      expect(r.id, `group "${r.name}"`).toMatch(UUID_RE)
  })

  it('all artist ids are valid uuids', () => {
    for (const r of dataset.artists)
      expect(r.id, `artist "${r.real_name}"`).toMatch(UUID_RE)
  })

  it('all album ids are valid uuids', () => {
    for (const r of dataset.albums)
      expect(r.id, `album "${r.name}"`).toMatch(UUID_RE)
  })

  it('all album_version ids are valid uuids', () => {
    for (const r of dataset.album_versions)
      expect(r.id, `album_version "${r.name}"`).toMatch(UUID_RE)
  })

  it('all digipack ids are valid uuids', () => {
    for (const r of dataset.digipacks)
      expect(r.id, `digipack "${r.name}"`).toMatch(UUID_RE)
  })

  it('all photocard ids are valid uuids', () => {
    for (const r of dataset.photocards)
      expect(r.id, `photocard "${r.name}"`).toMatch(UUID_RE)
  })
})

describe('date patterns', () => {
  it('all group debut_dates are valid dates', () => {
    for (const r of dataset.groups)
      expect(r.debut_date, `group "${r.name}"`).toMatch(DATE_RE)
  })

  it('all album release_dates are valid dates', () => {
    for (const r of dataset.albums)
      expect(r.release_date, `album "${r.name}"`).toMatch(DATE_RE)
  })

  it('all album_version release_dates are valid dates', () => {
    for (const r of dataset.album_versions)
      expect(r.release_date, `album_version "${r.name}"`).toMatch(DATE_RE)
  })

  it('artist birth_dates are valid when present', () => {
    for (const r of dataset.artists)
      if (r.birth_date)
        expect(r.birth_date, `artist "${r.real_name}"`).toMatch(DATE_RE)
  })

  it('artist solo_debut_dates are valid when present', () => {
    for (const r of dataset.artists)
      if (r.solo_debut_date)
        expect(r.solo_debut_date, `artist "${r.real_name}"`).toMatch(DATE_RE)
  })
})

describe('uri patterns', () => {
  it('image_urls are valid uris when present', () => {
    const allWithImage = [
      ...dataset.agencies,
      ...dataset.groups,
      ...dataset.albums,
      ...dataset.album_versions,
      ...dataset.digipacks,
      ...dataset.lightsticks,
      ...dataset.fanclub_kits,
      ...dataset.photocards,
    ] as Array<{ image_url: string | null; name?: string; real_name?: string }>

    for (const r of allWithImage)
      if (r.image_url)
        expect(r.image_url, `image_url on "${r.name ?? r.real_name}"`).toMatch(URI_RE)
  })
})

// ─── Unicité des IDs ──────────────────────────────────────────────────────────

describe('id uniqueness', () => {
  function assertUniqueIds(records: { id: string }[], label: string) {
    const seen = new Set<string>()
    for (const r of records) {
      expect(seen.has(r.id), `duplicate id ${r.id} in ${label}`).toBe(false)
      seen.add(r.id)
    }
  }

  it('agency ids are unique',        () => assertUniqueIds(dataset.agencies,       'agencies'))
  it('group ids are unique',         () => assertUniqueIds(dataset.groups,         'groups'))
  it('artist ids are unique',        () => assertUniqueIds(dataset.artists,        'artists'))
  it('artist_alias ids are unique',  () => assertUniqueIds(dataset.artist_aliases, 'artist_aliases'))
  it('album ids are unique',         () => assertUniqueIds(dataset.albums,         'albums'))
  it('album_version ids are unique', () => assertUniqueIds(dataset.album_versions, 'album_versions'))
  it('digipack ids are unique',      () => assertUniqueIds(dataset.digipacks,      'digipacks'))
  it('lightstick ids are unique',    () => assertUniqueIds(dataset.lightsticks,    'lightsticks'))
  it('fanclub_kit ids are unique',   () => assertUniqueIds(dataset.fanclub_kits,   'fanclub_kits'))
  it('photocard ids are unique',     () => assertUniqueIds(dataset.photocards,     'photocards'))
})

// ─── FK : agencies ────────────────────────────────────────────────────────────

describe('FK → agencies', () => {
  it('group.agency_id references a known agency', () => {
    for (const g of dataset.groups)
      expect(agencyIds.has(g.agency_id), `group "${g.name}" agency_id`).toBe(true)
  })

  it('artist.solo_agency_id references a known agency when set', () => {
    for (const a of dataset.artists)
      if (a.solo_agency_id)
        expect(agencyIds.has(a.solo_agency_id), `artist "${a.real_name}" solo_agency_id`).toBe(true)
  })
})

// ─── FK : groups ──────────────────────────────────────────────────────────────

describe('FK → groups', () => {
  it('group_member.group_id references a known group', () => {
    for (const m of dataset.group_members)
      expect(groupIds.has(m.group_id), `group_member artist=${m.artist_id}`).toBe(true)
  })

  it('album.group_id references a known group when set', () => {
    for (const a of dataset.albums)
      if (a.group_id)
        expect(groupIds.has(a.group_id), `album "${a.name}" group_id`).toBe(true)
  })

  it('lightstick.group_id references a known group when set', () => {
    for (const ls of dataset.lightsticks)
      if (ls.group_id)
        expect(groupIds.has(ls.group_id), `lightstick "${ls.name}" group_id`).toBe(true)
  })

  it('fanclub_kit.group_id references a known group when set', () => {
    for (const k of dataset.fanclub_kits)
      if (k.group_id)
        expect(groupIds.has(k.group_id), `fanclub_kit "${k.name}" group_id`).toBe(true)
  })
})

// ─── FK : artists ─────────────────────────────────────────────────────────────

describe('FK → artists', () => {
  it('group_member.artist_id references a known artist', () => {
    for (const m of dataset.group_members)
      expect(artistIds.has(m.artist_id), `group_member artist=${m.artist_id}`).toBe(true)
  })

  it('artist_alias.artist_id references a known artist', () => {
    for (const al of dataset.artist_aliases)
      expect(artistIds.has(al.artist_id), `alias "${al.name}" artist_id`).toBe(true)
  })

  it('album.artist_id references a known artist when set', () => {
    for (const a of dataset.albums)
      if (a.artist_id)
        expect(artistIds.has(a.artist_id), `album "${a.name}" artist_id`).toBe(true)
  })

  it('digipack.artist_id references a known artist when set', () => {
    for (const dp of dataset.digipacks)
      if (dp.artist_id)
        expect(artistIds.has(dp.artist_id), `digipack "${dp.name}" artist_id`).toBe(true)
  })

  it('lightstick.artist_id references a known artist when set', () => {
    for (const ls of dataset.lightsticks)
      if (ls.artist_id)
        expect(artistIds.has(ls.artist_id), `lightstick "${ls.name}" artist_id`).toBe(true)
  })

  it('fanclub_kit.artist_id references a known artist when set', () => {
    for (const k of dataset.fanclub_kits)
      if (k.artist_id)
        expect(artistIds.has(k.artist_id), `fanclub_kit "${k.name}" artist_id`).toBe(true)
  })

  it('photocard.artist_id references a known artist when set', () => {
    for (const pc of dataset.photocards)
      if (pc.artist_id)
        expect(artistIds.has(pc.artist_id), `photocard "${pc.name}" artist_id`).toBe(true)
  })
})

// ─── FK : albums ──────────────────────────────────────────────────────────────

describe('FK → albums', () => {
  it('album_version.album_id references a known album', () => {
    for (const v of dataset.album_versions)
      expect(albumIds.has(v.album_id), `album_version "${v.name}" album_id`).toBe(true)
  })

  it('digipack.album_id references a known album', () => {
    for (const dp of dataset.digipacks)
      expect(albumIds.has(dp.album_id), `digipack "${dp.name}" album_id`).toBe(true)
  })
})

// ─── FK : album_versions & digipacks ─────────────────────────────────────────

describe('FK → album_versions / digipacks', () => {
  it('photocard.album_version_id references a known album_version when set', () => {
    for (const pc of dataset.photocards)
      if (pc.album_version_id)
        expect(albumVersionIds.has(pc.album_version_id), `photocard "${pc.name}" album_version_id`).toBe(true)
  })

  it('photocard.digipack_id references a known digipack when set', () => {
    for (const pc of dataset.photocards)
      if (pc.digipack_id)
        expect(digipackIds.has(pc.digipack_id), `photocard "${pc.name}" digipack_id`).toBe(true)
  })
})

// ─── Contraintes métier ───────────────────────────────────────────────────────

describe('business rules', () => {
  it('each album belongs to exactly one of group or artist', () => {
    for (const a of dataset.albums) {
      const hasGroup  = a.group_id  !== null
      const hasArtist = a.artist_id !== null
      expect(hasGroup !== hasArtist, `album "${a.name}" must have exactly one of group_id / artist_id`).toBe(true)
    }
  })

  it('each photocard has exactly one source FK (album_version or digipack)', () => {
    for (const pc of dataset.photocards) {
      const fkCount = [pc.album_version_id, pc.digipack_id].filter(Boolean).length
      expect(fkCount, `photocard "${pc.name}" must have exactly one FK`).toBe(1)
    }
  })

  it('each group_member pair (artist_id, group_id) is unique', () => {
    const seen = new Set<string>()
    for (const m of dataset.group_members) {
      const key = `${m.artist_id}:${m.group_id}`
      expect(seen.has(key), `duplicate group_member ${key}`).toBe(false)
      seen.add(key)
    }
  })

  it('each artist has at most one primary alias per kind', () => {
    const primaryByKind = new Map<string, Set<string>>()
    for (const al of dataset.artist_aliases) {
      if (!al.is_primary) continue
      const key = `${al.artist_id}:${al.kind}`
      expect(primaryByKind.has(key), `artist ${al.artist_id} has multiple primary aliases of kind "${al.kind}"`).toBe(false)
      primaryByKind.set(key, new Set([al.id]))
    }
  })

  it('album_version formats are within the allowed enum', () => {
    const ALLOWED = new Set(['cd', 'ep', 'album', 'mini_album', 'vinyl', 'kit'])
    for (const v of dataset.album_versions)
      expect(ALLOWED.has(v.format), `album_version "${v.name}" has unknown format "${v.format}"`).toBe(true)
  })

  it('alias kinds are within the allowed enum', () => {
    const ALLOWED = new Set(['group_stage', 'solo_stage', 'original'])
    for (const al of dataset.artist_aliases)
      expect(ALLOWED.has(al.kind), `alias "${al.name}" has unknown kind "${al.kind}"`).toBe(true)
  })

  it('every group_stage alias artist is a group member', () => {
    for (const al of dataset.artist_aliases) {
      if (al.kind !== 'group_stage') continue
      const isMember = dataset.group_members.some(m => m.artist_id === al.artist_id)
      expect(isMember, `alias "${al.name}" (group_stage) artist has no group_member row`).toBe(true)
    }
  })

  it('artists with solo_debut_date have solo_agency_id set', () => {
    for (const a of dataset.artists) {
      if (a.solo_debut_date) {
        expect(a.solo_agency_id, `artist "${a.real_name}" has solo_debut_date but no solo_agency_id`).not.toBeNull()
      }
    }
  })

  it('lightstick or fanclub_kit has at least one of group_id / artist_id set', () => {
    for (const ls of dataset.lightsticks) {
      const hasOwner = ls.group_id !== null || ls.artist_id !== null
      expect(hasOwner, `lightstick "${ls.name}" has no owner`).toBe(true)
    }
    for (const k of dataset.fanclub_kits) {
      const hasOwner = k.group_id !== null || k.artist_id !== null
      expect(hasOwner, `fanclub_kit "${k.name}" has no owner`).toBe(true)
    }
  })
})