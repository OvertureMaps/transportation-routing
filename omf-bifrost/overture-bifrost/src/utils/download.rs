use anyhow::{Context, Result};
use duckdb::{Connection, params};
use log::{debug, info};

/// Default Overture Maps release version
pub const DEFAULT_RELEASE_VERSION: &str = "2025-05-21.0";

pub struct OvertureMapsConfig {
    pub base_url: String,
    pub release_version: String,
}

impl Default for OvertureMapsConfig {
    fn default() -> Self {
        Self {
            base_url: "s3://overturemaps-us-west-2/release".to_string(),
            release_version: DEFAULT_RELEASE_VERSION.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BoundingBox {
    pub xmin: f64,
    pub xmax: f64,
    pub ymin: f64,
    pub ymax: f64,
}

impl BoundingBox {
    pub fn new(xmin: f64, ymin: f64, xmax: f64, ymax: f64) -> Self {
        Self {
            xmin,
            xmax,
            ymin,
            ymax,
        }
    }

    pub fn to_wkt_polygon(&self) -> String {
        format!(
            "POLYGON(({} {}, {} {}, {} {}, {} {}, {} {}))",
            self.xmin,
            self.ymin,
            self.xmax,
            self.ymin,
            self.xmax,
            self.ymax,
            self.xmin,
            self.ymax,
            self.xmin,
            self.ymin
        )
    }
}

pub struct OvertureDuckDB {
    conn: Connection,
}

impl OvertureDuckDB {
    pub fn new() -> Result<Self> {
        let conn = Connection::open_in_memory().context("Failed to create DuckDB connection")?;

        info!("Installing and loading DuckDB extensions");

        conn.execute("INSTALL spatial", [])
            .context("Failed to install spatial extension")?;
        conn.execute("LOAD spatial", [])
            .context("Failed to load spatial extension")?;
        conn.execute("INSTALL httpfs", [])
            .context("Failed to install httpfs extension")?;
        conn.execute("LOAD httpfs", [])
            .context("Failed to load httpfs extension")?;

        Ok(Self { conn })
    }

    pub fn execute_query(&self, query: &str) -> Result<()> {
        debug!("Executing DuckDB query:\n{}", query);
        self.conn
            .execute(query, [])
            .with_context(|| format!("Failed to execute query: {}", query))?;
        Ok(())
    }

    pub fn count_parquet_rows(&self, path: &str) -> Result<i64> {
        let mut stmt = self
            .conn
            .prepare("SELECT COUNT(1) FROM read_parquet(?)")
            .context("Failed to prepare count query")?;
        let count: i64 = stmt
            .query_row(params![path], |row| row.get(0))
            .with_context(|| format!("Failed to count rows in {}", path))?;
        Ok(count)
    }
}

pub struct OvertureMapsQuery {
    config: OvertureMapsConfig,
}

impl OvertureMapsQuery {
    pub fn new(config: OvertureMapsConfig) -> Self {
        Self { config }
    }

    pub fn transportation_query(&self, bbox: &BoundingBox, output_path: &str) -> String {
        format!(
            r#"
            COPY (
                SELECT *
                FROM read_parquet('{base_url}/{version}/theme=transportation/type=*/*', 
                                filename=true, hive_partitioning=1)
                WHERE
                    bbox.xmin >= {xmin}
                    AND bbox.xmax <= {xmax}
                    AND bbox.ymin >= {ymin}
                    AND bbox.ymax <= {ymax}
            ) TO '{output_path}' (FORMAT PARQUET);
            "#,
            base_url = self.config.base_url,
            version = self.config.release_version,
            xmin = bbox.xmin,
            xmax = bbox.xmax,
            ymin = bbox.ymin,
            ymax = bbox.ymax,
            output_path = output_path
        )
    }

    pub fn division_areas_query(&self, bbox: &BoundingBox, output_path: &str) -> String {
        let bbox_wkt = bbox.to_wkt_polygon();
        format!(
            r#"
            COPY (
                SELECT *
                FROM read_parquet('{base_url}/{version}/theme=divisions/type=division_area/*', 
                                filename=true, hive_partitioning=1)
                WHERE
                    bbox.xmin <= {xmax}
                    AND bbox.xmax >= {xmin}
                    AND bbox.ymin <= {ymax}
                    AND bbox.ymax >= {ymin}
                    AND ST_Intersects(geometry, ST_GeomFromText('{bbox_wkt}'))
            ) TO '{output_path}' (FORMAT PARQUET);
            "#,
            base_url = self.config.base_url,
            version = self.config.release_version,
            xmax = bbox.xmax,
            xmin = bbox.xmin,
            ymax = bbox.ymax,
            ymin = bbox.ymin,
            bbox_wkt = bbox_wkt,
            output_path = output_path
        )
    }

    pub fn divisions_query(&self, area_output_path: &str, division_output_path: &str) -> String {
        format!(
            r#"
            COPY (
                SELECT *
                FROM read_parquet('{base_url}/{version}/theme=divisions/type=division/*', 
                                filename=true, hive_partitioning=1)
                WHERE id IN (
                    SELECT DISTINCT division_id
                    FROM read_parquet('{area_output_path}')
                )
            ) TO '{division_output_path}' (FORMAT PARQUET);
            "#,
            base_url = self.config.base_url,
            version = self.config.release_version,
            area_output_path = area_output_path,
            division_output_path = division_output_path
        )
    }
}
pub fn download_overture_data(
    release_version: &str,
    xmin: f64,
    xmax: f64,
    ymin: f64,
    ymax: f64,
    output_path: &str,
) -> Result<()> {
    let config = OvertureMapsConfig {
        release_version: release_version.to_string(),
        ..Default::default()
    };

    let bbox = BoundingBox::new(xmin, ymin, xmax, ymax);
    let db = OvertureDuckDB::new()?;
    let query_builder = OvertureMapsQuery::new(config);

    info!("Downloading transportation data...");
    let query = query_builder.transportation_query(&bbox, output_path);
    db.execute_query(&query)?;

    let count = db.count_parquet_rows(output_path)?;
    info!(
        "Download complete! Found {} transportation features in the specified area.",
        count
    );

    Ok(())
}

pub fn download_overture_admins(
    release_version: &str,
    xmin: f64,
    xmax: f64,
    ymin: f64,
    ymax: f64,
    division_output_path: &str,
    area_output_path: &str,
) -> Result<()> {
    let config = OvertureMapsConfig {
        release_version: release_version.to_string(),
        ..Default::default()
    };

    let bbox = BoundingBox::new(xmin, ymin, xmax, ymax);
    let db = OvertureDuckDB::new()?;
    let query_builder = OvertureMapsQuery::new(config);

    // Download division areas first
    info!("Downloading division areas...");
    let areas_query = query_builder.division_areas_query(&bbox, area_output_path);
    db.execute_query(&areas_query)?;

    let area_count = db.count_parquet_rows(area_output_path)?;
    info!("Found {} division area features", area_count);

    // Download corresponding divisions
    info!("Downloading division metadata for matching areas...");
    let divisions_query = query_builder.divisions_query(area_output_path, division_output_path);
    db.execute_query(&divisions_query)?;

    let division_count = db.count_parquet_rows(division_output_path)?;
    info!("Found {} division features", division_count);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bounding_box_wkt() {
        let bbox = BoundingBox::new(-122.4, 47.6, -122.3, 47.7);
        let wkt = bbox.to_wkt_polygon();
        assert!(wkt.contains("POLYGON"));
        assert!(wkt.contains("-122.4"));
        assert!(wkt.contains("47.6"));
    }

    #[test]
    fn test_overture_config_default() {
        let config = OvertureMapsConfig::default();
        assert!(config.base_url.contains("overturemaps"));
        assert!(!config.release_version.is_empty());
    }
}
