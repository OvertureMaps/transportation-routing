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
