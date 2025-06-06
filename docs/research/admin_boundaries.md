# Processing Overture Divisions for Valhalla: DuckDB & Spatialite Approach

## Overview

Valhalla requires a Spatialite (SQLite with spatial extensions) database for administrative boundary lookups during tile building. To use Overture Maps Foundation (OMF) Divisions (administrative boundaries) GeoParquet data for Valhalla, you must:

1. Filter and extract division boundaries from the Overture Divisions GeoParquet dataset (using DuckDB or similar tools).
2. Transform and load the data into a Valhalla-compatible Spatialite database, following Valhalla's strict schema and spatial requirements.

This document explains the process, requirements, and highlights source-backed important details about compatibility, explicitly reflecting the Overture Divisions schema, but tailored for Valhalla's admin import rules.

---

## DuckDB Capabilities for Processing Overture Divisions

DuckDB can efficiently filter and transform large Parquet/GeoParquet files with SQL and spatial extensions.

**Key Features:**
- Reads GeoParquet directly via `read_parquet()`
- Can project, filter, and convert geometries (with extensions like `ST_AsText`, `ST_Intersects` if spatial extension is installed)
- Can export to CSV, Arrow, Parquet, or trigger Python/R bindings for post-processing

*Example usage pattern:*
```sql
SELECT * FROM read_parquet('overture-divisions-area.parquet') LIMIT 5;
COPY (
    SELECT
        id,
        subtype,
        country,
        region,
        names,
        parent_division_id,
        class,
        ST_AsText(geometry) AS wkt
    FROM read_parquet('overture-divisions-area.parquet')
    WHERE is_land = TRUE
     AND geometry IS NOT NULL
) TO 'filtered_divisions.csv' (HEADER, DELIMITER ',');
```
---

## Valhalla Administrative Database Requirements

Valhalla uses admin data to support:
* Legal/directional rules (drive side, intersection rules)
* Crossing costs
* Instruction generation (user-facing narratives)
* Default access and speed estimation

**Required Table (`admins`) structure:**
```sql
CREATE TABLE admins (
    admin_level INTEGER NOT NULL,
    iso_code TEXT,
    parent_admin INTEGER,
    name TEXT NOT NULL,
    name_en TEXT,
    drive_on_right INTEGER NULL,
    allow_intersection_names INTEGER NULL,
    default_language TEXT,
    supported_languages TEXT
);
```

## Adminbuilder logic

| Field                      | Required? | Note                                                                                                       |
|----------------------------|-----------|------------------------------------------------------------------------------------------------------------|
| `admin_level`              | Yes       | From `admin_level` tag, only 2/4, with exceptions                                                          |
| `iso_code`                 | Yes       | From `ISO3166-1:alpha2` or `ISO3166-1` (level 2), or parsed from `ISO3166-2` (level 4); with exceptions    |
| `parent_admin`             | Yes       | Assigned via SQL spatial containment (`ST_Covers`)                                                         |
| `name`                     | Yes       | From `name` tag.                                                                                           |
| `name_en`                  | Optional  | From `name:en` tag if present                                                                              |
| `drive_on_right`           | Yes       | Hardcoded Lua lookup (`drive_on_right` table) using `name` or `name:en`; if not found, defaults to "true". |
| `allow_intersection_names` | Yes       | Hardcoded Lua lookup (`allow_intersection_names` table); if not found, defaults to "false".                |
| `default_language`         | Optional  | From `default_language` tag, with exceptions.                                                              |
| `supported_languages`      | Optional  | Hardcoded lookup (`kSupportedLanguages` table)                                                             |
| `geom` (MULTIPOLYGON 4326) | Yes       | Multipolygon SRID 4326, validated and indexed.                                                             |

## Exceptions and Special Cases

| Condition                                                                                               | Resulting Action and Value Assignment             |
|---------------------------------------------------------------------------------------------------------|---------------------------------------------------|
| admin_level == "6" and name == "District of Columbia"                                                   | admin_level = 4                                   |
| admin_level == "3" and name in [special-exception-list]                                                 | admin_level = 2 (see below)                       |
| admin_level == "2" and name in {"France", "United Kingdom"}                                             | Skip                                              |
| admin_level == "2" and name:en in {"Abkhazia", "South Ossetia"}                                         | admin_level = 4                                   |
| name == "Metro Manila"                                                                                  | admin_level = 4                                   |
| admin_level == "3" and name:en == "Metropolitan France"                                                 | admin_level = 2, name = "France", iso_code = "FR" |
| admin_level == "4" and name in {"England", "Alba / Scotland", "Cymru / Wales", "Northern Ireland"}      | set admin_level = 2                               |
| admin_level == "2" and name == "British Sovereign Base Areas"                                           | iso_code = "GB"                                   |
| None of the above                                                                                       | Skip                                              |

**Note:**
- "special-exception-list" for admin_level 3 are special French overseas regions and others explicitly listed.
---

**admin_access table**

The `admin_access` table is used to define access rules for different administrative levels. It is not directly derived from Overture data needs to be populated separately based on Valhalla's expectations.

```sql
-- Table for storing per-country/region access defaults
CREATE TABLE admin_access (
    admin_id INTEGER NOT NULL,
    iso_code TEXT,
    trunk INTEGER DEFAULT NULL,
    trunk_link INTEGER DEFAULT NULL,
    track INTEGER DEFAULT NULL,
    footway INTEGER DEFAULT NULL,
    pedestrian INTEGER DEFAULT NULL,
    bridleway INTEGER DEFAULT NULL,
    cycleway INTEGER DEFAULT NULL,
    path INTEGER DEFAULT NULL,
    motorroad INTEGER DEFAULT NULL
);
```

## Data Transformation Steps

1. Download Overture `division` and `division_area` GeoParquet dataset.
   - (optional) Download only areas overlapping your region of interest to reduce size
2. Use DuckDB to filter and extract relevant records:
   - Filter by `is_land = TRUE` to exclude water bodies.
   - Select only the required columns: `id`, `division_id`, `subtype`, `country`, `region`, `names`, `parent_division_id`, and the geometry.
   - Convert geometries to WKT MULTIPOLYGON format using `ST_AsText(geometry)`.
3. Transform the data to match Valhalla's `admins` table schema:
   - Map `subtype` to `admin_level`
   - Use `names["primary"]` as `name`, and `names["common"]["en"]` as `name_en` if available.
   - Set `drive_on_right` and `allow_intersection_names` based on static configuration.
   - `default_language` and `supported_languages` keep empty
4. Load the transformed data into a Spatialite database:
   - Create the `admins` table with the required schema.
   - Insert the transformed records into the `admins` table.
5. Assign `parent_admin` using `parent_division_id`, mapping to the numeric `rowid` in the SQLite/SpatiaLite database.
6. Validate the data:
   - Validate special cases and exceptions
   - Run spatial queries to ensure the geometries are correctly indexed and can be used for lookups.

---

## Typical Valhalla Admin Lookup Query

**Default language query:**
```sql
SELECT admin_level, supported_languages, default_language, st_astext(geom)
FROM admins
WHERE (supported_languages IS NOT NULL OR default_language IS NOT NULL)
  AND ST_Intersects(geom, BuildMBR(?, ?, ?, ?))
  AND admin_level > 4
  AND rowid IN (
    SELECT rowid FROM SpatialIndex
     WHERE f_table_name = 'admins'
       AND search_frame = BuildMBR(?, ?, ?, ?)
  )
ORDER BY admin_level DESC, name;
```

**Country-level lookup:**
```sql
SELECT name, "", iso_code, "", drive_on_right, allow_intersection_names, admin_level,
       supported_languages, default_language, st_astext(geom)
FROM admins
WHERE ST_Intersects(geom, BuildMBR(:minx, :miny, :maxx, :maxy))
  AND admin_level = 2
  AND rowid IN (
    SELECT rowid FROM SpatialIndex
    WHERE f_table_name = 'admins'
      AND search_frame = BuildMBR(:minx, :miny, :maxx, :maxy)
  )
ORDER BY name;
```

**State-level lookup:**
```sql
SELECT country.name, state.name, country.iso_code,
       state.iso_code, state.drive_on_right, state.allow_intersection_names, state.admin_level,
       state.supported_languages, state.default_language, st_astext(state.geom)
FROM admins state, admins country
WHERE ST_Intersects(state.geom, BuildMBR(:minx, :miny, :maxx, :maxy))
  AND country.rowid = state.parent_admin
  AND state.admin_level = 4
  AND state.rowid IN (
    SELECT rowid FROM SpatialIndex
    WHERE f_table_name = 'admins'
      AND search_frame = BuildMBR(:minx, :miny, :maxx, :maxy)
  )
ORDER BY state.name, country.name;
```

---

## Performance and Correctness Notes

- All polygons must be valid MULTIPOLYGON, CRS EPSG:4326.

---

## Known Limitations / Issues

- Mapping Overture subtypes to OSM/Valhalla `admin_level` is not 1:1 and must follow Valhalla remapping logic.
- Some fields (drive side, language) may be missing in Overture and must be filled with Valhalla hardcoded defaults or left NULL.
- The `default_language` and `supported_languages` columns are not part of the Overture schema (see `division_area.yaml` and related schemas) and will be left blank.

---

## References

- Valhalla admin table schema & usage: [`adminbuilder.cc`](https://github.com/valhalla/valhalla/blob/93b25cba491b660807e9033b80c23a1a81b3ae0f/src/mjolnir/adminbuilder.cc), [`admin.cc`](https://github.com/valhalla/valhalla/blob/93b25cba491b660807e9033b80c23a1a81b3ae0f/src/mjolnir/admin.cc), [`admin.lua`](https://github.com/valhalla/valhalla/blob/93b25cba491b660807e9033b80c23a1a81b3ae0f/lua/admin.lua)
- Overture divisions schema: [`divisions](https://github.com/OvertureMaps/schema/tree/7d20936cd1cc4af1a12c867fccd83b0c6f57cfef/schema/divisions)
- DuckDB spatial and Parquet reading: https://duckdb.org/docs/extensions/spatial.html
- Spatialite SQL functions: https://www.gaia-gis.it/fossil/libspatialite/index
