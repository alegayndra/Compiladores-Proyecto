from lectura import *
from globales import *
from copy import *

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
    elif dirValor[0] == 1: # Valor a asignas es local
      mapa_memoria[dirDestino[0]][dirDestino[1]][dirDestino[2]][destino - base_destino] = mapa_memoria[dirValor[0]][len(mapa_memoria[dirValor[0]]) - 1][dirValor[1]][dirValor[2]][valor - base_valor]
    else: # Ninguna es local
      mapa_memoria[dirDestino[0]][dirDestino[1]][dirDestino[2]][destino - base_destino] = mapa_memoria[dirValor[0]][dirValor[1]][dirValor[2]][valor - base_valor]

def escribe(valor):
  print(extraerMemoria(valor))

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

def goto(cuadruplo):
  num_cuadruplo[0] = cuadruplo - 1

def gotof(valor, cuadruplo):
  if not extraerMemoria(valor):
    # print("entramos")
    goto(cuadruplo)
  # else:
  #   print("No entramos")

def endfunc():
  mapa_memoria[1].pop()
  goto(pila_cuadruplos[len(pila_cuadruplos) - 1] + 1)
  pila_cuadruplos.pop()

def regresa(valor, destino):
  asignacion(valor, destino)
  endfunc()

def lee(valor):
  var = input("")
  leerValor(var, valor)

def era(funcion):
  memoriaFuncionEnProgreso.append(deepcopy(auxLocales))
  
  for i in range(len(funciones)):
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

      if cantVarsLocales[0][0] >= limitesVarsLocales[0][0] + dir_memoria[1][0][0]:
        print("Se excedió la memoria disponibles para enteros dentro del contexto local")
        sys.exit()
      if cantVarsLocales[0][1] >= limitesVarsLocales[0][1] + dir_memoria[1][0][1]:
        print("Se excedió la memoria disponibles para enteros termporales dentro del contexto local")
        sys.exit()
      if cantVarsLocales[1][0] >= limitesVarsLocales[1][0] + dir_memoria[1][1][0]:
        print("Se excedió la memoria disponibles para flotantes dentro del contexto local")
        sys.exit()
      if cantVarsLocales[1][1] >= limitesVarsLocales[1][1] + dir_memoria[1][1][1]:
        print("Se excedió la memoria disponibles para flotantes termporales dentro del contexto local")
        sys.exit()
      if cantVarsLocales[2][0] >= limitesVarsLocales[2][0] + dir_memoria[1][2][0]:
        print("Se excedió la memoria disponibles para caracter dentro del contexto local")
        sys.exit()
      if cantVarsLocales[2][1] >= limitesVarsLocales[2][1] + dir_memoria[1][2][1]:
        print("Se excedió la memoria disponibles para caracter termporales dentro del contexto local")
        sys.exit()

      memoriaFuncionEnProgreso[len(memoriaFuncionEnProgreso) - 1][0][0] = [None] * int(cntIntNormal)
      memoriaFuncionEnProgreso[len(memoriaFuncionEnProgreso) - 1][0][1] = [None] * int(cntIntTemp)
      memoriaFuncionEnProgreso[len(memoriaFuncionEnProgreso) - 1][1][0] = [None] * int(cntFloatNormal)
      memoriaFuncionEnProgreso[len(memoriaFuncionEnProgreso) - 1][1][1] = [None] * int(cntFloatTemp)
      memoriaFuncionEnProgreso[len(memoriaFuncionEnProgreso) - 1][2][0] = [None] * int(cntCharNormal)
      memoriaFuncionEnProgreso[len(memoriaFuncionEnProgreso) - 1][2][1] = [None] * int(cntCharTemp)
      return
    i += 1

def param(valor, parametro):
  val = extraerMemoria(valor)
  tipo = len(dir_memoria[1]) - 1
  while tipo >= 0:
    if parametro >= dir_memoria[1][tipo][0]:
      memoriaFuncionEnProgreso[len(memoriaFuncionEnProgreso) - 1][tipo][0][parametro - dir_memoria[1][tipo][0]] = val
      print('params', memoriaFuncionEnProgreso[len(memoriaFuncionEnProgreso) - 1][tipo][0][parametro - dir_memoria[1][tipo][0]])
    tipo -= 1
  
def gosub(cuad_funcion):
  mapa_memoria[1].append(deepcopy(memoriaFuncionEnProgreso[len(memoriaFuncionEnProgreso) - 1]))
  memoriaFuncionEnProgreso.pop()
  pila_cuadruplos.append(num_cuadruplo[0])
  goto(cuad_funcion)
def verificar(lim_inf, lim_sup, val_verificar):
  valor = extraerMemoria(val_verificar)
  if valor >= extraerMemoria(lim_inf) and valor <= extraerMemoria(lim_sup):
    return
  else:
    print("El subindice excede las dimensiones del arreglo")
    sys.exit()

def acceder(apuntador, destino):
  asignacion(extraerMemoria(apuntador), destino)
  return

extraerMemoria(extraerMemoria(t1)) = 1
def asginacionArreglo(valor, destino):
  asignacion(valor, extraerMemoria(destino))
  return

def switchCubo(cuadruplo):
  if cuadruplo[0] >= 0 and cuadruplo[0] <= 11:
    guardarValor(operacionNormal(cuadruplo[1], cuadruplo[2], cuadruplo[0]), cuadruplo[3])
    return
  elif cuadruplo[0] == 12: # Asignacion
    # print("Vamos a asignar")
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
  elif cuadruplo[0] == 23: # VER
    verificar(cuadruplo[2], cuadruplo[3], cuadruplo[1])
  elif cuadruplo[0] == 24: # ACC
    acceder(cuadruplo[1], cuadruplo[3])
  elif cuadruplo[0] == 25: # ASG
    asignacionArreglo(cuadruplo[1], cuadruplo[3])
    return

def ejecutar_programa():
  while num_cuadruplo[0] < len(lista_cuadruplos):
    # print("Ejecutamos el cuadruplo #", num_cuadruplo[0])
    # print(lista_cuadruplos[num_cuadruplo[0]])
    switchCubo(lista_cuadruplos[num_cuadruplo[0]]) 
    # print("Ahorita vamos en el cuadruplo --> ",num_cuadruplo)
    num_cuadruplo[0] += 1

'''
 Funciones que guardan los valores del .txt en memoria
'''

leer_obj()
ejecutar_programa()
