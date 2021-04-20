# Compiladores-Proyecto
Repo para el proyecto final de la clase de Diseño de Compiladores

## Instrucciones 

Para correr el programa, se necesita tener instalado el ambiente de desarrollo rust. Una vez instalado, se corre `cargo build` dentro de la carpeta para instalar las dependencias y `cargo run` para correr el programa. Para correr las pruebas unitarias se corre `cargo test`.

## Avances

### Avance Semana 2
- Parser completo
- Tabla de variables definida

#### **Documentos**
- Pequeños ajustes en estructura de gramáticas 
    - Agregar token ';' al final de algunas instrucciones.
    - En no terminal 'Factor', los tokens de '+' y '-' son opcionales previo al no terminal 'Valor'
    - Expresiones son opcionales en algunos casos.

#### **Código**
- Analizador léxico de _white space_.
- Analizador léxico de tipos.
- Parser completo.
- Tabla de variables.
