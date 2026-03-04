import { describe, it, expect } from "vitest";
import dataset from "../../public/dataset.json" assert { type: "json" };

function isUuidV4(s: unknown): s is string {
  if (typeof s !== "string") return false;
  return /^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/i.test(
    s,
  );
}

function isIsoTimestamp(s: unknown): s is string {
  if (typeof s !== "string") return false;
  return (
    /^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(?:\.\d+)?Z$/.test(s) &&
    !Number.isNaN(Date.parse(s))
  );
}

function hasOwn(obj: Record<string, unknown>, key: string) {
  return Object.hasOwn(obj, key);
}

describe("Dataset structure", () => {
  it("root contains required top-level keys with correct types", () => {
    expect(dataset).toBeTruthy();
    expect(typeof dataset.datasetVersion).toBe("string");
    expect(isIsoTimestamp(dataset.generatedAt)).toBe(true);
    expect(Array.isArray(dataset.groups)).toBe(true);
    expect(Array.isArray(dataset.contexts)).toBe(true);
    expect(Array.isArray(dataset.collectibles)).toBe(true);
  });
});

describe("UUID validation and uniqueness", () => {
  it("all IDs (groups/contexts/collectibles) are valid UUID v4 and globally unique", () => {
    const allIds: string[] = [];

    for (const g of dataset.groups) {
      expect(isUuidV4(g.id)).toBe(true);
      allIds.push(g.id);
    }

    for (const c of dataset.contexts) {
      expect(isUuidV4(c.id)).toBe(true);
      allIds.push(c.id);
    }

    for (const col of dataset.collectibles) {
      expect(isUuidV4(col.id)).toBe(true);
      allIds.push(col.id);
    }

    const unique = new Set(allIds);
    expect(unique.size).toBe(allIds.length);
  });

  it("all referential IDs (groupId) are valid UUID v4", () => {
    for (const c of dataset.contexts) {
      expect(isUuidV4(c.groupId)).toBe(true);
    }

    for (const col of dataset.collectibles) {
      expect(isUuidV4(col.groupId)).toBe(true);
    }
  });
});

describe("Referential integrity", () => {
  it("every context.groupId references an existing group", () => {
    const groupIds = new Set(dataset.groups.map((g) => g.id));

    for (const c of dataset.contexts) {
      expect(groupIds.has(c.groupId)).toBe(true);
    }
  });

  it("every collectible.groupId references an existing group", () => {
    const groupIds = new Set(dataset.groups.map((g) => g.id));

    for (const col of dataset.collectibles) {
      expect(groupIds.has(col.groupId)).toBe(true);
    }
  });
});

describe("Field validation for groups", () => {
  it("groups contain all required fields with correct types", () => {
    for (const g of dataset.groups) {
      expect(hasOwn(g, "id")).toBe(true);
      expect(hasOwn(g, "name")).toBe(true);
      expect(hasOwn(g, "agency")).toBe(true);
      expect(hasOwn(g, "debutYear")).toBe(true);
      expect(hasOwn(g, "isActive")).toBe(true);

      expect(typeof g.name).toBe("string");
      expect(typeof g.agency).toBe("string");
      expect(Number.isInteger(g.debutYear)).toBe(true);
      expect(typeof g.isActive).toBe("boolean");
    }
  });
});

describe("Field validation for collectibles", () => {
  it("collectibles contain base required fields", () => {
    for (const col of dataset.collectibles) {
      expect(hasOwn(col, "id")).toBe(true);
      expect(hasOwn(col, "groupId")).toBe(true);
      expect(hasOwn(col, "kind")).toBe(true);
      expect(hasOwn(col, "name")).toBe(true);
      expect(hasOwn(col, "releaseYear")).toBe(true);
      expect(hasOwn(col, "image")).toBe(true);
      expect(hasOwn(col, "verified")).toBe(true);

      expect(typeof col.name).toBe("string");
      expect(col.name.length).toBeGreaterThan(0);
      expect(col.releaseDate).toBe(true);
      expect(typeof col.image).toBe("string");
      expect(col.image.length).toBeGreaterThan(0);
      expect(typeof col.verified).toBe("boolean");
    }
  });

  it("Album collectibles have a valid subType", () => {
    const allowedSubTypes = new Set([
      "Single Album",
      "EP",
      "Studio Album",
      "Anthology",
    ]);

    for (const col of dataset.collectibles) {
      if (col.kind === "Album") {
        expect(hasOwn(col, "subType")).toBe(true);
        expect(allowedSubTypes.has(col.subType)).toBe(true);
      }
    }
  });

  it("Lightstick collectibles do not require subType", () => {
    for (const col of dataset.collectibles) {
      if (col.kind === "Lightstick") {
        expect(typeof col.name).toBe("string");
      }
    }
  });
});
