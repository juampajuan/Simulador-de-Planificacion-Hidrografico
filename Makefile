# =====================================
# Configuración
# =====================================

IMAGE_NAME := taller-tp5
CONTAINER_NAME := taller-tp5

SERVER_BIN := ./target/release/server

.PHONY: help deps front back build run cli clean \
        docker-build docker-run docker-stop docker-rm \
        docker-shell docker-logs rebuild dev front-dev

# =====================================
# Ayuda
# =====================================

help:
	@echo ""
	@echo "Comandos disponibles:"
	@echo "  make deps           Instala dependencias de Rust (wasm + trunk)"
	@echo "  make front          Compila el frontend"
	@echo "  make back           Compila el backend"
	@echo "  make build          Compila frontend + backend"
	@echo "  make run            Ejecuta el servidor (compilado)"
	@echo "  make cli            Ejecuta el CLI"
	@echo "  make clean          Limpia archivos compilados"
	@echo "  make rebuild        Clean + Build"
	@echo ""
	@echo "Desarrollo:"
	@echo "  make dev            cargo run -p server"
	@echo "  make front-dev      trunk serve"
	@echo ""

# =====================================
# Dependencias
# =====================================

deps:
	sudo apt update
	sudo apt install -y \
		build-essential \
		pkg-config \
		libclang-dev \
		clang \
		libgdal-dev \
		gdal-bin

	rustup target add wasm32-unknown-unknown
	cargo install trunk

# =====================================
# Compilación
# =====================================

front:
	cd client && trunk build --release --dist dist

back:
	cargo build -p server --release

build: front back

rebuild: clean build

clean:
	cargo clean

# =====================================
# Ejecución
# =====================================

run:
	$(SERVER_BIN)

cli:
	cargo run -p cli

dev:
	cargo run -p server

front-dev:
	cd client && trunk serve