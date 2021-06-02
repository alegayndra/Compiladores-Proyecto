# Estructura de validación de segmentación de memori
dir_memoria = [
  [ # Globales
    # Normales Temporales
    [    0,      833 ], # Enteros
    [ 1250,     2083 ], # Flotantes
    [ 2500,     2833 ], # Caracteres
  ],
  [ # Locales
    # Normales Temporales
    [ 3000,     4665 ], # Enteros
    [ 5500,     7164 ], # Flotantes
    [ 8000,     8666 ], # Caracteres
  ],
  [ # Constantes
    # Normales 
    [ 9000 ], # Enteros
    [ 9401 ], # Flotantes
    [ 9801 ], # Caracteres
  ],
]

# Estructura para validación de límite de memoria 
# del segmento correspondien
limitesVarsLocales = [
  # Normales Temporales
  [ 4665,     5500 ], # Enteros
  [ 7164,     8000 ], # Flotantes
  [ 8666,     9000 ], # Caracteres
]

# Contador de cuántas variables hay cuando estemos en memoria local
cantVarsLocales = [
  # Normales Temporales
  [ 0,     0 ], # Enteros
  [ 0,     0 ], # Flotantes
  [ 0,     0 ], # Caracteres
]

# Template que agarra cada función para la estructura de memoria local
auxLocales = [
  # Normales Temporales
  [ [],      [] ], # Enteros
  [ [],      [] ], # Flotantes
  [ [],      [] ], # Caracteres
]

# Mapa de para guardar toda la memoria
mapa_memoria = [
  [ # Globales
    # Normales Temporales
    [ [],      [] ], # Enteros
    [ [],      [] ], # Flotantes
    [ [],      [] ], # Caracteres
  ],
  [], # Locales,
  [ # Constantes
    # Normales
    [ [] ], # Enteros
    [ [] ], # Flotantes
    [ [] ], # Caracteres
  ]
]

funciones = [] # Guarda temporalmente la información de las funciones
memoriaFuncionEnProgreso = [] # Guarda las funciones pendientes por procesar

pila_cuadruplos = [] # Stack de cuadruplos pendientes por procesar
lista_cuadruplos = [] # Guarda todos los cuadruplos a ejecutar

num_cuadruplo = [0] # Cuadruplo que se esta leyendo actualmente