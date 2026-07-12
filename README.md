# Taller de Programacion 5

### Integrantes

- Julen Gaumard
- Juan Torga
- Felipe Gazcon
- Juan Pablo Dominguez Lucia

### Limitaciones/Aclaraciones

- Las coordenadas deben ser **proyectadas**
- El area debe ser **rectangular**

<br/>

# Ejecución

A continuacion se detallan las 2 formas esperables para ejecutar el servidor.

## 1. Docker

*Se puede ejecutar el proyecto bajo un contenedor de docker.*

El `compose.yaml` gestionará la descarga, compilacion y ejecucion del proyecto.

```bash
# Dentro de la carpeta del proyecto
docker compose up
```

> [!important]
> Esto lo va a inicializar con la configuracion establecida en `config.toml`. 
>
> __Se recomienda cambiar ANTES la `ADMIN_PASS`.__

<br/>

Para realizar interactuar con el `CLI`, hay 2 opciones.

1. Desde docker

    Desde una terminal en el mismo dispositivo que ejecuta __Docker__.

    ```bash
    docker attach <id_container>
    ```

2. Desde otro dispositivo

    Compilando y ejecutando el `CLI` de forma manual como se explica [debajo](###Ejecución).


## 2. Manual

*Se puede compilar y ejecutar el proyecto de forma manual.*

### Requerimientos

* Se recomienda usar `rustc 1.96.0 (ac68faa20 2026-05-25)`.

    *Con otras versiones de rust, algunos crates pueden ser rechazados.*
  
* Se debe tener instalada la libreria `gdal`.

    ```bash
    sudo apt install libgdal-dev gdal-bin
    ```

* Por ultimos se instalan dependencias para compilar el front `wasm`.

    ```bash
    rustup target add wasm32-unknown-unknown
    cargo install trunk
    ```


### Compilación

*Se pueden compilar en distintas terminales y al mismo tiempo.*

Para compilar el front:

```bash
# En la ruta /src/client
trunk build --release --dist dist
```

Para compilar el server:

```bash
cargo build -p server --release
# Automaticamente instala el resto de dependencias/crates.
# Deja el binario compilado en /target/release/
```

### Ejecución

Como el `server` tambien entrega el front, con ejecutarlo ya tendremos toda el proyecto en funcionamiento.

```bash
./target/release/server 
# Lo ejecuta.
```

Si se necesita utilizar el `CLI`, para gestionar docentes desde otro dispositivo.

```bash
cargo run -p cli
```

## Desarrollo

Para el desarrollo, se agregan unas dependencias para compilar cambios de estilo en el `front`.

Se debe tener `node` instalado:

```bash
sudo apt install nodejs npm -y
```

Y luego:

```bash
# En /src/client/ui
npm install
```

1. Server/Simulation

    Para ejecutar el servidor en desarrollo, se puede usar: 

    ```bash
    cargo run -p server
    # Compila y ejecuta con 1 solo comando.
    ```

    *Tambien se puede hacer por separado, como se mostró con anterioridad.*

2. Front

    Una vez instaladas las dependencias del `front` se puede iniciar su desarrollo ejecutando.

    ```bash
    # En primera terminal:
    # En la ruta root/src/client
    trunk serve

    # En la segunda
    # En la ruta root/src/client/ui 
    npx tailwindcss -i ./styles.css -o ./tailwind.css --watch      
    ```

    Esto otorga `hot reload` antes los cambios, para hacer el desarrollo mas ameno.

> [!warning]
> *Será necesario desabilitar `CORS` del server, para que se acepten las requests.*

## Makefile

Se ofrece un `Makefile` para facilitar la compilación y la ejecución.

El mismo posee los siguientes comandos:

```bash
# Muestra los comandos disponibles.
make help

# Instala dependencias (GDAL, Trunk, etc.)
make deps       

# Compilar todo (Front + Back)
make build      

# Ejecutar el servidor (compilado)
make run        

# Desarrollo de server/simulation
make dev

# Limpiar la compilación
make clean

# Limpiar y volver a compilar
make rebuild
```