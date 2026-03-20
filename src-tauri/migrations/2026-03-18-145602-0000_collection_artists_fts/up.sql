-- Your SQL goes here
CREATE VIRTUAL TABLE IF NOT EXISTS collection_artists_fts USING fts5 (artist_id UNINDEXED, name, tokenize = 'unicode61');

CREATE TRIGGER IF NOT EXISTS uc_after_insert_collection_artists_fts AFTER INSERT ON user_collection BEGIN
DELETE FROM collection_artists_fts
WHERE
  artist_id IN (
    SELECT
      al.artist_id
    FROM
      album_versions av
      JOIN albums al ON al.id = av.album_id
      AND al.artist_id IS NOT NULL
    WHERE
      av.id = NEW.album_version_id
    UNION
    SELECT
      artist_id
    FROM
      digipacks
    WHERE
      id = NEW.digipack_id
      AND artist_id IS NOT NULL
    UNION
    SELECT
      artist_id
    FROM
      lightsticks
    WHERE
      id = NEW.lightstick_id
      AND artist_id IS NOT NULL
    UNION
    SELECT
      artist_id
    FROM
      fanclub_kits
    WHERE
      id = NEW.fanclub_kit_id
      AND artist_id IS NOT NULL
  );

INSERT INTO
  collection_artists_fts (artist_id, name)
SELECT
  ar.id,
  ar.real_name || ' ' || COALESCE(
    (
      SELECT
        GROUP_CONCAT (aa.name, ' ')
      FROM
        artist_aliases aa
      WHERE
        aa.artist_id = ar.id
        AND aa.is_deleted = 0
    ),
    ''
  )
FROM
  artists ar
WHERE
  ar.is_deleted = 0
  AND ar.solo_agency_id IS NOT NULL
  AND ar.id IN (
    SELECT
      al.artist_id
    FROM
      album_versions av
      JOIN albums al ON al.id = av.album_id
      AND al.artist_id IS NOT NULL
    WHERE
      av.id = NEW.album_version_id
    UNION
    SELECT
      artist_id
    FROM
      digipacks
    WHERE
      id = NEW.digipack_id
      AND artist_id IS NOT NULL
    UNION
    SELECT
      artist_id
    FROM
      lightsticks
    WHERE
      id = NEW.lightstick_id
      AND artist_id IS NOT NULL
    UNION
    SELECT
      artist_id
    FROM
      fanclub_kits
    WHERE
      id = NEW.fanclub_kit_id
      AND artist_id IS NOT NULL
  )
  AND EXISTS (
    SELECT
      1
    FROM
      user_collection uc2
      JOIN album_versions av2 ON av2.id = uc2.album_version_id
      JOIN albums al2 ON al2.id = av2.album_id
      AND al2.artist_id = ar.id
    UNION ALL
    SELECT
      1
    FROM
      user_collection uc2
      JOIN digipacks dp2 ON dp2.id = uc2.digipack_id
      AND dp2.artist_id = ar.id
    UNION ALL
    SELECT
      1
    FROM
      user_collection uc2
      JOIN lightsticks ls2 ON ls2.id = uc2.lightstick_id
      AND ls2.artist_id = ar.id
    UNION ALL
    SELECT
      1
    FROM
      user_collection uc2
      JOIN fanclub_kits fk2 ON fk2.id = uc2.fanclub_kit_id
      AND fk2.artist_id = ar.id
  );

END;

CREATE TRIGGER IF NOT EXISTS uc_after_delete_collection_artists_fts AFTER DELETE ON user_collection BEGIN
DELETE FROM collection_artists_fts
WHERE
  artist_id IN (
    SELECT
      al.artist_id
    FROM
      album_versions av
      JOIN albums al ON al.id = av.album_id
      AND al.artist_id IS NOT NULL
    WHERE
      av.id = OLD.album_version_id
    UNION
    SELECT
      artist_id
    FROM
      digipacks
    WHERE
      id = OLD.digipack_id
      AND artist_id IS NOT NULL
    UNION
    SELECT
      artist_id
    FROM
      lightsticks
    WHERE
      id = OLD.lightstick_id
      AND artist_id IS NOT NULL
    UNION
    SELECT
      artist_id
    FROM
      fanclub_kits
    WHERE
      id = OLD.fanclub_kit_id
      AND artist_id IS NOT NULL
  );

INSERT INTO
  collection_artists_fts (artist_id, name)
SELECT
  ar.id,
  ar.real_name || ' ' || COALESCE(
    (
      SELECT
        GROUP_CONCAT (aa.name, ' ')
      FROM
        artist_aliases aa
      WHERE
        aa.artist_id = ar.id
        AND aa.is_deleted = 0
    ),
    ''
  )
FROM
  artists ar
WHERE
  ar.is_deleted = 0
  AND ar.solo_agency_id IS NOT NULL
  AND ar.id IN (
    SELECT
      al.artist_id
    FROM
      album_versions av
      JOIN albums al ON al.id = av.album_id
      AND al.artist_id IS NOT NULL
    WHERE
      av.id = OLD.album_version_id
    UNION
    SELECT
      artist_id
    FROM
      digipacks
    WHERE
      id = OLD.digipack_id
      AND artist_id IS NOT NULL
    UNION
    SELECT
      artist_id
    FROM
      lightsticks
    WHERE
      id = OLD.lightstick_id
      AND artist_id IS NOT NULL
    UNION
    SELECT
      artist_id
    FROM
      fanclub_kits
    WHERE
      id = OLD.fanclub_kit_id
      AND artist_id IS NOT NULL
  )
  AND EXISTS (
    SELECT
      1
    FROM
      user_collection uc2
      JOIN album_versions av2 ON av2.id = uc2.album_version_id
      JOIN albums al2 ON al2.id = av2.album_id
      AND al2.artist_id = ar.id
    UNION ALL
    SELECT
      1
    FROM
      user_collection uc2
      JOIN digipacks dp2 ON dp2.id = uc2.digipack_id
      AND dp2.artist_id = ar.id
    UNION ALL
    SELECT
      1
    FROM
      user_collection uc2
      JOIN lightsticks ls2 ON ls2.id = uc2.lightstick_id
      AND ls2.artist_id = ar.id
    UNION ALL
    SELECT
      1
    FROM
      user_collection uc2
      JOIN fanclub_kits fk2 ON fk2.id = uc2.fanclub_kit_id
      AND fk2.artist_id = ar.id
  );

END;