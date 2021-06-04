# Killer Queen
Repo para el proyecto final de la clase de Diseño de Compiladores.

Compilador desarrollado en _rust_, con las librerías de _nom_ y _lazy static_.

Máquina virtual desarrollada en _python_.

[Link al repo](https://github.com/alegayndra/KillerQueen)

## Instrucciones

### Correr programa

Para correr el compilador y la máquina virtual, se necesita tener instalado el ambiente de desarrollo rust y de python. Una vez instalados, se siguen las siguientes instrucciones dentro de la carpeta principal.

Para correr el compilador se corre el siguiente comando:

```bash
$ cargo run nombre_archivo
```

Donde `nombre_archivo` es el nombre del archivo con el código a compilar sin la terminación de `.eo`. Por ejemplo, para compilar el acrhivo `sumas_y_restas.eo`, se debe correr `cargo run sumas_y_restas`.

También se tiene que crear un directorio llamado `cuadruplos` para que se pueda generar el archivo de sálida.

Una vez generado el archivo de sálida con el código intermedio, para poder ejectutarlo, se corre el siguiente comando para correr la máquina virtual:

```bash
linux: 
$ python3 Maquina_Virtual/main.py

windows:
$ python Maquina_Virtual/main.py
```

### Pruebas

#### Pruebas unitarias
Para correr las pruebas unitarias dentro de Rust, se corre el siguiente comando:

```bash
cargo test -- --test-threads=1
```

#### Pruebas de compilador
Para correr los diferentes archivos de prueba, se corre `cargo run Pruebas/archivo`, donde `archivo` es el nombre del archivo deseado a correr. Luego, para correr la maquina virtual, se corre `python3 Maquina_Virtual/main.py`.

### Generar documentación

Para generar y abrir la documentación corre `cargo doc --lib --open`.
