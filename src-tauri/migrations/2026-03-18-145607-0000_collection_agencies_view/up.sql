-- Your SQL goes here
CREATE VIEW
  IF NOT EXISTS collection_agencies_view AS
SELECT DISTINCT
  ag.id AS agency_id,
  ag.name AS agency_name
FROM
  user_collection uc
  LEFT JOIN album_versions av ON av.id = uc.album_version_id
  LEFT JOIN albums al_av ON al_av.id = av.album_id
  LEFT JOIN groups g_av ON g_av.id = al_av.group_id
  AND g_av.is_deleted = 0
  LEFT JOIN artists ar_av ON ar_av.id = al_av.artist_id
  AND ar_av.is_deleted = 0
  LEFT JOIN digipacks dp ON dp.id = uc.digipack_id
  LEFT JOIN albums al_dp ON al_dp.id = dp.album_id
  LEFT JOIN groups g_dp ON g_dp.id = al_dp.group_id
  AND g_dp.is_deleted = 0
  LEFT JOIN artists ar_dp ON ar_dp.id = dp.artist_id
  AND ar_dp.is_deleted = 0
  LEFT JOIN lightsticks ls ON ls.id = uc.lightstick_id
  LEFT JOIN groups g_ls ON g_ls.id = ls.group_id
  AND g_ls.is_deleted = 0
  LEFT JOIN artists ar_ls ON ar_ls.id = ls.artist_id
  AND ar_ls.is_deleted = 0
  LEFT JOIN fanclub_kits fk ON fk.id = uc.fanclub_kit_id
  LEFT JOIN groups g_fk ON g_fk.id = fk.group_id
  AND g_fk.is_deleted = 0
  LEFT JOIN artists ar_fk ON ar_fk.id = fk.artist_id
  AND ar_fk.is_deleted = 0
  JOIN agencies ag ON ag.id IN (
    SELECT
      agency_id
    FROM
      groups
    WHERE
      id IN (g_av.id, g_dp.id, g_ls.id, g_fk.id)
    UNION
    SELECT
      solo_agency_id
    FROM
      artists
    WHERE
      id IN (ar_av.id, ar_dp.id, ar_ls.id, ar_fk.id)
  );