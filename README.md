# Compiladores-Proyecto
Repo para el proyecto final de la clase de Diseño de Compiladores

## Instrucciones 

Para correr el programa, se necesita tener instalado el ambiente de desarrollo rust. Una vez instalado, se corre `cargo build` dentro de la carpeta para instalar las dependencias y `cargo run nombre_archivo` para correr el programa, donde `nombre_archivo` es el nombre del archivo con el código a compilar. Para correr las pruebas unitarias se corre `cargo test -- --test-threads=1`.

También se tiene que crear un directorio llamado `cuadruplos` para que se pueda generar el archivo de salida.

## Código de ejemplo

Copia y pega este código de ejemplo en un archivo de terminación `.eo` para poder correr el programa.

```
programa idPrograma;

void funcion func (entero var) {
	entero i;
	i = 10;
	char j;
	lee(j);
	regresa 10 + i;
}

entero num;
char id;
flotante promedio;

clase Estudiante {
  char nombre[10];
  entero edad;
  flotante promedio;
  entero conseguirEdad() {
    regresa edad;
  }
};

principal() {
  num = 10 * 2;
  promedio = 10.1;
  %% id = \"a\"; %%
  %% esto es un comentario %%
  lee(i);
  escribe(10);
  escribe(\"aaa\");
}
```

### Entrega 7
- Implementación y uso de tablas de variables, funciones y clases.
- Generación de cuadruplos de expresiones aritmeticas, relacionales y lógicas.
- Modificación gramática funcion para permitir varios returns.