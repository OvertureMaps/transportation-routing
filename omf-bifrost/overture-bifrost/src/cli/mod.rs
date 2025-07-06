use anyhow::Result;
use clap::{Parser, Subcommand};
use log::{debug, info};
use std::fs;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Verbosity level (-v = info, -vv = debug, -vvv = trace)
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Build Valhalla graph from Overture Maps data
    BuildTiles {
        /// Input GeoParquet file containing Overture Maps transportation data
        /// This should contain segments and connectors that will be converted to Valhalla's graph structure
        #[arg(short, long)]
        input: String,

        /// Directory where the resulting Valhalla graph tiles will be written
        /// The directory structure will match Valhalla's hierarchical tile organization
        #[arg(short, long)]
        output_dir: String,

        /// Path to a JSON configuration file with Valhalla settings
        /// Contains tile hierarchy settings, costing options, and other Valhalla configurations
        #[arg(short, long)]
        config: Option<String>,

        /// Number of parallel threads to use during processing
        /// Defaults to available CPU cores if not specified
        #[arg(short, long)]
        threads: Option<usize>,

        /// JSON configuration string provided directly on the command line
        /// Allows overriding specific configuration options without a separate file
        /// Example: --inline-config '{"bifrost":{"tile_dir":"/custom/path"}}'
        #[arg(long)]
        inline_config: Option<String>,
    },
    /// Convert Overture Maps data to Valhalla binary format
    Convert {
        /// Input GeoParquet file containing Overture Maps transportation data
        /// This should contain the segments and connectors to be converted
        #[arg(short, long)]
        input: String,

        /// Directory where the resulting binary files will be written
        /// Will contain ways.bin, nodes.bin, and way_nodes.bin files
        #[arg(short, long)]
        output_dir: String,

        /// Number of parallel threads to use during conversion
        /// Defaults to available CPU cores if not specified
        #[arg(short, long)]
        threads: Option<usize>,
    },
    /// Build administrative data from Overture Maps data
    BuildAdmins {
        /// Input GeoParquet file containing Overture Maps admin division definitions
        #[arg(short = 'd', long)]
        divisions: String,

        /// Input GeoParquet file with areas for Overture Maps administrative divisions
        /// Should include boundaries and attributes for administrative divisions (e.g., countries, states)
        #[arg(short = 'a', long)]
        division_areas: String,

        /// Directory where the resulting administrative database will be written
        /// Will contain files needed for administrative lookups during routing
        #[arg(short, long)]
        output_dir: String,

        /// Path to a JSON configuration file with admin building settings
        /// Contains settings for administrative hierarchy and boundary processing
        #[arg(short, long)]
        config: Option<String>,
    },
    /// Generate the default admin config for customization
    GenerateAdminConfig {
        /// Output path for config JSON
        #[arg(short, long)]
        output: String,
    },
    /// Download sample Overture Maps transportation data
    Download {
        /// Directory where the downloaded data will be saved
        #[arg(long, default_value = "data")]
        output_dir: String,

        /// Output filename (will be saved as a Parquet file)
        #[arg(long, default_value = "example-data.parquet")]
        output_file: String,

        /// Overture Maps release version
        #[arg(short, long, default_value = "2025-05-21.0")]
        release_version: String,

        /// Bounding box minimum longitude
        #[arg(long, default_value_t = -122.355509)]
        xmin: f64,

        /// Bounding box maximum longitude
        #[arg(long, default_value_t = -122.316885)]
        xmax: f64,

        /// Bounding box minimum latitude
        #[arg(long, default_value_t = 47.610561)]
        ymin: f64,

        /// Bounding box maximum latitude
        #[arg(long, default_value_t = 47.628727)]
        ymax: f64,
    },
    /// Download sample Overture Maps administrative data
    DownloadAdmin {
        /// Directory where the downloaded admin data will be saved
        #[arg(long, default_value = "data")]
        output_dir: String,

        /// Output filename (will be saved as a Parquet file)
        #[arg(long, default_value = "example-divisions.parquet")]
        output_divisions_file: String,

        /// Output filename (will be saved as a Parquet file)
        #[arg(long, default_value = "example-division-areas.parquet")]
        output_division_areas_file: String,

        /// Overture Maps release version
        #[arg(short, long, default_value = "2025-05-21.0")]
        release_version: String,

        /// Bounding box minimum longitude
        #[arg(long, default_value_t = -122.355509)]
        xmin: f64,

        /// Bounding box maximum longitude
        #[arg(long, default_value_t = -122.316885)]
        xmax: f64,

        /// Bounding box minimum latitude
        #[arg(long, default_value_t = 47.610561)]
        ymin: f64,

        /// Bounding box maximum latitude
        #[arg(long, default_value_t = 47.628727)]
        ymax: f64,
    },
}

/// Parse command line arguments
pub fn parse() -> Cli {
    Cli::parse()
}

/// Run the command line interface with pre-parsed arguments
pub fn run_with_args(cli: Cli) -> Result<()> {
    match cli.verbose {
        0 => debug!("Log level: ERROR"),
        1 => debug!("Log level: INFO"),
        2 => debug!("Log level: DEBUG"),
        _ => debug!("Log level: TRACE"),
    }

    match &cli.command {
        Commands::BuildTiles {
            input,
            output_dir,
            config,
            threads,
            inline_config,
        } => {
            info!("Building tiles from Overture Maps data");
            info!("Input: {}", input);
            info!("Output directory: {}", output_dir);

            if let Some(config_path) = config {
                info!("Configuration file: {}", config_path);
            }

            if let Some(thread_count) = threads {
                info!("Number of threads: {}", thread_count);
            }

            if inline_config.is_some() {
                info!("Using inline configuration");
            }

            // TODO: Implement actual tile building logic
            info!("Tile building not yet implemented");
        }
        Commands::Convert {
            input,
            output_dir,
            threads,
        } => {
            info!("Converting Overture Maps data to Valhalla binary format");
            info!("Input: {}", input);
            info!("Output directory: {}", output_dir);

            if let Some(thread_count) = threads {
                info!("Using {} threads", thread_count);
            }

            // TODO: Implement actual conversion logic
            info!("Conversion not yet implemented");
        }
        Commands::BuildAdmins {
            divisions,
            division_areas,
            output_dir,
            config,
        } => {
            info!("Building administrative data from Overture Maps data");
            info!("Input: {}; {}", divisions, division_areas);
            info!("Output directory: {}", output_dir);

            let admin_config = crate::admin::load_admin_config(config.as_deref())?;
            let sqlite_path = format!("{}/admin.sqlite", output_dir);
            crate::admin::build_admins_from_geo_parquet(
                divisions,
                division_areas,
                &sqlite_path,
                &admin_config,
            )?;
            info!("Admin building complete, db at {}", sqlite_path);
        }
        Commands::GenerateAdminConfig { output } => {
            crate::admin::save_default_admin_config(output)?;
            info!("Default admin config written to {}", output);
        }
        Commands::Download {
            output_dir,
            output_file,
            release_version,
            xmin,
            xmax,
            ymin,
            ymax,
        } => {
            info!("Downloading Overture Maps transportation data");
            info!("Release version: {}", release_version);
            info!("Bounding box: ({}, {}) to ({}, {})", xmin, ymin, xmax, ymax);
            info!("Output path: {}/{}", output_dir, output_file);

            // Create output directory if it doesn't exist
            let output_path = Path::new(output_dir).join(output_file);
            if !Path::new(output_dir).exists() {
                fs::create_dir_all(output_dir)?;
            }

            // Use duckdb to download the data
            crate::utils::download::download_overture_data(
                release_version,
                *xmin,
                *xmax,
                *ymin,
                *ymax,
                &output_path.to_string_lossy(),
            )?;

            info!("Download complete! Data saved to {}", output_path.display());
        }
        Commands::DownloadAdmin {
            output_dir,
            output_divisions_file,
            output_division_areas_file,
            release_version,
            xmin,
            xmax,
            ymin,
            ymax,
        } => {
            info!("Downloading Overture Maps admin divisions data");
            info!("Release version: {}", release_version);
            info!("Bounding box: ({}, {}) to ({}, {})", xmin, ymin, xmax, ymax);
            info!("Output path: {}", output_dir);
            info!("Output divisions file: {}", output_divisions_file);
            info!("Output division areas file: {}", output_division_areas_file);

            {
                let output_divisions_path = Path::new(output_dir).join(output_divisions_file);
                let output_division_areas_path =
                    Path::new(output_dir).join(output_division_areas_file);
                if !Path::new(output_dir).exists() {
                    fs::create_dir_all(output_dir)?;
                }
                crate::utils::download::download_overture_admins(
                    release_version,
                    *xmin,
                    *xmax,
                    *ymin,
                    *ymax,
                    &output_divisions_path.to_string_lossy(),
                    &output_division_areas_path.to_string_lossy(),
                )?;
                info!(
                    "Admin downloads complete! Divisions: {}, Areas: {}",
                    output_divisions_path.display(),
                    output_division_areas_path.display()
                );
            }
        }
    }

    Ok(())
}
