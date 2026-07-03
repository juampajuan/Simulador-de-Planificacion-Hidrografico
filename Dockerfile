FROM rust:1.96.0

WORKDIR /server

# Instalo gdal y otras dependencias
RUN apt-get update
RUN apt-get install -y libgdal-dev gdal-bin
# RUN apt-get install -y nodejs
RUN rm -rf /var/lib/apt/lists/*

# Agrego target wasm y Trunk para front
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk

COPY . .

# WORKDIR /server/src/client/ui
# RUN npm install

WORKDIR /server/client
RUN trunk build --release --dist dist

WORKDIR /server
RUN cargo build -p server --release

EXPOSE 3000