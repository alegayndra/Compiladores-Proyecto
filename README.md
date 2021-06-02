# Killer Queen
Repo para el proyecto final de la clase de Diseño de Compiladores.

Compilador desarrollado en _rust_, con las librerías de _nom_ y _lazy static_.

Máquina virtual desarrollada en _python_.

## Instrucciones

### Correr programa

Para correr el programa, se necesita tener instalado el ambiente de desarrollo rust. Una vez instalado, se corre `cargo build` dentro de la carpeta para instalar las dependencias y `cargo run nombre_archivo` para correr el programa, donde `nombre_archivo` es el nombre del archivo con el código a compilar. EL nombre del archivo debe ser ingresado sin el `.eo`. Por ejemplo, para compilar el acrhivo `sumas_y_restas.eo`, se debe correr `cargo run sumas_y_restas`.

También se tiene que crear un directorio llamado `cuadruplos` para que se pueda generar el archivo de salida.

### Pruebas

#### Pruebas unitarias
Para correr las pruebas unitarias dentro de Rust, se corre `cargo test -- --test-threads=1`.

#### Pruebas de compilador
Para correr los diferentes archivos de prueba, se corre `cargo run Pruebas/archivo`, donde `archivo` es el nombre del archivo deseado a correr. Luego, para correr la maquina virtual, se corre `python3 Maquina_Virtual/main.py`.

### Generar documentación

Para generar y abrir la documentación corre `cargo doc --lib --open`.
