# Manual de usuario - Killer Queen
Manual de usuario sobre el lenguaje de programación _Killer Queen_

Todo el código que se genere tiene que pertenecer a un archivo de terminación `.eo`.

Nota: Cuando se haga alusión a una variable (nombre personalizable) en las explicaciones, se encontrará dicho texto rodeado de `<` `>`, dichos caracteres *NO* forman parte del código  
Por ejemplo: 
```
entero < variable > ;
```

Ya con la variable seleccionada la misma línea de código se vería así:  
```
entero var1;
```

## Estructura básica

La estructura básica de un código del lenguaje es la siguiente:

```
programa < id > ;

< declaraciones >

principal() {
  < estatutos >
}
```

### ID Programa
La parte inicial del código sirve para indiciar el nombre del programa.

### Declaraciones

Dentro de la sección de `declaraciones` se pueden declarar variables globales y/o funciones.

#### Variables

La variables se declaran de la siguiente manera:

```
< tipo > < id > < dimensiones > ;
```

Donde el `tipo` y el `id` son requisitos, mientras que las `dimensiones` son opcionales.

Los tipos de variables existentes en el lenguaje son:
- entero
- flotante
- char

La estructura de las dimensiones es la siguiente

```
[ < num entero > ] [ < num entero > ]
```

También se pueden declarar varias variables en la misma linea de la siguiente manera: 

```
< tipo > < id > < dimensiones > , < id > < dimensiones > , < id > < dimensiones > , ... < id > < dimensiones > ;
```

##### Ejemplos
```
entero num;
flotante promedio;
char letra;
entero a, b, c, d;
char nombre[10];
```

#### Funciones

Las funciones se declaran de la siguiente manera:

```
< tipo > funcion < id > ( < parametros > ) { < estatutos > }
```

Los tipos de variables existentes en el lenguaje son:
- entero
- flotante
- char
- void

Los parametros son atómicos y siguen la siguiente estructura:

```
< tipo > < id > , < tipo > < id >, < tipo > < id > ... , < tipo > < id >
```

Puede que una función no tenga parámetros.

##### Ejemplos
```
entero funcion suma(entero a, entero b) { < estatutos > }
flotante funcion multiplicacion(flotante a, flotante b) { < estatutos > }
void funcion imprimir() { < estatutos > }
```
### Expresiones

Cada expresión _< exp >_, _< expresion >_ genera derivaciones para ejecutar operaciones aritméticas, lógicas y relacionales. 

### Estatutos

Los estatutos son las acciones de código que pertenecen dentro de una función y son las siguientes:
- Asignaciones
- Lectura
- Escritura
- Llamada de función
- Ciclos
- Condicionales
- Retornos
- Comentarios

Representa la "columna vertebral" del lenguaje, casi todas las acciones por ejecutar derivan de _estatuto_.

#### Asignaciones

Todas las asginaciones tienen la siguiente estructura:
```
< id > = < exp >;
```
Cada expresión _< exp >_ permite asignar el resultado obtenido a < id >. 

##### Ejemplos
```
num = 10;
promedio = 9.7;
letra = 'J';
a = b - d / c * 3;
nombre[1] = 'M';
```
#### Lectura
La estructura para mostrar un mensaje en la consola es la siguiente.

```
 lee ( < id >, < id >, < id > ... , < id > );
```
##### Ejemplo
```
lee(num, promedio);
```
#### Escritura
La estructura para escribir un mensaje en la consola es la siguiente.

```
 escribe ( < texto >, < texto >, < texto > ... , < texto > );
```
También se permiten que se escriba el resultado de una expresión siguiendo la estructura de
```
 escribe ( < expresion > );
```

##### Ejemplos
```
escribe(suma(2, 3, 4));
escribe("Hola mundo");
escribe(num);
```
#### Llamada de función
La estructura para llamar una función es la siguiente.

```
< id > ( < expresion >, < expresion > );
```
Cada expresión va a representar el parámetro a enviar de dicha función.
También se permiten que se escriba el resultado de una expresión siguiendo la estructura de

##### Ejemplos
```
suma(2, 3, 4);
llenar_arreglo();
resta(9 - 4);
```
#### Repetición
Hay dos estructura para hacer un ciclo, depende de si sigues un formato de _while loop_ o un formato de _for loop_. 

##### While loop
```
mientras ( < expresion > ){ < estatuto > }
```

##### For loop
```
desde < id > = < exp > hasta < exp > { < estatuto > }
```
Cada expresión va a representar el parámetro a enviar de dicha función.
También se permiten que se escriba el resultado de una expresión siguiendo la estructura de

##### Ejemplos
```
desde i = 10 hasta 20 {
    escribe(i);
}
mientras ( var > 1) {
    var = var - 1;
    acum = acum * (var);
  }
```
#### Condicionales
La estructura para realizar una decisión en el lenguaje es la siguiente.

```
si ( < expresion >) { 
  < estatuto > 
}
sino { < estatuto > }
```

##### Ejemplos
```
si (var > 0) {
    regresa 1;
}
sino{
  regresa i;
}
```
#### Retornos
La estructura para ejecutar un retorno de una función en el lenguaje es la siguiente.
```
regresa < exp >;
```

##### Ejemplo
```
regresa i;
```

#### Comentarios
La estructura para mostrar un comentario en el lenguaje es la siguiente.
```
%% < texto > %%
```
Solo se permite hacer comentarios dentro de las funciones.

##### Ejemplos
```
%% entero i; %%
```