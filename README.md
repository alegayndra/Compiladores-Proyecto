# Killer Queen
Repo para el proyecto final de la clase de Diseño de Compiladores

## Instrucciones

### Correr programa

Para correr el programa, se necesita tener instalado el ambiente de desarrollo rust. Una vez instalado, se corre `cargo build` dentro de la carpeta para instalar las dependencias y `cargo run nombre_archivo` para correr el programa, donde `nombre_archivo` es el nombre del archivo con el código a compilar. EL nombre del archivo debe ser ingresado sin el `.eo`. Por ejemplo, para compilar el acrhivo `sumas_y_restas.eo`, se debe correr `cargo run sumas_y_restas`.

También se tiene que crear un directorio llamado `cuadruplos` para que se pueda generar el archivo de salida.

### Correr pruebas
Para correr las pruebas unitarias se corre `cargo test -- --test-threads=1`.

## Código de ejemplo

Copia y pega este código de ejemplo en un archivo de terminación `.eo` para poder correr el programa.

```
programa idPrograma;

entero i;
char letra;
flotante promedio;

principal() {
  %% asignaciones %%
  i = 20;
  promedio = 10.1;
  letra = "2";

  %% imprimir valores a consola %%
  escribe("i", i);
  escribe("promedio", promedio);
  escribe("letra", letra);

  %% lectura de valores %%
  escribe("i");
  lee(i);

  escribe("promedio");
  lee(promedio);

  escribe("letra");
  lee(letra);

  %% deciciones %%
  si (i > 10) {
    escribe("hola");
  } sino {
    escribe("adios");
  }


  %% ciclos %%
  i = 20;
  mientras (i > 10) {
    escribe(i);
    i = i - 1;
  }

  desde i = 10 hasta 20 {
    escribe(i);
  }
}
```

### Entrega 7
- Implementación y uso de tablas de variables, funciones y clases.
- Generación de cuadruplos de expresiones aritmeticas, relacionales y lógicas.
- Modificación gramática funcion para permitir varios returns.