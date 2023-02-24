from lectura import *
from globales import *
from copy import *

# Funcion principal para obtener una 
# direccion de memoria en el mapa_memoria
def extraerMemoria(direccion):
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

# Funcion que realiza operaciones con dos operandos
# Operaciones aritmeticas, relaciones y lógicas
def operacionNormal(izq, der, op):
  if izq != -1:
    opIzq = extraerMemoria(izq)
  else:
    return None
  if der != -1:
    opDer = extraerMemoria(der)
  else:
    return None

  # Identificar operación a resolver siguiendo la
  # tabla de código de operaciones
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

# Almacena valor en cierta dirección de memoria, 
# dentro de mapa_memoria
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

# Itera buscando ambas direcciones a la vez,
# la de valor y la de destino
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
        if (not dirDestino) and destino >= dir_memoria[contexto][tipoVariable][esTemporal]:
          dirDestino = [
            contexto,
            tipoVariable, 
            esTemporal
          ]
        if (not dirValor) and valor >= dir_memoria[contexto][tipoVariable][esTemporal]:
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
    base_destino = dir_memoria[dirDestino[0]][dirDestino[1]][dirDestino[2]]
    base_valor = dir_memoria[dirValor[0]][dirValor[1]][dirValor[2]]
    if dirDestino[0] == 1 and dirValor[0] == 1: # Ambas son locales
      mapa_memoria[dirDestino[0]][len(mapa_memoria[dirDestino[0]]) - 1][dirDestino[1]][dirDestino[2]][destino - base_destino] = mapa_memoria[dirValor[0]][len(mapa_memoria[dirValor[0]]) - 1][dirValor[1]][dirValor[2]][valor - base_valor]
    elif dirDestino[0] == 1: # Destino es local
      mapa_memoria[dirDestino[0]][len(mapa_memoria[dirDestino[0]]) - 1][dirDestino[1]][dirDestino[2]][destino - base_destino] = mapa_memoria[dirValor[0]][dirValor[1]][dirValor[2]][valor - base_valor]
    elif dirValor[0] == 1: # Valor a asignar es local
      mapa_memoria[dirDestino[0]][dirDestino[1]][dirDestino[2]][destino - base_destino] = mapa_memoria[dirValor[0]][len(mapa_memoria[dirValor[0]]) - 1][dirValor[1]][dirValor[2]][valor - base_valor]
    else: # Ninguna es local
      mapa_memoria[dirDestino[0]][dirDestino[1]][dirDestino[2]][destino - base_destino] = mapa_memoria[dirValor[0]][dirValor[1]][dirValor[2]][valor - base_valor]

# Despliega lo que se encuentre en dado segmento de memoria
def escribe(valor):
  print(extraerMemoria(valor))

# Busca dirección de memoria para guardar un valor
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

# Actualiza el indicador de nuestro cuádruplo actual
def goto(cuadruplo):
  num_cuadruplo[0] = cuadruplo - 1

# Hace goto() si lo que se busca en memoria es falso
def gotof(valor, cuadruplo):
  if not extraerMemoria(valor):
    goto(cuadruplo)

# Libera memoria de función procesada
def endfunc():
  cntIntNormal   =  len(mapa_memoria[1][len(mapa_memoria[1]) - 1][0][0])
  cntIntTemp     =  len(mapa_memoria[1][len(mapa_memoria[1]) - 1][0][1])
  cntFloatNormal =  len(mapa_memoria[1][len(mapa_memoria[1]) - 1][1][0])
  cntFloatTemp   =  len(mapa_memoria[1][len(mapa_memoria[1]) - 1][1][1])
  cntCharNormal  =  len(mapa_memoria[1][len(mapa_memoria[1]) - 1][2][0])
  cntCharTemp    =  len(mapa_memoria[1][len(mapa_memoria[1]) - 1][2][1])

  cantVarsLocales[0][0] -= cntIntNormal
  cantVarsLocales[0][1] -= cntIntTemp
  cantVarsLocales[1][0] -= cntFloatNormal
  cantVarsLocales[1][1] -= cntFloatTemp
  cantVarsLocales[2][0] -= cntCharNormal
  cantVarsLocales[2][1] -= cntCharTemp
  mapa_memoria[1].pop()
  goto(pila_cuadruplos[len(pila_cuadruplos) - 1] + 1)
  pila_cuadruplos.pop()

# Regresa valor de la función y cambia de contexto
def regresa(valor, destino):
  asignacion(valor, destino)
  endfunc()

# Pide input al usuario
def lee(valor):
  var = input("")
  leerValor(var, valor)

# Creación del Active Record
def era(funcion):
  memoriaFuncionEnProgreso.append(deepcopy(auxLocales))
  
  # Busca la función indicada en ejecución
  for i in range(len(funciones)):
    # Prepara memoria temporal local con valores leídos del .txt
    if funciones[i][0][2] == funcion:
      cntIntNormal   =  funciones[i][1][0]
      cntIntTemp     =  funciones[i][1][1]
      cntFloatNormal =  funciones[i][2][0]
      cntFloatTemp   =  funciones[i][2][1]
      cntCharNormal  =  funciones[i][3][0]
      cntCharTemp    =  funciones[i][3][1]

      cantVarsLocales[0][0] += cntIntNormal
      cantVarsLocales[0][1] += cntIntTemp
      cantVarsLocales[1][0] += cntFloatNormal
      cantVarsLocales[1][1] += cntFloatTemp
      cantVarsLocales[2][0] += cntCharNormal
      cantVarsLocales[2][1] += cntCharTemp

      # Checan que cada segmento no exceda el límite de memoria definido
      if cantVarsLocales[0][0] >= limitesVarsLocales[0][0] - dir_memoria[1][0][0]:
        print("Se excedió la memoria disponibles para enteros dentro del contexto local")
        sys.exit()
      if cantVarsLocales[0][1] >= limitesVarsLocales[0][1] - dir_memoria[1][0][1]:
        print("Se excedió la memoria disponibles para enteros termporales dentro del contexto local")
        sys.exit()
      if cantVarsLocales[1][0] >= limitesVarsLocales[1][0] - dir_memoria[1][1][0]:
        print("Se excedió la memoria disponibles para flotantes dentro del contexto local")
        sys.exit()
      if cantVarsLocales[1][1] >= limitesVarsLocales[1][1] - dir_memoria[1][1][1]:
        print("Se excedió la memoria disponibles para flotantes termporales dentro del contexto local")
        sys.exit()
      if cantVarsLocales[2][0] >= limitesVarsLocales[2][0] - dir_memoria[1][2][0]:
        print("Se excedió la memoria disponibles para caracter dentro del contexto local")
        sys.exit()
      if cantVarsLocales[2][1] >= limitesVarsLocales[2][1] - dir_memoria[1][2][1]:
        print("Se excedió la memoria disponibles para caracter termporales dentro del contexto local")
        sys.exit()

      # Inicializa la cantidad de memoria requerida por la función,
      # no más, no menos
      memoriaFuncionEnProgreso[len(memoriaFuncionEnProgreso) - 1][0][0] = [None] * int(cntIntNormal)
      memoriaFuncionEnProgreso[len(memoriaFuncionEnProgreso) - 1][0][1] = [None] * int(cntIntTemp)
      memoriaFuncionEnProgreso[len(memoriaFuncionEnProgreso) - 1][1][0] = [None] * int(cntFloatNormal)
      memoriaFuncionEnProgreso[len(memoriaFuncionEnProgreso) - 1][1][1] = [None] * int(cntFloatTemp)
      memoriaFuncionEnProgreso[len(memoriaFuncionEnProgreso) - 1][2][0] = [None] * int(cntCharNormal)
      memoriaFuncionEnProgreso[len(memoriaFuncionEnProgreso) - 1][2][1] = [None] * int(cntCharTemp)
      return
    i += 1

# Extrae valor en memoria para asignarlo a los parámetros de una función
def param(valor, parametro):
  val = extraerMemoria(valor)
  tipo = len(dir_memoria[1]) - 1
  while tipo >= 0:
    if parametro >= dir_memoria[1][tipo][0]:
      memoriaFuncionEnProgreso[len(memoriaFuncionEnProgreso) - 1][tipo][0][parametro - dir_memoria[1][tipo][0]] = val
    tipo -= 1
  
# Cambio de contexto y salto al cuádruplo de la función a procesar
def gosub(cuad_funcion):
  mapa_memoria[1].append(deepcopy(memoriaFuncionEnProgreso[len(memoriaFuncionEnProgreso) - 1]))
  memoriaFuncionEnProgreso.pop()
  pila_cuadruplos.append(num_cuadruplo[0])
  goto(cuad_funcion)

# Valida que la memoria a acceder de una dimension 
def verificar(lim_inf, lim_sup, val_verificar):
  valor = extraerMemoria(val_verificar)
  if valor >= extraerMemoria(lim_inf) and valor <= extraerMemoria(lim_sup):
    return
  else:
    print("El subindice excede las dimensiones del arreglo")
    sys.exit()

# Busca memoria de dirección de arreglo y guarda la referencia
def acceder(apuntador, destino):
  asignacion(extraerMemoria(apuntador), destino)
  return

# Realiza la asignación de valor para index de un arreglo
def asignacionArreglo(valor, destino):
  asignacion(valor, extraerMemoria(destino))
  return

# Identificar acción a ejecutar
def switchCubo(cuadruplo):
  if cuadruplo[0] >= 0 and cuadruplo[0] <= 11:
    guardarValor(operacionNormal(cuadruplo[1], cuadruplo[2], cuadruplo[0]), cuadruplo[3])
    return
  elif cuadruplo[0] == 12: # Asignacion
    asignacion(cuadruplo[1], cuadruplo[3])
    return
  elif cuadruplo[0] == 13: # Print
    escribe(cuadruplo[3])
    return
  elif cuadruplo[0] == 14: # Read
    lee(cuadruplo[3])
    return
  elif cuadruplo[0] == 15: # Goto
    goto(cuadruplo[3])
    return
  elif cuadruplo[0] == 16: # GotoT
    if extraerMemoria(cuadruplo[1]):
      num_cuadruplo[0] = cuadruplo[3] - 1
    return
  elif cuadruplo[0] == 17: # GotoF
    gotof(cuadruplo[1], cuadruplo[3])
    return
  elif cuadruplo[0] == 18: # EndFunc
    endfunc()
    return
  elif cuadruplo[0] == 19: # Return
    regresa(cuadruplo[1], cuadruplo[3])
    return
  elif cuadruplo[0] == 20: # ERA
    era(cuadruplo[3])
    return
  elif cuadruplo[0] == 21: # Param
    param(cuadruplo[1], cuadruplo[3])
    return
  elif cuadruplo[0] == 22: # GoSub
    gosub(cuadruplo[3])
    return
  elif cuadruplo[0] == 23: # VER
    verificar(cuadruplo[2], cuadruplo[3], cuadruplo[1])
    return
  elif cuadruplo[0] == 24: # ACC
    acceder(cuadruplo[1], cuadruplo[3])
    return
  elif cuadruplo[0] == 25: # ASG
    asignacionArreglo(cuadruplo[1], cuadruplo[3])
    return

def ejecutar_programa():
  while num_cuadruplo[0] < len(lista_cuadruplos):
    switchCubo(lista_cuadruplos[num_cuadruplo[0]]) 
    num_cuadruplo[0] += 1

'''
 Funciones que guardan los valores del .txt en memoria
'''

leer_obj() # Leer .txt
ejecutar_programa() # Iniciar ejecución
