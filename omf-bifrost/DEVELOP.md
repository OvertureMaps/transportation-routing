# Development Guide for OMF-Bifrost

This document provides information for developers who want to contribute to or build upon the OMF-Bifrost project.

## Development Environment Setup

### Prerequisites

- Rust toolchain (1.70.0 or newer)
- Cargo package manager
- Git

### Optional Tools

- [Just](https://github.com/casey/just) - Command runner for development tasks
- [cargo-watch](https://github.com/watchexec/cargo-watch) - For auto-rebuilding on file changes

### Getting Started

1. Clone the repository:
   ```bash
   git clone https://github.com/OvertureMaps/transportation-routing.git
   cd transportation-routing/omf-bifrost
   ```

2. Build the project:
   ```bash
   cargo build
   ```

3. Run tests:
   ```bash
   cargo test
   ```

## Project Structure

```
omf-bifrost/
├── src/
│   ├── cli/           # Command-line interface code
│   ├── core/          # Core conversion logic
│   ├── io/            # Input/output handling
│   ├── utils/         # Utility functions
│   └── main.rs        # Application entry point
├── tests/             # Integration tests
├── examples/          # Example usage
├── justfile           # Build script tasks (if using Just)
└── Cargo.toml         # Project dependencies and metadata
```

## Build Scripts

We provide two options for running development tasks:

### Option 1: Using Just (Recommended)

[Just](https://github.com/casey/just) is a handy command runner that simplifies common development tasks.

1. Install Just:
   ```bash
   cargo install just
   ```

2. Available commands:
   ```bash
   just                # Show available commands
   just build          # Build the project
   just test           # Run tests
   just download-data  # Download sample Overture data for testing
   just build-tiles    # Build tiles from sample data
   just convert        # Convert sample data to Valhalla binary format
   just build-admins   # Build admin data from sample data
   just all            # Run all processing tasks
   ```

### Option 2: Using Standard Cargo Commands

If you prefer not to install Just, you can use standard Cargo commands:

```bash
# Build the project
cargo build

# Run tests
cargo test

# Download sample data
cargo run --features download -- -v download

# Run the application
cargo run -- build-tiles --input data/example-data.parquet --output-dir output/tiles
cargo run -- convert --input data/example-data.parquet --output-dir output/binary
cargo run -- build-admins --input data/example-data.parquet --output-dir output/admins
```

## Code Quality

We maintain high code quality standards through consistent formatting and static analysis.

### Code Formatting

We use `rustfmt` to ensure consistent code formatting across the project:

- Configuration is defined in `rustfmt.toml` at the project root
- Format code with `just fmt` or `cargo fmt`
- Check formatting without modifying files with `just fmt-check`
- Format before building with `just build-fmt`

Our `rustfmt.toml` configures:
- Rust 2024 edition formatting rules
- 100 character line width
- 4-space indentation
- Unix-style line endings
- Field initialization shorthand
- Try operator shorthand (`?`)
- Organized imports by module

### Static Analysis with Clippy

[Clippy](https://github.com/rust-lang/rust-clippy) is the official Rust linter that helps catch common mistakes and improve code quality:

- Run Clippy with `just lint` or `cargo clippy -- -D warnings`
- Apply automatic fixes with `just lint-fix` or `cargo clippy --fix -- -D warnings`
- Run both formatting and linting checks with `just check`

Clippy helps identify:
- Correctness issues (logical errors)
- Style inconsistencies (non-idiomatic code)
- Complexity problems (overly complex code)
- Performance issues (inefficient code)
- Cargo configuration problems

### Additional Recommended Tools

For more comprehensive code quality checks, consider these tools:

1. **cargo-audit**: Checks dependencies for security vulnerabilities
   ```bash
   cargo install cargo-audit
   cargo audit
   ```

2. **cargo-deny**: Enforces license compliance and dependency policies
   ```bash
   cargo install cargo-deny
   cargo deny check
   ```

3. **cargo-outdated**: Identifies outdated dependencies
   ```bash
   cargo install cargo-outdated
   cargo outdated
   ```

4. **cargo-udeps**: Finds unused dependencies
   ```bash
   cargo install cargo-udeps
   cargo +nightly udeps
   ```

### IDE Integration

#### VSCode
1. Install the "rust-analyzer" extension
2. Add to settings.json:
```json
{
    "rust-analyzer.checkOnSave.command": "clippy",
    "rust-analyzer.checkOnSave.allTargets": true,
    "editor.formatOnSave": true,
    "[rust]": {
        "editor.defaultFormatter": "rust-lang.rust-analyzer"
    }
}
```

#### IntelliJ/RustRover
1. Go to Settings → Languages & Frameworks → Rust → External Linters
2. Enable Clippy with the desired options
3. Go to Settings → Languages & Frameworks → Rust → Rustfmt
4. Check "Run rustfmt on Save"

## Development Workflow

1. Create a new branch for your feature or bugfix:
   ```bash
   git checkout -b feature-name
   ```

2. Make your changes and write tests

3. Ensure all tests pass:
   ```bash
   cargo test
   ```

4. Check code quality:
   ```bash
   just check  # Runs both formatting and linting checks
   ```

5. Fix any issues:
   ```bash
   just fmt    # Format code
   just lint-fix  # Fix linting issues where possible
   ```

6. Commit your changes with a descriptive message

7. Push your branch and create a pull request

## Working with Overture Maps Data

### Data Format

Overture Maps data is provided in GeoParquet format. The transportation schema includes:

- Segments: Road segments with properties like road class, surface type, etc.
- Connectors: Connections between segments, including turn restrictions
- Metadata: Additional information about the transportation network

### Sample Data

For development and testing, you can use sample data:

```bash
# Using Just
just download-data

# Or manually
cargo run --features download -- -v download
```

## Valhalla Integration

OMF-Bifrost converts Overture data to formats compatible with Valhalla:

1. **Binary Format**: Intermediate files (ways.bin, nodes.bin, etc.) used by Valhalla's mjolnir
2. **Graph Tiles**: Final tile format used by Valhalla for routing
3. **Admin Data**: Administrative boundary information for region identification

Refer to the [Valhalla documentation](https://github.com/valhalla/valhalla/blob/master/docs/mjolnir/getting_started.md) for more details on these formats.

## Logging

The application uses the `log` crate with `env_logger` for logging. Control verbosity with:

- No flag: ERROR level
- `-v`: INFO level
- `-vv`: DEBUG level
- `-vvv`: TRACE level

You can also set the `RUST_LOG` environment variable for more control:

```bash
RUST_LOG=debug cargo run -- build-tiles --input data/example-data.parquet --output-dir output/tiles
```

## Performance Considerations

- Use the `--threads` parameter to control parallel processing
- For large datasets, ensure sufficient memory is available
- Consider processing data in batches if working with global-scale data

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch
3. Commit your changes
4. Push to the branch
5. Open a Pull Request
