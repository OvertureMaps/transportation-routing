use anyhow::Result;
use env_logger::Env;
use log::{error, info};
use overture_bifrost::utils::logging::configure_logging;

fn main() -> Result<()> {
    // Parse CLI arguments first to get verbosity level
    let cli = overture_bifrost::cli::parse();

    // Configure logger based on verbosity
    let log_level = configure_logging(cli.verbose);
    env_logger::Builder::from_env(Env::default().default_filter_or(log_level.to_string())).init();

    info!("Starting omf-bifrost");

    // Run the CLI with the already parsed arguments and match on the result
    match overture_bifrost::cli::run_with_args(cli) {
        Ok(_) => {
            info!("Operation completed successfully");
            Ok(())
        }
        Err(e) => {
            error!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
