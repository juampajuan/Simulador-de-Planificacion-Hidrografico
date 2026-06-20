```bash
# Para agregar el target
rustup target add wasm32-unknown-unknown
cargo install trunk
```

```bash
cargo add gloo-net -p client
```

### Ejecucion

Para la ejecucion del __client__.
```bash
# En primera terminal:
# Con la ruta en /src/client
trunk serve

# En la segunda
# Con la ruta en /src/client/ui
npm install
./node_modules/.bin/tailwindcss -i ./styles.css -o ./tailwind.css
```

Para el __server__:

```bash
cargo run -p server
```

Para correr el main de **simulations**
```bash
# Con esto se puede probar, sin necesidad de levantar el server.
cargo run -p simulations
```

> [!important]
> El archivo `.tif` __debe__ estar la carpeta root.
>
> Cambiar algo para que lo busque en un root/files/geotiff. __En esa carpeta los va a dejar el server.__