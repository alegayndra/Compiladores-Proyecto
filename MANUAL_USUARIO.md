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
