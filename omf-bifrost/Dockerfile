FROM debian:bullseye-slim AS base
RUN apt-get update && apt-get install -y curl unzip libsqlite3-dev libsqlite3-mod-spatialite && rm -rf /var/lib/apt/lists/*

RUN ARCH=$(dpkg --print-architecture) && \
    if [ "$ARCH" = "amd64" ] || [ "$ARCH" = "arm64" ]; then \
        DUCKDB_URL="https://github.com/duckdb/duckdb/releases/download/v1.3.0/libduckdb-linux-${ARCH}.zip"; \
    else \
        echo "Unsupported architecture: $ARCH"; exit 1; \
    fi && \
    curl -L -o libduckdb.zip "$DUCKDB_URL" && \
    unzip libduckdb.zip -d duckdb && \
    cp duckdb/libduckdb.so /usr/local/lib/ && \
    ldconfig && \
    rm -r duckdb libduckdb.zip

FROM base AS builder
RUN apt-get update && apt-get install -y build-essential && rm -rf /var/lib/apt/lists/*
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
WORKDIR /app
COPY . /app
RUN cargo build --release

FROM base AS runtime
WORKDIR /run
COPY --from=builder /app/target/release/omf-bifrost /usr/local/bin/omf-bifrost
ENTRYPOINT ["/usr/local/bin/omf-bifrost"]
