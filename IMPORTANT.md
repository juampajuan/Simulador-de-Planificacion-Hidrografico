# Taller de Programacion {Grupo}

## Integrantes

- Julen Gaumard
- Juan Torga
- Felipe Gazcon
- Juan Pablo Dominguez Lucia

## Como usar

A continuacion se detallan los pasos para compilar y ejecutar el programa.

### Version de Rust

Se recomienda usar `rustc 1.96.0 (ac68faa20 2026-05-25)`.

Con otras versiones de rust algunos crates pueden ser rechazados.

### Compilacion

Se debe tener instalada la libreria gdal:

```bash
sudo apt install libgdal-dev gdal-bin
```

Para el **client**, agregar el target de wasm e instalar trunk:

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk
```

### Limitaciones/Aclaraciones

- Las coordenadas deben ser proyectadas
- El area debe ser rectangular

> [!important]
> El archivo `.tif` __debe__ estar la carpeta root.
>
> Cambiar algo para que lo busque en un root/files/geotiff. __En esa carpeta los va a dejar el server.__

### Como correr

Para la ejecucion del **client**:

```bash
# En primera terminal:
# Con la ruta en /src/client
trunk serve

# En la segunda
# Con la ruta en /src/client/ui
npm install
./node_modules/.bin/tailwindcss -i ./styles.css -o ./tailwind.css
```

Para el **server**:

```bash
cargo run -p server
```