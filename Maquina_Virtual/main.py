from os import DirEntry, read
from pathlib import Path
import io
import sys

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
  [ #000 Locales
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

lista_cuadruplos = []
num_cuadruplo = [0]

def extaerMemoria(direccion):
  contexto = len(dir_memoria) - 1
  # Itera sobre todos los contextos
  while contexto >= 0:
    tipoVariable = len(dir_memoria[contexto]) - 1

    # Itera sobre todos los tipos de variables
    while tipoVariable >= 0:
      esTemporal = len(dir_memoria[contexto][tipoVariable]) - 1
      
      # Itera sobre los espacios reservados para las variables normales y temporales
      while esTemporal >= 0:
        if direccion >= dir_memoria[contexto][tipoVariable][esTemporal]:
          # Checa si estamos en un contexto local
          if contexto == 1:
            return mapa_memoria[contexto][len(mapa_memoria[contexto]) - 1][tipoVariable][esTemporal][direccion - dir_memoria[contexto][tipoVariable][esTemporal]]
          else:
            return mapa_memoria[contexto][tipoVariable][esTemporal][direccion - dir_memoria[contexto][tipoVariable][esTemporal]]
        esTemporal -= 1
      tipoVariable -= 1
    contexto -= 1
  return None

# Funcion que realiza operaciones con dos operados
# Operaciones aritmeticas, relaciones y lógicas
def operacionNormal(opIzq, opDer, op):
  if opIzq != -1:
    opIzq = extaerMemoria(opIzq)
  else:
    return None
  if opDer != -1:
    opDer = extaerMemoria(opDer)
  else:
    return None
    
  if op == 0:
    return opIzq + opDer
  elif op == 1:
    return opIzq - opDer
  elif op == 2:
    return opIzq * opDer
  elif op == 3:
    return opIzq / opDer
  elif op == 4:
    return 1 if opIzq > opDer else 0
  elif op == 5:
    return 1 if opIzq < opDer else 0
  elif op == 6:
    return 1 if opIzq >= opDer else 0
  elif op == 7:
    return 1 if opIzq <= opDer else 0
  elif op == 8:
    return 1 if opIzq == opDer else 0
  elif op == 9:
    return 1 if opIzq != opDer else 0
  elif op == 10:
    return 1 if opIzq and opDer else 0
  elif op == 11:
    return 1 if opIzq or opDer else 0
  else:
    return None

def guardarValor(valor, direccion):
  contexto = len(dir_memoria) - 1
  # Itera sobre todos los contextos
  while contexto >= 0:
    tipoVariable = len(dir_memoria[contexto]) - 1

    # Itera sobre todos los tipos de variables
    while tipoVariable >= 0:
      esTemporal = len(dir_memoria[contexto][tipoVariable]) - 1
      
      # Itera sobre los espacios reservados para las variables normales y temporales
      while esTemporal >= 0:
        if direccion >= dir_memoria[contexto][tipoVariable][esTemporal]:
          
          # Checa si estamos en un contexto local
          if contexto == 1:
            mapa_memoria[contexto][len(mapa_memoria[contexto]) - 1][tipoVariable][esTemporal][direccion - dir_memoria[contexto][tipoVariable][esTemporal]] = valor
            return
          else:
            mapa_memoria[contexto][tipoVariable][esTemporal][direccion - dir_memoria[contexto][tipoVariable][esTemporal]] = valor
            return
        esTemporal -= 1
      tipoVariable -= 1
    contexto -= 1
  return None

def asignacion(valor, destino):
  dirDestino = None
  dirValor = None
  contexto = len(dir_memoria) - 1
  # Itera sobre todos los contextos
  while contexto >= 0:
    tipoVariable = len(dir_memoria[contexto]) - 1

    # Itera sobre todos los tipos de variables
    while tipoVariable >= 0:
      esTemporal = len(dir_memoria[contexto][tipoVariable]) - 1
      
      # Itera sobre los espacios reservados para las variables normales y temporales
      while esTemporal >= 0:
        if destino >= dir_memoria[contexto][tipoVariable][esTemporal]:
          dirDestino = [
            contexto,
            tipoVariable, 
            esTemporal
          ]
        if valor >= dir_memoria[contexto][tipoVariable][esTemporal]:
          dirValor = [
            contexto,
            tipoVariable, 
            esTemporal
          ]

        # Checa que se hayan encontrado ambas posiciones dentro de la memoria
        if dirValor and dirDestino:
          break
        esTemporal -= 1

      # Checa que se hayan encontrado ambas posiciones dentro de la memoria
      if dirValor and dirDestino:
        break
      tipoVariable -= 1

    # Checa que se hayan encontrado ambas posiciones dentro de la memoria
    if dirValor and dirDestino:
      break
    contexto -= 1
  
  # Checa que se hayan encontrado ambas posiciones dentro de la memoria
  if dirValor and dirDestino:
    # Checa si las variables están en un cotexto local
    if dirDestino[1] == 1 and dirValor[1] == 1: # Ambas son locales
      mapa_memoria[dirDestino[0]][len(mapa_memoria[dirDestino[0]]) - 1][dirDestino[1]][dirDestino[2]] = mapa_memoria[dirValor[0]][len(mapa_memoria[dirValor[0]]) - 1][dirValor[1]][dirValor[2]]
    elif dirDestino[1] == 1: # Destino es local
      mapa_memoria[dirDestino[0]][len(mapa_memoria[dirDestino[0]]) - 1][dirDestino[1]][dirDestino[2]] = mapa_memoria[dirValor[0]][dirValor[1]][dirValor[2]]
    elif dirValor[1] == 1: # Valor a asignas es local
      mapa_memoria[dirDestino[0]][dirDestino[1]][dirDestino[2]] = mapa_memoria[dirValor[0]][len(mapa_memoria[dirValor[0]]) - 1][dirValor[1]][dirValor[2]]
    else: # Ninguna es local
      mapa_memoria[dirDestino[0]][dirDestino[1]][dirDestino[2]] = mapa_memoria[dirValor[0]][dirValor[1]][dirValor[2]]

def escribe(valor):
  print(extaerMemoria(valor))

def leerValor(valor, direccion):
  contexto = len(dir_memoria) - 1
  # Itera sobre todos los contextos
  while contexto >= 0:
    tipoVariable = len(dir_memoria[contexto]) - 1

    # Itera sobre todos los tipos de variables
    while tipoVariable >= 0:
      esTemporal = len(dir_memoria[contexto][tipoVariable]) - 1
      
      # Itera sobre los espacios reservados para las variables normales y temporales
      while esTemporal >= 0:
        if direccion >= dir_memoria[contexto][tipoVariable][esTemporal]:
          if tipoVariable == 2 and len(valor) > 1:
            print("Tipos incompatibles")
            sys.exit()
          if tipoVariable == 1:
            valor = float(valor)
          if tipoVariable == 0:
            valor = int(valor)
          # Checa si estamos en un contexto local
          if contexto == 1:
            mapa_memoria[contexto][len(mapa_memoria[contexto]) - 1][tipoVariable][esTemporal][direccion - dir_memoria[contexto][tipoVariable][esTemporal]] = valor
            return
          else:
            mapa_memoria[contexto][tipoVariable][esTemporal][direccion - dir_memoria[contexto][tipoVariable][esTemporal]] = valor
            return
        esTemporal -= 1
      tipoVariable -= 1
    contexto -= 1
  return None

def lee(valor):
  var = input("")
  leerValor(var, valor)

def switchCubo(cuadruplo):
  if cuadruplo[0] >= 0 and cuadruplo[0] <= 11:
    guardarValor(operacionNormal(cuadruplo[1], cuadruplo[2], cuadruplo[0]), cuadruplo[3])
    return
  elif cuadruplo[0] == 12: # Asignacion
    asignacion(cuadruplo[1], cuadruplo[2])
    return
  elif cuadruplo[0] == 13: # Print
    escribe(cuadruplo[3])
    return
  elif cuadruplo[0] == 14: # Read
    lee(cuadruplo[3])
    return
  elif cuadruplo[0] == 15: # Goto
    num_cuadruplo[0] = cuadruplo[3] - 1
    return
  elif cuadruplo[0] == 16: # GotoT
    if extaerMemoria(cuadruplo[1]):
      num_cuadruplo[0] = cuadruplo[3] - 1
    return
  elif cuadruplo[0] == 17: # GotoF
    # print(type(cuadruplo[3]))
    # print(cuadruplo[3])
    # print(mapa_memoria[0][0])
    print(extaerMemoria[cuadruplo[1]])
    if not extaerMemoria(cuadruplo[1]):
      print("entramos")
      num_cuadruplo[0] = cuadruplo[3] - 1
    else:
      print("No entramos")
    return
  elif cuadruplo[0] == 18: # GoSub
    return
  elif cuadruplo[0] == 19: # ERA
    return
  elif cuadruplo[0] == 20: # Return
    return
  elif cuadruplo[0] == 21: # Param
    return

def ejecutar_programa():
  print(len(lista_cuadruplos))
  while num_cuadruplo[0] < len(lista_cuadruplos):
    print("Ejecutamos el cuadruplo #", num_cuadruplo[0])
    print(lista_cuadruplos[num_cuadruplo[0]])
    switchCubo(lista_cuadruplos[num_cuadruplo[0]]) 
    print("Ahorita vamos en el cuadruplo --> ",num_cuadruplo)
    num_cuadruplo[0] += 1

'''
 Funciones que guardan los valores del .txt en memoria
'''

def leerCuadruplos(txt_cuadruplos):
  readStr = 0
  while readStr < len(txt_cuadruplos):
    cantidades = txt_cuadruplos[ readStr:txt_cuadruplos.find( '\n', readStr ) ]
    operador = int( cantidades[ 1:cantidades.find( ',' ) ] )
    cantidades = cantidades[ cantidades.find( ',' ) + 2: ]
    varIzq = int(cantidades[ :cantidades.find(',')] )
    cantidades = cantidades[ cantidades.find(',') + 2: ]
    varDer = int(cantidades[ :cantidades.find( ',' ) ] )
    cantidades = cantidades[ cantidades.find(',') + 2:]
    guardar = int(cantidades[ :cantidades.find( ')' ) ] )

    lista_cuadruplos.append((operador, varIzq, varDer, guardar))
    
    readStr = txt_cuadruplos.find('\n', readStr) + 1
  ejecutar_programa()

def guardarMapaGlobs(direcciones_globs):
  readStr = 0
  cantidades = direcciones_globs[readStr:direcciones_globs.find('\n', readStr)]
  readStr = direcciones_globs.find('\n', readStr) + 1
  cntIntNormal = cantidades[1:cantidades.find(',')] 
  cantidades = cantidades[cantidades.find(',') + 2:]
  cntIntTemp = cantidades[:cantidades.find(')')]
  
  cantidades = direcciones_globs[readStr:direcciones_globs.find('\n', readStr)]
  readStr = direcciones_globs.find('\n', readStr) + 1
  cntFloatNormal = cantidades[1:cantidades.find(',')] 
  cantidades = cantidades[cantidades.find(',') + 2:]
  cntFloatTemp = cantidades[:cantidades.find(')')]
  
  cantidades = direcciones_globs[readStr:direcciones_globs.find('\n', readStr)]
  readStr = direcciones_globs.find('\n', readStr) + 1
  cntCharNormal = cantidades[1:cantidades.find(',')] 
  cantidades = cantidades[cantidades.find(',') + 2:]
  cntCharTemp = cantidades[:cantidades.find(')')]
  
  mapa_memoria[0][0][0] = [None] * int(cntIntNormal)
  mapa_memoria[0][0][1] = [None] * int(cntIntTemp)
  mapa_memoria[0][1][0] = [None] * int(cntFloatNormal)
  mapa_memoria[0][1][1] = [None] * int(cntFloatTemp)
  mapa_memoria[0][2][0] = [None] * int(cntCharNormal)
  mapa_memoria[0][2][1] = [None] * int(cntCharTemp)

def guardarMapaCons(direcciones_const):
  readStr = 0
  cantidades = direcciones_const[readStr:direcciones_const.find('\n', readStr)]
  cntInt = cantidades[1:cantidades.find(',')]
  cantidades = cantidades[cantidades.find(',') + 2:]
  cntFloat = cantidades[:cantidades.find(',')]
  cantidades = cantidades[cantidades.find(',') + 2:]
  cntChar = cantidades[:cantidades.find(',')]
  mapa_memoria[2][0][0] = [None] * int(cntInt)
  mapa_memoria[2][1][0] = [None] * int(cntFloat)
  mapa_memoria[2][2][0] = [None] * int(cntChar)

  direcciones_const = direcciones_const[direcciones_const.find('\n') + 1:]
  
  while readStr < len(direcciones_const):
    valYdir = direcciones_const[readStr:direcciones_const.find('\n', readStr)]

    value = valYdir[1:valYdir.find(',')]
    valYdir = valYdir[valYdir.find(',') + 2:]
    direccion = valYdir[:valYdir.find(',')]
    valYdir = valYdir[valYdir.find(',') + 2:]
    tipo = valYdir[:valYdir.find(')')]

    if tipo == "entero":
      value = int(value)
    elif tipo == "flotante":
      value = float(value)
    direccion = int(direccion)
    i = len(dir_memoria[2])-1
    while i >= 0:
      if(direccion >= dir_memoria[2][i][0]):
        mapa_memoria[2][i][0][direccion - dir_memoria[2][i][0]] = value
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
  guardarMapaCons(stringTxt[stringTxt.find("CONSTANTES") + 11:stringTxt.find("FIN_CONSTANTES")])

  #Registro de valores para globales en mapa de memoria
  guardarMapaGlobs(stringTxt[stringTxt.find("GLOBALES") + 9:stringTxt.find("FIN_GLOBALES")])

  leerCuadruplos(stringTxt[stringTxt.find("CUADRUPLOS") + 11:stringTxt.find("FIN_CUADRUPLOS")])
  print("Aqui estan las variables globales")
  print(mapa_memoria[0])
  print("")
  print("Aqui estan las variables normales globales")
  print(mapa_memoria[0][0][0], mapa_memoria[0][1][0], mapa_memoria[0][2][0])
  print("")
  print("Aqui estan las variables temporales globales")
  print(mapa_memoria[0][0][1], mapa_memoria[0][1][1], mapa_memoria[0][2][1])
  print("")

  print("Aqui estan las variables locales")
  print(mapa_memoria[1])
  
  print("Aqui estan las variables constantes")
  print(mapa_memoria[2])
  print("")
  print("Aqui estan las variables INT constantes")
  print(mapa_memoria[2][0])
  print("")
  print("Aqui estan las variables FLOAT globales")
  print(mapa_memoria[2][1])
  print("")
  print("Aqui estan las variables CHAR globales")
  print(mapa_memoria[2][2])

leer_obj()
ejecutar_programa()
