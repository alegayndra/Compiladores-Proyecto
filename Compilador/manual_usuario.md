# Manual de usuario
Instrucciones sobre la creación de datos para el lenguaje de compilación _Killer Queen_

Todo el código que se genere tiene que pertenecer a un archivo de terminación `.eo`.

Cuando se haga alusión a una variable (nombre personalizable) mientras se explica, se encontrará dicho texto rodeado de `<` `>`, dichos caracteres *NO* forman parte del código  
Por ejemplo: 
```
entero < variable > ;
```


Ya con la variable seleccionada la misma línea de código se vería así:  
```
entero var1;
```
## Instrucciones

### Estructura

La primera línea de código debe ser
```
programa < nombre > ;
```
Para indicar el nombre del programa.

Si se quieren declarar variables globales eso es lo siguiente a declarar, primero se indica el tipo y luego se indica el nombre.  
Los tipos de datos existentes en el lenguaje son:
- entero
- flotante
- char

Y se pueden crear arreglos de estos también.

Un ejemplo de como se verían la declaración de variables es
```
entero < variable >;
entero < variable >[< tamaño_arreglo >];
flotante < variable >;
char < variable >;
```

Si se quiere crear funciones además de la función _principal()_ eso es lo siguiente.  
Primero se indica el tipo de retorno de la función si es que tiene usando las palabras reservadas _entero_, _flotante_, _char_, _void_.  
Luego se coloca la palabra reservada _funcion_, después un < nombre > y por último los _()_.

Lo anterior visto en código daría un ejemplo como este:
```
void funcion < nombre >()
```

Si se desean agregar parámetros a la función se tiene que indicar el tipo de dato y luego el nombre de la misma.  
Un ejemplo se puede ver así:
```
char funcion < nombre >(flotante < variable_1 >, entero < variable_2 >)
```

Por último todo el segmento de código de una función se tiene que encontrar rodeando entre **{ }**

Un ejemplo con todas las especificaciones de funciones se vería así:
```
void funcion buscar(entero var){
    
}
```

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
