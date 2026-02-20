import { describe, it, expect } from 'vitest'
import dataset from './public/dataset.json' assert { type: 'json' }

import type { Group, Album, Lightstick } from './src/types'

function isUuidV4(s: unknown): s is string {
  if (typeof s !== 'string') return false
  return /^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/i.test(s)
}

function isIsoTimestamp(s: unknown): s is string {
  if (typeof s !== 'string') return false
  return /^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(?:\.\d+)?Z$/.test(s) && !Number.isNaN(Date.parse(s))
}

function isYyyyMmDd(s: unknown): s is string {
  if (typeof s !== 'string') return false
  if (!/^\d{4}-\d{2}-\d{2}$/.test(s)) return false
  const [y, m, d] = s.split('-').map(Number)
  const dt = new Date(Date.UTC(y, m - 1, d))
  return dt.getUTCFullYear() === y && dt.getUTCMonth() === m - 1 && dt.getUTCDate() === d
}

function hasOwn(obj: Record<string, unknown>, key: string) {
  return Object.prototype.hasOwnProperty.call(obj, key)
}

describe('Dataset structure', () => {
  it('root contains required top-level keys with correct types', () => {
    expect(dataset).toBeTruthy()
    expect(typeof dataset.datasetVersion).toBe('string')
    expect(isIsoTimestamp(dataset.generatedAt)).toBe(true)
    expect(Array.isArray(dataset.groups)).toBe(true)
    expect(Array.isArray(dataset.albums)).toBe(true)
    expect(Array.isArray(dataset.lightsticks)).toBe(true)
  })
})

describe('UUID validation and uniqueness', () => {
  it('all IDs (groups/albums/lightsticks) are valid UUID v4 and globally unique', () => {
    const allIds: string[] = []
    for (const g of dataset.groups as Group[]) {
      expect(isUuidV4(g.id)).toBe(true)
      allIds.push(g.id)
    }
    for (const a of dataset.albums as Album[]) {
      expect(isUuidV4(a.id)).toBe(true)
      allIds.push(a.id)
    }
    for (const l of dataset.lightsticks as Lightstick[]) {
      expect(isUuidV4(l.id)).toBe(true)
      allIds.push(l.id)
    }
    const unique = new Set(allIds)
    expect(unique.size).toBe(allIds.length)
  })

  it('all referential IDs (groupId on albums/lightsticks) are valid UUID v4', () => {
    for (const a of dataset.albums as Album[]) {
      expect(isUuidV4(a.groupId)).toBe(true)
    }
    for (const l of dataset.lightsticks as Lightstick[]) {
      expect(isUuidV4(l.groupId)).toBe(true)
    }
  })
})

describe('Referential integrity', () => {
  it('every album.groupId references an existing group', () => {
    const groupIds = new Set((dataset.groups as Group[]).map(g => g.id))
    for (const a of dataset.albums as Album[]) {
      expect(groupIds.has(a.groupId)).toBe(true)
    }
  })

  it('every lightstick.groupId references an existing group', () => {
    const groupIds = new Set((dataset.groups as Group[]).map(g => g.id))
    for (const l of dataset.lightsticks as Lightstick[]) {
      expect(groupIds.has(l.groupId)).toBe(true)
    }
  })
})

describe('Field validation for groups, albums, and lightsticks', () => {
  it('groups contain all required fields with correct types', () => {
    for (const g of dataset.groups as Group[]) {
      expect(hasOwn(g as any, 'id')).toBe(true)
      expect(hasOwn(g as any, 'name')).toBe(true)
      expect(hasOwn(g as any, 'agency')).toBe(true)
      expect(hasOwn(g as any, 'debutYear')).toBe(true)
      expect(hasOwn(g as any, 'isActive')).toBe(true)

      expect(typeof g.name).toBe('string')
      expect(typeof g.agency).toBe('string')
      expect(Number.isInteger(g.debutYear)).toBe(true)
      expect(typeof g.isActive).toBe('boolean')
    }
  })

  it('albums contain all required fields and have valid values', () => {
    const allowedTypes = new Set(['EP', 'Album', 'Single'])
    for (const a of dataset.albums as Album[]) {
      expect(hasOwn(a as any, 'id')).toBe(true)
      expect(hasOwn(a as any, 'groupId')).toBe(true)
      expect(hasOwn(a as any, 'title')).toBe(true)
      expect(hasOwn(a as any, 'type')).toBe(true)
      expect(hasOwn(a as any, 'releaseDate')).toBe(true)
      expect(hasOwn(a as any, 'coverImage')).toBe(true)
      expect(hasOwn(a as any, 'barcode')).toBe(true)
      expect(hasOwn(a as any, 'verified')).toBe(true)

      expect(typeof a.title).toBe('string')
      expect(allowedTypes.has(a.type)).toBe(true)
      expect(isYyyyMmDd(a.releaseDate)).toBe(true)
      expect(typeof a.coverImage).toBe('string')
      expect(a.coverImage.length).toBeGreaterThan(0)
      expect(typeof a.barcode).toBe('string')
      expect(a.barcode.length).toBeGreaterThan(0)
      expect(typeof a.verified).toBe('boolean')
    }
  })

  it('lightsticks contain all required fields and have valid values', () => {
    for (const l of dataset.lightsticks as Lightstick[]) {
      expect(hasOwn(l as any, 'id')).toBe(true)
      expect(hasOwn(l as any, 'groupId')).toBe(true)
      expect(hasOwn(l as any, 'name')).toBe(true)
      expect(hasOwn(l as any, 'version')).toBe(true)
      expect(hasOwn(l as any, 'releaseYear')).toBe(true)
      expect(hasOwn(l as any, 'image')).toBe(true)
      expect(hasOwn(l as any, 'verified')).toBe(true)

      expect(typeof l.name).toBe('string')
      expect(l.name.length).toBeGreaterThan(0)
      expect(typeof l.version).toBe('string')
      expect(Number.isInteger(l.releaseYear)).toBe(true)
      expect(typeof l.image).toBe('string')
      expect(l.image.length).toBeGreaterThan(0)
      expect(typeof l.verified).toBe('boolean')
    }
  })
})

