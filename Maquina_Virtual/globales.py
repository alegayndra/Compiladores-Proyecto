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

funciones = []

memoriaFuncionEnProgreso = [0]

auxLocales = [ # Locales
    [ # Enteras
      # Normales  Temporales
      [],         []
    ],
    [ # Flotantes
      # Normales  Temporales
      [],         []
    ],
    [ # Caracteres
      # Normales  Temporales
      [],         [] 
    ]
]

pila_cuadruplos = []

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

lista_cuadruplos = []
num_cuadruplo = [0]
