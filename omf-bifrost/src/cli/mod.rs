use anyhow::Result;
use clap::{Parser, Subcommand};
use log::{debug, info};

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

            if let Some(_) = inline_config {
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
    }

    Ok(())
}
