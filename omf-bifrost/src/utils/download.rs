use anyhow::Result;
use duckdb::{Connection, params};
use log::{debug, info};

/// Download Overture Maps transportation data for a specific bounding box
pub fn download_overture_data(
    release_version: &str,
    xmin: f64,
    xmax: f64,
    ymin: f64,
    ymax: f64,
    output_path: &str,
) -> Result<()> {
    // Create an in-memory DuckDB connection
    let conn = Connection::open_in_memory()?;

    // Install and load required extensions
    info!("Installing and loading DuckDB extensions");
    conn.execute("INSTALL spatial", [])?;
    conn.execute("LOAD spatial", [])?;
    conn.execute("INSTALL httpfs", [])?;
    conn.execute("LOAD httpfs", [])?;

    // Format the query with parameter values
    let query = format!(
        "
        COPY (
            SELECT *
            FROM read_parquet('s3://overturemaps-us-west-2/release/{}/theme=transportation/type=*/*', filename=true, hive_partitioning=1)
            WHERE
                bbox.xmin >= {}
                AND bbox.xmax <= {}
                AND bbox.ymin >= {}
                AND bbox.ymax <= {}
        ) TO '{}' (FORMAT PARQUET);
        ",
        release_version, xmin, xmax, ymin, ymax, output_path
    );

    // Log the query being executed
    debug!("Executing DuckDB query:\n{}", query);

    // Execute the query to download and save the data
    info!("Downloading...");
    conn.execute(&query, [])?;

    // Count the number of features downloaded
    let mut stmt = conn.prepare("SELECT COUNT(1) FROM read_parquet(?)")?;
    let count: i64 = stmt.query_row(params![output_path], |row| row.get(0))?;

    info!(
        "Download complete! Found {} transportation features in the specified area.",
        count
    );

    Ok(())
}

/// Download Overture Maps administrative data (divisions/areas) overlapping a bounding box
pub fn download_overture_admins(
    release_version: &str,
    xmin: f64,
    xmax: f64,
    ymin: f64,
    ymax: f64,
    division_output_path: &str,
    area_output_path: &str,
) -> Result<()> {
    let conn = Connection::open_in_memory()?;

    info!("Installing and loading DuckDB extensions");
    conn.execute("INSTALL spatial", [])?;
    conn.execute("LOAD spatial", [])?;
    conn.execute("INSTALL httpfs", [])?;
    conn.execute("LOAD httpfs", [])?;

    let bbox_wkt = format!(
        "POLYGON(({} {}, {} {}, {} {}, {} {}, {} {}))",
        xmin, ymin, xmax, ymin, xmax, ymax, xmin, ymax, xmin, ymin
    );

    let area_query = format!(
        "
        COPY (
            SELECT *
            FROM read_parquet('s3://overturemaps-us-west-2/release/{release_version}/theme=divisions/type=division_area/*', filename=true, hive_partitioning=1)
            WHERE
                bbox.xmin <= {xmax}
                AND bbox.xmax >= {xmin}
                AND bbox.ymin <= {ymax}
                AND bbox.ymax >= {ymin}
                AND ST_Intersects(geometry, ST_GeomFromText('{bbox_wkt}'))
        ) TO '{area_output_path}' (FORMAT PARQUET);
        ",
        release_version = release_version,
        xmax = xmax,
        xmin = xmin,
        ymax = ymax,
        ymin = ymin,
        bbox_wkt = bbox_wkt,
        area_output_path = area_output_path
    );

    info!("Downloading division areas ...");
    debug!("Executing DuckDB query:\n{}", area_query);
    conn.execute(&area_query, [])?;

    let mut stmt = conn.prepare("SELECT COUNT(1) FROM read_parquet(?)")?;
    let area_count: i64 = stmt.query_row(params![area_output_path], |row| row.get(0))?;
    info!(
        "Filtered division_area result: found {} features",
        area_count
    );

    let division_query = format!(
        "
        COPY (
            SELECT *
            FROM read_parquet('s3://overturemaps-us-west-2/release/{release_version}/theme=divisions/type=division/*', filename=true, hive_partitioning=1)
            WHERE id IN (
                SELECT DISTINCT division_id
                FROM read_parquet('{area_output_path}')
            )
        ) TO '{division_output_path}' (FORMAT PARQUET);
        ",
        release_version = release_version,
        area_output_path = area_output_path,
        division_output_path = division_output_path
    );

    info!("Downloading division metadata for matching areas ...");
    debug!("Executing DuckDB query:\n{}", division_query);
    conn.execute(&division_query, [])?;

    let mut stmt = conn.prepare("SELECT COUNT(1) FROM read_parquet(?)")?;
    let division_count: i64 = stmt.query_row(params![division_output_path], |row| row.get(0))?;
    info!(
        "Filtered division result: found {} features",
        division_count
    );
    Ok(())
}
