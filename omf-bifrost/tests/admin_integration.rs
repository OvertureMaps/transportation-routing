use omf_bifrost::admin::{AdminConfig, build_admins_from_geo_parquet};
use tempfile::tempdir;

#[derive(Debug)]
struct AdminRow {
    rowid: i64,
    admin_level: i32,
    iso_code: Option<String>,
    parent_admin: Option<i64>,
    name: String,
    name_en: Option<String>,
    drive_on_right: Option<i32>,
    allow_intersection_names: Option<i32>,
    default_language: Option<String>,
    supported_languages: Option<String>,
    geom: Option<Vec<u8>>,
}

impl AdminRow {
    fn from_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(AdminRow {
            rowid: row.get(0)?,
            admin_level: row.get(1)?,
            iso_code: row.get(2)?,
            parent_admin: row.get(3)?,
            name: row.get(4)?,
            name_en: row.get(5)?,
            drive_on_right: row.get(6)?,
            allow_intersection_names: row.get(7)?,
            default_language: row.get(8)?,
            supported_languages: row.get(9)?,
            geom: row.get(10)?,
        })
    }
}

#[test]
fn test_build_admins_from_geo_parquet_wa_example_data() {
    let division_parquet = "tests/data/wa-divisions.parquet";
    let area_parquet = "tests/data/wa-division-areas.parquet";

    let tmp_dir = tempdir().expect("failed to make tempdir");
    let sqlite_path = tmp_dir.path().join("test_admin.sqlite");

    let admin_config = AdminConfig::default();

    let result = build_admins_from_geo_parquet(
        division_parquet,
        area_parquet,
        &sqlite_path.to_string_lossy(),
        &admin_config,
    );
    assert!(result.is_ok(), "admin building failed: {:?}", result);

    let conn = rusqlite::Connection::open(&sqlite_path).unwrap();
    let mut stmt = conn
        .prepare(
            "SELECT rowid, admin_level, iso_code, parent_admin, name, name_en,
           drive_on_right, allow_intersection_names, default_language, supported_languages, geom
         FROM admins ORDER BY rowid",
        )
        .unwrap();

    let admins: Vec<AdminRow> = stmt
        .query_map([], AdminRow::from_row)
        .unwrap()
        .map(Result::unwrap)
        .collect();

    assert_eq!(admins.len(), 2, "Expected 2 features, got {admins:?}");

    let us = &admins[0];
    assert_eq!(us.admin_level, 2);
    assert_eq!(us.iso_code.as_deref(), Some("US"));
    assert_eq!(us.parent_admin, None);
    assert_eq!(us.name, "United States");
    assert_eq!(us.name_en.as_deref(), Some("United States"),);
    assert_eq!(us.drive_on_right, Some(1));
    assert_eq!(us.allow_intersection_names, Some(0));
    assert_eq!(us.default_language, None);
    assert_eq!(us.supported_languages, None);
    assert!(us.geom.is_some());

    let wa = &admins[1];
    assert_eq!(wa.admin_level, 4);
    assert_eq!(wa.iso_code.as_deref(), Some("WA"));
    assert_eq!(wa.parent_admin, Some(us.rowid));
    assert_eq!(wa.name, "Washington");
    assert_eq!(wa.name_en.as_deref(), Some("Washington"));
    assert_eq!(wa.drive_on_right, Some(1));
    assert_eq!(wa.allow_intersection_names, Some(0));
    assert_eq!(wa.default_language, None);
    assert_eq!(wa.supported_languages, None);
    assert!(wa.geom.is_some(), "geom should be non-null");

    let mut stmt = conn.prepare(
        "SELECT admin_id, iso_code, trunk, trunk_link, track, footway, pedestrian, bridleway, cycleway, path, motorroad FROM admin_access"
    ).unwrap();

    let admin_access: Vec<(
        i64,
        String,
        Option<i64>,
        Option<i64>,
        Option<i64>,
        Option<i64>,
        Option<i64>,
        Option<i64>,
        Option<i64>,
        Option<i64>,
        Option<i64>,
    )> = stmt
        .query_map([], |row| {
            Ok((
                row.get(0)?,  // admin_id
                row.get(1)?,  // iso_code
                row.get(2)?,  // trunk
                row.get(3)?,  // trunk_link
                row.get(4)?,  // track
                row.get(5)?,  // footway
                row.get(6)?,  // pedestrian
                row.get(7)?,  // bridleway
                row.get(8)?,  // cycleway
                row.get(9)?,  // path
                row.get(10)?, // motorroad
            ))
        })
        .unwrap()
        .map(Result::unwrap)
        .collect();

    assert_eq!(
        admin_access.len(),
        1,
        "Expected 1 admin_access row, got {:?}",
        admin_access
    );

    let row = &admin_access[0];
    assert_eq!(row.0, us.rowid);
    assert_eq!(row.1, "US");
    assert_eq!(row.2, None);
    assert_eq!(row.3, None);
    assert_eq!(row.4, None);
    assert_eq!(row.5, None);
    assert_eq!(row.6, Some(262));
    assert_eq!(row.7, Some(262));
    assert_eq!(row.8, Some(262));
    assert_eq!(row.9, Some(774));
    assert_eq!(row.10, None);
}

#[test]
fn test_build_admins_from_geo_parquet_japan_example_data() {
    let division_parquet = "tests/data/tokio-divisions.parquet";
    let area_parquet = "tests/data/tokio-division-areas.parquet";

    let tmp_dir = tempdir().expect("failed to make tempdir");
    let sqlite_path = tmp_dir.path().join("test_admin.sqlite");

    let admin_config = AdminConfig::default();

    let result = build_admins_from_geo_parquet(
        division_parquet,
        area_parquet,
        &sqlite_path.to_string_lossy(),
        &admin_config,
    );
    assert!(result.is_ok(), "admin building failed: {:?}", result);

    let conn = rusqlite::Connection::open(&sqlite_path).unwrap();
    let mut stmt = conn
        .prepare(
            "SELECT rowid, admin_level, iso_code, parent_admin, name, name_en,
           drive_on_right, allow_intersection_names, default_language, supported_languages, geom
         FROM admins ORDER BY rowid",
        )
        .unwrap();

    let admins: Vec<AdminRow> = stmt
        .query_map([], AdminRow::from_row)
        .unwrap()
        .map(Result::unwrap)
        .collect();

    assert_eq!(admins.len(), 2, "Expected 2 features, got {:?}", admins);

    let jp = &admins[0];
    assert_eq!(jp.admin_level, 2);
    assert_eq!(jp.iso_code.as_deref(), Some("JP"));
    assert_eq!(jp.parent_admin, None);
    assert_eq!(jp.name, "日本");
    assert_eq!(jp.name_en.as_deref(), Some("Japan"));
    assert_eq!(jp.drive_on_right, Some(0));
    assert_eq!(jp.allow_intersection_names, Some(1));
    assert_eq!(jp.default_language, None);
    assert_eq!(jp.supported_languages, None);
    assert!(jp.geom.is_some());

    let tokyo = &admins[1];
    assert_eq!(tokyo.admin_level, 4);
    assert_eq!(tokyo.iso_code.as_deref(), Some("13"));
    assert_eq!(tokyo.parent_admin, Some(jp.rowid));
    assert_eq!(tokyo.name, "東京都");
    assert_eq!(tokyo.name_en.as_deref(), Some("Tokyo"));
    assert_eq!(tokyo.drive_on_right, Some(0));
    assert_eq!(tokyo.allow_intersection_names, Some(1));
    assert_eq!(tokyo.default_language, None);
    assert_eq!(tokyo.supported_languages, None);
    assert!(tokyo.geom.is_some());

    let count: i64 = conn
        .query_row("SELECT COUNT(*) FROM admin_access", [], |row| row.get(0))
        .unwrap();
    assert_eq!(count, 0, "Expected no admin_access rows, but found {count}",);
}
