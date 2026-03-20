-- Your SQL goes here
CREATE VIRTUAL TABLE IF NOT EXISTS collection_groups_fts USING fts5 (group_id UNINDEXED, name, tokenize = 'unicode61');

CREATE TRIGGER IF NOT EXISTS uc_after_insert_collection_groups_fts AFTER INSERT ON user_collection BEGIN
DELETE FROM collection_groups_fts
WHERE
  group_id IN (
    SELECT
      al.group_id
    FROM
      album_versions av
      JOIN albums al ON al.id = av.album_id
      AND al.group_id IS NOT NULL
    WHERE
      av.id = NEW.album_version_id
    UNION
    SELECT
      al.group_id
    FROM
      digipacks dp
      JOIN albums al ON al.id = dp.album_id
      AND al.group_id IS NOT NULL
    WHERE
      dp.id = NEW.digipack_id
    UNION
    SELECT
      group_id
    FROM
      lightsticks
    WHERE
      id = NEW.lightstick_id
      AND group_id IS NOT NULL
    UNION
    SELECT
      group_id
    FROM
      fanclub_kits
    WHERE
      id = NEW.fanclub_kit_id
      AND group_id IS NOT NULL
  );

INSERT INTO
  collection_groups_fts (group_id, name)
SELECT
  g.id,
  g.name
FROM
  groups g
WHERE
  g.is_deleted = 0
  AND g.id IN (
    SELECT
      al.group_id
    FROM
      album_versions av
      JOIN albums al ON al.id = av.album_id
      AND al.group_id IS NOT NULL
    WHERE
      av.id = NEW.album_version_id
    UNION
    SELECT
      al.group_id
    FROM
      digipacks dp
      JOIN albums al ON al.id = dp.album_id
      AND al.group_id IS NOT NULL
    WHERE
      dp.id = NEW.digipack_id
    UNION
    SELECT
      group_id
    FROM
      lightsticks
    WHERE
      id = NEW.lightstick_id
      AND group_id IS NOT NULL
    UNION
    SELECT
      group_id
    FROM
      fanclub_kits
    WHERE
      id = NEW.fanclub_kit_id
      AND group_id IS NOT NULL
  )
  AND EXISTS (
    SELECT
      1
    FROM
      user_collection uc2
      JOIN album_versions av2 ON av2.id = uc2.album_version_id
      JOIN albums al2 ON al2.id = av2.album_id
      AND al2.group_id = g.id
    UNION ALL
    SELECT
      1
    FROM
      user_collection uc2
      JOIN digipacks dp2 ON dp2.id = uc2.digipack_id
      JOIN albums al2 ON al2.id = dp2.album_id
      AND al2.group_id = g.id
    UNION ALL
    SELECT
      1
    FROM
      user_collection uc2
      JOIN lightsticks ls2 ON ls2.id = uc2.lightstick_id
      AND ls2.group_id = g.id
    UNION ALL
    SELECT
      1
    FROM
      user_collection uc2
      JOIN fanclub_kits fk2 ON fk2.id = uc2.fanclub_kit_id
      AND fk2.group_id = g.id
  );

END;

CREATE TRIGGER IF NOT EXISTS uc_after_delete_collection_groups_fts AFTER DELETE ON user_collection BEGIN
DELETE FROM collection_groups_fts
WHERE
  group_id IN (
    SELECT
      al.group_id
    FROM
      album_versions av
      JOIN albums al ON al.id = av.album_id
      AND al.group_id IS NOT NULL
    WHERE
      av.id = OLD.album_version_id
    UNION
    SELECT
      al.group_id
    FROM
      digipacks dp
      JOIN albums al ON al.id = dp.album_id
      AND al.group_id IS NOT NULL
    WHERE
      dp.id = OLD.digipack_id
    UNION
    SELECT
      group_id
    FROM
      lightsticks
    WHERE
      id = OLD.lightstick_id
      AND group_id IS NOT NULL
    UNION
    SELECT
      group_id
    FROM
      fanclub_kits
    WHERE
      id = OLD.fanclub_kit_id
      AND group_id IS NOT NULL
  );

INSERT INTO
  collection_groups_fts (group_id, name)
SELECT
  g.id,
  g.name
FROM
  groups g
WHERE
  g.is_deleted = 0
  AND g.id IN (
    SELECT
      al.group_id
    FROM
      album_versions av
      JOIN albums al ON al.id = av.album_id
      AND al.group_id IS NOT NULL
    WHERE
      av.id = OLD.album_version_id
    UNION
    SELECT
      al.group_id
    FROM
      digipacks dp
      JOIN albums al ON al.id = dp.album_id
      AND al.group_id IS NOT NULL
    WHERE
      dp.id = OLD.digipack_id
    UNION
    SELECT
      group_id
    FROM
      lightsticks
    WHERE
      id = OLD.lightstick_id
      AND group_id IS NOT NULL
    UNION
    SELECT
      group_id
    FROM
      fanclub_kits
    WHERE
      id = OLD.fanclub_kit_id
      AND group_id IS NOT NULL
  )
  AND EXISTS (
    SELECT
      1
    FROM
      user_collection uc2
      JOIN album_versions av2 ON av2.id = uc2.album_version_id
      JOIN albums al2 ON al2.id = av2.album_id
      AND al2.group_id = g.id
    UNION ALL
    SELECT
      1
    FROM
      user_collection uc2
      JOIN digipacks dp2 ON dp2.id = uc2.digipack_id
      JOIN albums al2 ON al2.id = dp2.album_id
      AND al2.group_id = g.id
    UNION ALL
    SELECT
      1
    FROM
      user_collection uc2
      JOIN lightsticks ls2 ON ls2.id = uc2.lightstick_id
      AND ls2.group_id = g.id
    UNION ALL
    SELECT
      1
    FROM
      user_collection uc2
      JOIN fanclub_kits fk2 ON fk2.id = uc2.fanclub_kit_id
      AND fk2.group_id = g.id
  );

END;