use anyhow::Result;
use clap::{Parser, Subcommand};
#[cfg(not(feature = "download"))]
use log::error;
use log::{debug, info};
#[cfg(feature = "download")]
use std::fs;
#[cfg(feature = "download")]
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
        /// Input GeoParquet file containing Overture Maps administrative boundary data
        /// Should include polygons defining administrative areas with their hierarchy
        #[arg(short, long)]
        input: String,

        /// Directory where the resulting administrative database will be written
        /// Will contain files needed for administrative lookups during routing
        #[arg(short, long)]
        output_dir: String,

        /// Path to a JSON configuration file with admin building settings
        /// Contains settings for administrative hierarchy and boundary processing
        #[arg(short, long)]
        config: Option<String>,
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
            input,
            output_dir,
            config,
        } => {
            info!("Building administrative data from Overture Maps data");
            info!("Input: {}", input);
            info!("Output directory: {}", output_dir);

            if let Some(config_path) = config {
                info!("Using configuration file: {}", config_path);
            }

            // TODO: Implement actual admin building logic
            info!("Admin building not yet implemented");
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

            #[cfg(feature = "download")]
            {
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

            #[cfg(not(feature = "download"))]
            {
                error!("Download feature is not enabled in this build.");
                error!("To use this feature, rebuild with: cargo build --features download");
            }
        }
    }

    Ok(())
}
