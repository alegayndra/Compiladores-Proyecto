from os import DirEntry, read
from pathlib import Path
import io

dir_memoria = [
  [ # Globales
    [ # Enteras
      # Inicio  Limite
      0,    833 # Normales
    ],
    [ # Flotantes
      # Inicio  Limite
      1250,    2083 # Normales
    ],
    [ # Caracteres
      # Inicio  Limite
      2500,    2833 # Normales
    ]
  ],
  [ # Locales
    [ # Enteras
      # Inicio  Limite
      3000,    4665 # Normales
    ],
    [ # Flotantes
      # Inicio  Limite
      5500,    7164 # Normales
    ],
    [ # Caracteres
      # Inicio  Limite
      8000,    8666 # Normales
    ]
  ],
  [ # Constantes
    [ # Enteras
      # Inicio
      9000 # Normales
    ],
    [ # Flotantes
      # Inicio
      9401 # Normales
    ],
    [ # Caracteres
      # Inicio
      9801 # Normales
    ]
  ]
]

auxLocales = [ # Locales
    [ # Enteras
      # Inicio  Limite
      [ [],    []], # Normales
    ],
    [ # Flotantes
      # Inicio  Limite
      [ [],    []], # Normales
    ],
    [ # Caracteres
      # Inicio  Limite
      [ [],    []], # Normales
    ]
]

mapa_memoria = [
  [ # Globales
    [ # Enteras
      # Inicio  Limite
      [],  [] # Normales
    ],
    [ # Flotantes
      # Inicio  Limite
      [],  [] # Normales
    ],
    [ # Caracteres
      # Inicio  Limite
      [],  [] # Normales
    ]
  ],
  [ # Locales
   # Aqui se van a pushear stacks de memoria
  ],
  [ # Constantes
    [ # Enteras
      # Inicio
      [] # Normales
    ],
    [ # Flotantes
      # Inicio
      [] # Normales
    ],
    [ # Caracteres
      # Inicio
      [] # Normales
    ]
  ]
]

def guardarMapaGlobs(direcciones_globs):
  readStr = 0
  print(direcciones_globs)
  cantidades = direcciones_globs[readStr:direcciones_globs.find('\n', readStr)]
  readStr = direcciones_globs.find('\n', readStr)+1
  cntIntNormal = cantidades[1:cantidades.find(',')] 
  cantidades = cantidades[cantidades.find(',')+1:]
  cntIntTemp = cantidades[:cantidades.find(')')]
  
  cantidades = direcciones_globs[readStr:direcciones_globs.find('\n', readStr)]
  readStr = direcciones_globs.find('\n', readStr)+1
  cntFloatNormal = cantidades[1:cantidades.find(',')] 
  cantidades = cantidades[cantidades.find(',')+1:]
  cntFloatTemp = cantidades[:cantidades.find(')')]
  
  cantidades = direcciones_globs[readStr:direcciones_globs.find('\n', readStr)]
  readStr = direcciones_globs.find('\n', readStr)+1
  cntCharNormal = cantidades[1:cantidades.find(',')] 
  cantidades = cantidades[cantidades.find(',')+1:]
  cntCharTemp = cantidades[:cantidades.find(')')]
  
  mapa_memoria[0][0][0] = [None] * int(cntIntNormal)
  mapa_memoria[0][0][1] = [None] * int(cntIntTemp)
  mapa_memoria[0][1][0] = [None] * int(cntFloatNormal)
  mapa_memoria[0][1][1] = [None] * int(cntFloatTemp)
  mapa_memoria[0][2][0] = [None] * int(cntCharNormal)
  mapa_memoria[0][2][1] = [None] * int(cntCharTemp)

def gaurdarMapaCons(direcciones_const):
  readStr = 0
  cantidades = direcciones_const[readStr:direcciones_const.find('\n', readStr)]
  cntInt = cantidades[1:cantidades.find(',')]
  cantidades = cantidades[cantidades.find(',')+1:]
  cntFloat = cantidades[:cantidades.find(',')]
  cantidades = cantidades[cantidades.find(',')+1:]
  cntChar = cantidades[:cantidades.find(',')]
  mapa_memoria[2][0] = [None] * int(cntInt)
  mapa_memoria[2][1] = [None] * int(cntFloat)
  mapa_memoria[2][2] = [None] * int(cntChar)

  direcciones_const = direcciones_const[direcciones_const.find('\n')+1:]
  
  while readStr < len(direcciones_const):
    valYdir = direcciones_const[readStr:direcciones_const.find('\n', readStr)]

    value = valYdir[1:valYdir.find(',')]
    valYdir = valYdir[valYdir.find(',')+2:]
    direccion = valYdir[:valYdir.find(',')]
    valYdir = valYdir[valYdir.find(',')+2:]
    tipo = valYdir[:valYdir.find(')')]

    if tipo == "entero":
      value = int(value)
    elif tipo == "flotante":
      value = float(value)
    direccion = int(direccion)
    i = len(dir_memoria[2])-1
    while i >= 0:
      if(direccion >= dir_memoria[2][i][0]):
        mapa_memoria[2][i][direccion-dir_memoria[2][i][0]] = value
        break
      i -= 1

    readStr = direcciones_const.find('\n', readStr) + 1

def leer_obj():
  # Lectura y normalizacion de archivo
  prueba_cuadruplos = Path("C:/Users/delca/Documents/Tareas TEC/Compiladores/Compiladores-Proyecto/Compilador/cuadruplos")
  abrir = prueba_cuadruplos / "Killer_queen.txt"
  file_opened = open(abrir, 'r')
  stringTxt = file_opened.read()

  #Registro de valores para constantes en mapa de memoria
  gaurdarMapaCons(stringTxt[stringTxt.find("CONSTANTES")+11:stringTxt.find("FIN_CONSTANTES")])

  #Registro de valores para globales en mapa de memoria
  guardarMapaGlobs(stringTxt[stringTxt.find("GLOBALES")+9:stringTxt.find("FIN_GLOBALES")])


leer_obj()