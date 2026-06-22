# Taller de Programacion {Grupo}

## Integrantes

- Julen Gaumard
- Juan Torga
- Felipe Gazcon
- Juan Pablo Dominguez Lucia

## Como usar

A continuacion se detallan los pasos para compilar y ejecutar el programa.

### Limitaciones/Aclaraciones

- Las coordenadas deben ser **proyectadas**
- El area debe ser **rectangular**

### Requerimientos

Se recomienda usar `rustc 1.96.0 (ac68faa20 2026-05-25)`.

*Con otras versiones de rust, algunos crates pueden ser rechazados.*
 
<br/>

Se debe tener instalada la libreria `gdal`.

```bash
sudo apt install libgdal-dev gdal-bin
```

### Ejecucion

Como el server se encarga de servir a la web, con levantarlo es suficiente para tener todo el proyecto funcionando.

```bash
cargo run -p server 
# Automanticamente instala el resto de dependencias/crates y lo ejecuta.
```

<br/>

Si se necesita utilizar el `CLI`, en otra pc, para gestionar docentes.

```bash
cargo run -p cli
```

### Desarrollo

Para el desarrollo, puede ser necesario levantar el client. Para que actualice el front.

#### Requerimientos

Primero, se debe instarlar crates extras para `wasm`.

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk
```

Y para `tailwind` que maneja los estilos.

```bash
# En la segunda
# Con la ruta en /src/client/ui
npm install
```

#### Compilacion

Una vez instalada las dependencias del front se puede inicar su desarrollo ejecutando.

```bash
# En primera terminal:
# En la ruta root/src/client
trunk serve

# En la segunda
# En la ruta root/src/client/ui 
npx tailwindcss -i ./styles.css -o ./tailwind.css --watch      
```

<br/>

Para el servidor, se utiliza el mismo comando que para su comun ejecucion.

```bash
cargo run -p server
```
