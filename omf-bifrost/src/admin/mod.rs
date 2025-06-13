use anyhow::{Context, Result};
use duckdb::Connection as DuckConnection;
use log::info;
use rusqlite::{Connection, params};
use std::fs;
use std::path::Path;

mod config;

pub use config::{
    AccessMode, AdminConfig, HighwayType, load_admin_config, save_default_admin_config,
};

const CREATE_ADMINS_SQL: &str = "CREATE TABLE admins (
                admin_level INTEGER NOT NULL,
                iso_code TEXT,
                parent_admin INTEGER,
                name TEXT NOT NULL,
                name_en TEXT,
                drive_on_right INTEGER NULL,
                allow_intersection_names INTEGER NULL,
                default_language TEXT,
                supported_languages TEXT,
                division_id TEXT,
                parent_division_id TEXT
);";

const ADD_ADMINS_GEOM_SQL: &str =
    "SELECT AddGeometryColumn('admins', 'geom', 4326, 'MULTIPOLYGON', 2);";

const CREATE_ADMIN_INDEX_SQL: &str = "CREATE INDEX IdxLevel ON admins ('admin_level');";
const CREATE_ADMIN_DRIVE_INDEX_SQL: &str =
    "CREATE INDEX IdxDriveOnRight ON admins ('drive_on_right');";
const CREATE_ADMIN_INTERSECTION_INDEX_SQL: &str =
    "CREATE INDEX IdxAllowIntersectionNames ON admins ('allow_intersection_names');";
const CREATE_SPATIAL_INDEX_SQL: &str = "SELECT CreateSpatialIndex('admins', 'geom');";

const CREATE_ADMIN_ACCESS_SQL: &str = "CREATE TABLE admin_access (
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
);";

fn modes_to_bitmask(modes: &[AccessMode]) -> Option<i64> {
    let bm = modes.iter().map(|m| m.bit()).fold(0, |acc, bit| acc | bit);
    if bm == 0 { None } else { Some(bm) }
}

fn get_iso_code(admin_level: i64, country: &str, region: &Option<String>) -> Option<String> {
    if admin_level == 2 {
        Some(country.into())
    } else if admin_level == 4 {
        region
            .as_ref()
            .map(|r| r.split('-').nth(1).unwrap_or(r).to_string())
    } else {
        None
    }
}

fn get_drive_on_right(driving_side: &Option<String>) -> Option<i64> {
    match driving_side.as_deref() {
        Some("right") => Some(1),
        Some("left") => Some(0),
        _ => None,
    }
}

fn get_allow_intersection_names(country: &str, admin_config: &AdminConfig) -> i64 {
    admin_config
        .allow_intersection_names
        .get(country)
        .map(|&b| if b { 1 } else { 0 })
        .unwrap_or(0)
}

/// Creates and populates the Valhalla-compatible 'admins' table in SQLite, ingesting and transforming data from the Overture DuckDB source tables.
fn build_admins_table(
    duck_con: &DuckConnection,
    sqlite_con: &Connection,
    geoparquet_division_path: &str,
    geoparquet_area_path: &str,
    admin_config: &AdminConfig,
) -> Result<()> {
    info!("Creating admins table");
    sqlite_con.execute_batch(CREATE_ADMINS_SQL)?;

    info!("Adding geometry column to admins table");
    sqlite_con.execute_batch(ADD_ADMINS_GEOM_SQL)?;

    info!("Querying and joining division and area tables in DuckDB");
    let select_query = format!(
        "WITH divs AS (
            SELECT
                id as div_id,
                parent_division_id,
                norms.driving_side
         FROM read_parquet('{}')
        )
        SELECT
            area.division_id,
            divs.parent_division_id,
            CASE area.subtype
                WHEN 'country' THEN 2
                WHEN 'dependency' THEN 2
                WHEN 'region' THEN 4
                ELSE NULL END as admin_level,
            area.country,
            area.region,
            area.names.primary as name,
            area.names.common.en as name_en,
            divs.driving_side,
            ST_AsText(area.geometry) as wkt
        FROM read_parquet('{}') as area
        JOIN divs ON area.division_id = divs.div_id
        WHERE area.is_land = TRUE
            AND area.geometry IS NOT NULL
            AND area.subtype IN ('country','dependency','region')",
        geoparquet_division_path, geoparquet_area_path
    );

    let mut stmt = duck_con.prepare(&select_query)?;
    let mut rows = stmt.query([])?;

    info!("Processing admin records");
    sqlite_con.execute_batch("BEGIN;")?;
    let mut admin_count = 0u64;
    while let Some(row) = rows.next()? {
        let division_id: String = row.get(0)?;
        let parent_division_id: Option<String> = row.get(1)?;
        let admin_level: Option<i64> = row.get(2)?;
        let country: String = row.get(3)?;
        let region: Option<String> = row.get(4)?;
        let name: Option<String> = row.get(5)?;
        let name_en: Option<String> = row.get(6)?;
        let driving_side: Option<String> = row.get(7)?;
        let geom_wkt: String = row.get(8)?;

        let admin_level = match admin_level {
            Some(lvl) => lvl,
            _ => continue,
        };

        let iso_code = get_iso_code(admin_level, &country, &region);
        let drive_on_right = get_drive_on_right(&driving_side);
        let allow_intersection_names = get_allow_intersection_names(&country, admin_config);

        sqlite_con.execute(
            "INSERT INTO admins (
                admin_level, iso_code, parent_admin, name, name_en,
                drive_on_right, allow_intersection_names, default_language,
                supported_languages, geom, division_id, parent_division_id
            ) VALUES (?, ?, NULL, ?, ?, ?, ?, ?, ?, CastToMulti(GeomFromText(?, 4326)), ?, ?)",
            params![
                admin_level,
                iso_code,
                name.unwrap_or_default(),
                name_en.unwrap_or_default(),
                drive_on_right,
                allow_intersection_names,
                Option::<String>::None,
                Option::<String>::None,
                geom_wkt,
                division_id,
                parent_division_id
            ],
        )?;

        admin_count += 1;
        if admin_count % 1000 == 0 {
            info!("{} admins processed so far...", admin_count);
        }
    }
    info!("Finished inserting admin rows: {} total", admin_count);
    sqlite_con.execute_batch("COMMIT;")?;

    info!("Assigning parent_admin values");
    sqlite_con.execute_batch(
        "UPDATE admins AS child
        SET parent_admin = (
            SELECT parent.rowid FROM admins AS parent
            WHERE parent.division_id = child.parent_division_id
        )
        WHERE child.parent_division_id IS NOT NULL;",
    )?;

    info!("Dropping temporary columns");
    sqlite_con.execute_batch(
        "ALTER TABLE admins DROP COLUMN division_id;
         ALTER TABLE admins DROP COLUMN parent_division_id;",
    )?;

    info!("Updating drive_on_right");
    sqlite_con.execute_batch(
        "UPDATE admins
        SET drive_on_right = (
          SELECT parent.drive_on_right
          FROM admins parent
          WHERE parent.rowid = admins.parent_admin
        )
        WHERE drive_on_right IS NULL;",
    )?;

    info!("Creating spatial index");
    sqlite_con.execute_batch(CREATE_SPATIAL_INDEX_SQL)?;

    info!("Creating level index");
    sqlite_con.execute_batch(CREATE_ADMIN_INDEX_SQL)?;

    info!("Creating Drive On Right index");
    sqlite_con.execute_batch(CREATE_ADMIN_DRIVE_INDEX_SQL)?;

    info!("Creating allow intersection names index");
    sqlite_con.execute_batch(CREATE_ADMIN_INTERSECTION_INDEX_SQL)?;
    Ok(())
}

/// Populates the `admin_access` table with per-country access rules from the admin config.
fn build_admin_access_table(sqlite_con: &Connection, admin_config: &AdminConfig) -> Result<()> {
    info!("Creating admin_access table");
    sqlite_con.execute_batch(CREATE_ADMIN_ACCESS_SQL)?;

    let mut stmt = sqlite_con.prepare(
        "INSERT INTO admin_access (
            admin_id, iso_code, trunk, trunk_link, track, footway, pedestrian, bridleway, cycleway, path, motorroad
        ) SELECT rowid, iso_code, ?, ?, ?, ?, ?, ?, ?, ?, ?
        FROM admins WHERE admin_level = 2 and iso_code = ?;"
    )?;
    for (country_code, access_rules) in &admin_config.admin_access {
        let trunk = access_rules
            .get(&HighwayType::Trunk)
            .and_then(|modes| modes_to_bitmask(modes));
        let trunk_link = access_rules
            .get(&HighwayType::TrunkLink)
            .and_then(|modes| modes_to_bitmask(modes));
        let track = access_rules
            .get(&HighwayType::Track)
            .and_then(|modes| modes_to_bitmask(modes));
        let footway = access_rules
            .get(&HighwayType::Footway)
            .and_then(|modes| modes_to_bitmask(modes));
        let pedestrian = access_rules
            .get(&HighwayType::Pedestrian)
            .and_then(|modes| modes_to_bitmask(modes));
        let bridleway = access_rules
            .get(&HighwayType::Bridleway)
            .and_then(|modes| modes_to_bitmask(modes));
        let cycleway = access_rules
            .get(&HighwayType::Cycleway)
            .and_then(|modes| modes_to_bitmask(modes));
        let path = access_rules
            .get(&HighwayType::Path)
            .and_then(|modes| modes_to_bitmask(modes));
        let motorroad = access_rules
            .get(&HighwayType::Motorroad)
            .and_then(|modes| modes_to_bitmask(modes));

        let params: [&dyn rusqlite::ToSql; 10] = [
            &trunk,
            &trunk_link,
            &track,
            &footway,
            &pedestrian,
            &bridleway,
            &cycleway,
            &path,
            &motorroad,
            country_code as &dyn rusqlite::ToSql,
        ];
        let updated = stmt.execute(rusqlite::params_from_iter(params))?;
        if updated == 0 {
            log::warn!(
                "No matching admin row found for admin_access rule '{}'. Ignore if not using a planet extract",
                country_code
            );
        }
    }

    Ok(())
}

/// Build the admin and access SQLite database from Overture DuckDB-derived geo-parquet division and area files, using a given AdminConfig.
/// The Spatialite extension must be available in the environment for spatial support.
pub fn build_admins_from_geo_parquet(
    geoparquet_division_path: &str,
    geoparquet_area_path: &str,
    sqlite_path: &str,
    admin_config: &AdminConfig,
) -> Result<()> {
    info!(
        "Preparing to build admins from {} and {} into {}",
        geoparquet_division_path, geoparquet_area_path, sqlite_path
    );

    if Path::new(sqlite_path).exists() {
        info!("Removing existing SQLite file at {}", sqlite_path);
        fs::remove_file(sqlite_path)?;
    }
    let sqlite_con = Connection::open(sqlite_path)?;

    // Set SQLite pragmas for faster bulk import (less durability during import but higher performance)
    sqlite_con.execute_batch(
        "PRAGMA synchronous = OFF;
             PRAGMA journal_mode = MEMORY;
             PRAGMA temp_store = MEMORY;",
    )?;

    unsafe {
        sqlite_con.load_extension_enable()?;
        sqlite_con.load_extension("mod_spatialite", None::<&str>)
            .context("Failed to load mod_spatialite extension. Make sure SpatiaLite is installed and 'mod_spatialite' is available in your library path.")?;
        sqlite_con.load_extension_disable()?;
    }
    sqlite_con.execute_batch("SELECT InitSpatialMetaData(1);")?;

    let duck_con = DuckConnection::open_in_memory()?;
    info!("Installing and loading DuckDB spatial extension");
    duck_con.execute("INSTALL spatial", [])?;
    duck_con.execute("LOAD spatial", [])?;

    build_admins_table(
        &duck_con,
        &sqlite_con,
        geoparquet_division_path,
        geoparquet_area_path,
        admin_config,
    )?;
    build_admin_access_table(&sqlite_con, admin_config)?;

    info!("Admin building completed and DB ready at {}", sqlite_path);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modes_to_bitmask_one() {
        assert_eq!(modes_to_bitmask(&[AccessMode::Bicycle]), Some(4));

        let bm = modes_to_bitmask(&[AccessMode::Bicycle, AccessMode::Pedestrian]);
        assert_eq!(bm, Some(6));

        assert_eq!(modes_to_bitmask(&[]), None);
    }

    #[test]
    fn test_get_iso_code_country_and_region() {
        let code = get_iso_code(2, "US", &Some("US-WA".into()));
        assert_eq!(code, Some("US".into()));

        let code = get_iso_code(4, "US", &Some("US-WA".into()));
        assert_eq!(code, Some("WA".into()));
    }

    #[test]
    fn test_get_drive_on_right_basic() {
        assert_eq!(get_drive_on_right(&Some("right".into())), Some(1));
        assert_eq!(get_drive_on_right(&Some("left".into())), Some(0));
        assert_eq!(get_drive_on_right(&Some("other".into())), None);
        assert_eq!(get_drive_on_right(&None), None);
    }

    #[test]
    fn test_get_allow_intersection_names_lookup() {
        let ac = AdminConfig::default();
        assert_eq!(get_allow_intersection_names("JP", &ac), 1);
        assert_eq!(get_allow_intersection_names("GB", &ac), 0);
        assert_eq!(get_allow_intersection_names("XX", &ac), 0);
    }
}
