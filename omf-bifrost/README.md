# OMF-Bifrost

A tool for converting [Overture Maps Foundation](https://overturemaps.org/) data to [Valhalla](https://github.com/valhalla/valhalla) routing engine format.

## Overview

OMF-Bifrost bridges the gap between Overture Maps Foundation's transportation data and the Valhalla routing engine. It provides a set of tools to convert Overture's GeoParquet data into formats that Valhalla can understand and process.

Named after the rainbow bridge in Norse mythology that connects Midgard (Earth) to Asgard (realm of the gods), Bifrost symbolizes the connection between two powerful systems: Overture's comprehensive global mapping data and Valhalla's efficient routing capabilities.

## Features

- Convert Overture Maps transportation data to Valhalla's binary format
- Build Valhalla graph tiles directly from Overture data
- Generate administrative boundary information for Valhalla from Overture Divisions data
- Configurable processing with multi-threading support

## Installation

### Prerequisites

- Rust toolchain (1.70.0 or newer)
- Cargo package manager

### System Dependencies

#### macOS
Install the required dependencies using [Homebrew](https://brew.sh):

```bash
brew install duckdb sqlite libspatialite
```

**Important**: After installing dependencies, you need to configure library paths for the build system. Create a `.cargo/config.toml` file in the project root:

```toml
[env]
DUCKDB_LIB_DIR = "/opt/homebrew/lib"
SQLITE3_LIB_DIR = "/opt/homebrew/opt/sqlite/lib"
DYLD_LIBRARY_PATH="/opt/homebrew/lib:$DYLD_LIBRARY_PATH"

[target.aarch64-apple-darwin]
rustflags = ["-L", "/opt/homebrew/lib"]
```

**Why these settings are needed:**
- **`-L` flag**: Tells the Rust linker where to find libraries during compilation (compile-time)
- **`DYLD_LIBRARY_PATH`**: Tells macOS's dynamic linker where to find shared libraries when running the program (runtime)
- **Library directories**: Homebrew installs libraries in `/opt/homebrew/lib`, which isn't in the default system search paths

#### Linux
Install the required dependencies using your package manager. For example, on Ubuntu:

```bash
sudo apt-get install libsqlite3-dev libsqlite3-mod-spatialite
```

Follow the [DuckDB installation guide](https://duckdb.org/docs/installation) for your platform.

### Building from Source

```bash
# Clone the repository
git clone https://github.com/OvertureMaps/transportation-routing.git
cd transportation-routing/omf-bifrost

# Build the project
cargo build --release

# The binary will be available at target/release/omf-bifrost
```

## Usage

OMF-Bifrost provides three main commands:

### Build Tiles

Convert Overture Maps data directly to Valhalla graph tiles:

```bash
omf-bifrost build-tiles --input overture-transportation.parquet --output-dir valhalla_tiles
```

### Convert

Convert Overture Maps data to Valhalla's binary format:

```bash
omf-bifrost convert --input overture-transportation.parquet --output-dir valhalla_binary
```

### Building Administrative Boundaries

The `build-admins` command processes Overture Divisions data into the format required by Valhalla:

```bash
omf-bifrost build-admins \
  --divisions overture-divisions.parquet \
  --division-areas overture-division-areas.parquet \
  --output-dir valhalla_admin_boundaries
```

This creates a SQLite database ready for use by Valhalla.

#### Customizing Administrative Boundary Processing

By default, `build-admins` uses built-in settings. For more control—such as overriding access rules—use a configuration file. Start by generating the default config:

```bash
omf-bifrost generate-admin-config --output admin-config.json
```

Edit `admin-config.json` as needed, then supply it back to `build-admins`:

```bash
omf-bifrost build-admins \
  --config admin-config.json \
  --divisions overture-divisions.parquet \
  --division-areas overture-division-areas.parquet \
  --output-dir valhalla_admins
```

#### Example: Using Sample Data

Try building the administrative boundaries table from provided sample datasets:

```bash
omf-bifrost build-admins \
  --divisions tests/data/wa-divisions.parquet \
  --division-areas tests/data/wa-division-areas.parquet \
  --output-dir examples/
```

This creates `examples/admins.sqlite`, which you can inspect or use further for building tiles.

### Additional Options

- Use `-v`, `-vv`, or `-vvv` for increasing verbosity levels
- Specify `--threads` to control parallel processing
- Provide custom configuration with `--config` or `--inline-config`

For detailed help on each command:

```bash
omf-bifrost --help
omf-bifrost build-tiles --help
omf-bifrost convert --help
omf-bifrost build-admins --help
```

## Development

See [DEVELOP.md](DEVELOP.md) for information on contributing to the project, development workflows, and build scripts.

## License

This project is licensed under the [MIT License](LICENSE).

## Acknowledgments

- [Overture Maps Foundation](https://overturemaps.org/) for providing the open map data
- [Valhalla](https://github.com/valhalla/valhalla) routing engine project
