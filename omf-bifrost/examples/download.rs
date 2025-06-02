use anyhow::Result;
use duckdb::{params, Connection};
use std::fs;
use std::path::Path;

fn download_sample_data() -> Result<()> {
    let release_version = "2025-05-21.0";
    
    // Seattle South Lake Union area bbox coordinates
    let bbox_xmin = -122.342334;
    let bbox_xmax = -122.334523;
    let bbox_ymin = 47.618546;
    let bbox_ymax = 47.624533;

    println!("Starting sample data download script");

    // Create output directory if it doesn't exist
    let output_dir = Path::new("data");
    if !output_dir.exists() {
        fs::create_dir_all(output_dir)?;
    }
    let output_path_joined = output_dir.join("example-data.parquet");
    let output_path = output_path_joined.to_str().unwrap();
    
    let conn = Connection::open_in_memory()?;
    
    // Install and load extensions
    conn.execute("INSTALL spatial", [])?;
    conn.execute("LOAD spatial", [])?;
    conn.execute("INSTALL httpfs", [])?;
    conn.execute("LOAD httpfs", [])?;
    
    println!("Downloading Overture Maps transportation data...");
    
    // Format the query with actual parameter values
    let query = format!(
        "
        COPY (
            SELECT * 
            FROM read_parquet('s3://overturemaps-us-west-2/release/{}/theme=transportation/type=*/*', filename=true, hive_partitioning=1)
            WHERE
                bbox.xmin BETWEEN {} AND {}
                AND bbox.ymin BETWEEN {} AND {}
        ) TO '{}' (FORMAT PARQUET);
        ",
        release_version, bbox_xmin, bbox_xmax, bbox_ymin, bbox_ymax, output_path
    );
    
    // Print the formatted query
    println!("Executing query:\n{}", query);
    
    // Execute the original query with parameters
    conn.execute(&query, [])?;
    
    // Count the number of features downloaded
    let mut stmt = conn.prepare(
        "SELECT COUNT(1) FROM read_parquet(?)"
    )?;
    
    let count: i64 = stmt.query_row(params![output_path], |row| row.get(0))?;
    
    println!("Download complete! Found {} transportation features in the specified area.", count);
    println!("Data saved to {}", output_path);
    return Ok(());
}

fn main() -> Result<()> {
    download_sample_data()?;
    Ok(())
}
