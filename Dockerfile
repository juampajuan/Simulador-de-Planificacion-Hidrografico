# ==========================
# Etapa de compilación
# ==========================
FROM rust:1.96.0 AS builder

WORKDIR /server

# Dependencias necesarias para compilar
RUN apt-get update && \
    apt-get install -y \
        libgdal-dev \
        gdal-bin && \
    rm -rf /var/lib/apt/lists/*

# Agrego target wasm y Trunk para front
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk

COPY . .

# Compilo frontend
WORKDIR /server/client
RUN trunk build --release --dist dist

# Compilo backend
WORKDIR /server
RUN cargo build -p server --release


# ==========================
# Imagen final
# ==========================
FROM debian:trixie-slim

WORKDIR /server

# Solo lo necesario para ejecutar
RUN apt-get update && \
    apt-get install -y \
        libgdal-dev \
        gdal-bin && \
    rm -rf /var/lib/apt/lists/*

# Copio el binario
COPY --from=builder /server/target/release/server ./server

# Copio el config file.
COPY --from=builder /server/config.toml ./config.toml

# Copio el frontend generado
COPY --from=builder /server/client/dist ./client/dist