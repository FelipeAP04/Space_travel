# 🚀 Sistema Solar Completo - Renderizador 3D Avanzado

Un simulador interactivo del sistema solar desarrollado en Rust con capacidades avanzadas de renderizado 3D, navegación en tiempo real y efectos visuales espectaculares.

## 📹 Video Demostración
[Ver demostración en YouTube](https://youtube.com/shorts/oW5Qp7X-HVU)

## ✨ Características Implementadas

### 🌌 Sistema Solar Realista
- **Sol central**: Estrella con efectos de emisión y pulsaciones
- **6 planetas únicos**:
  - Mercurio - planeta rocoso cercano al sol
  - Venus - planeta dorado con atmósfera densa
  - Tierra - planeta azul con características terrestres
  - Marte - planeta rojo con superficie árida
  - Júpiter - gigante gaseoso con bandas atmosféricas
  - Luna - satélite natural orbitando la Tierra

### 🎮 Sistema de Cámara Avanzado (40 puntos)
#### Modo Orbital (por defecto)
- **Teclas de dirección**: Rotar alrededor del objetivo
- **W/S**: Acercar/Alejar zoom
- **C**: Cambiar a modo libre

#### Modo Libre 3D
- **WASD**: Movimiento en plano horizontal
- **Espacio**: Subir
- **Shift Izq**: Bajar
- **Direccionales**: Rotar vista
- **C**: Regresar a modo orbital

### ⚡ Teletransporte Instantáneo (10 + 10 puntos)
- **Tecla 0**: Teletransporte al Sol
- **Tecla 1**: Teletransporte a Mercurio
- **Tecla 2**: Teletransporte a Venus
- **Tecla 3**: Teletransporte a la Tierra
- **Tecla 4**: Teletransporte a Marte
- **Tecla 5**: Teletransporte a Júpiter
- **Efectos de animación**: Transiciones suaves entre ubicaciones

### 🛸 Nave Espacial Siguiendo la Cámara (30 puntos)
- Nave 3D modelada que siempre acompaña al jugador
- Posicionada estratégicamente en frente y ligeramente abajo de la cámara
- Rotación automática para mantener orientación con la vista
- Shader metálico especializado con efectos de desgaste

### 🌟 Fondo Estelar (Skybox) (10 puntos)
- Campo de estrellas procedural en 3D
- Estrellas generadas dinámicamente en todas las direcciones
- Efecto de profundidad infinita

### 🔄 Visualización de Órbitas (20 puntos)
- **Tecla O**: Mostrar/ocultar trayectorias orbitales
- Líneas orbitales para todos los planetas principales
- Animación sutil y colores distintivos
- Renderizado en tiempo real con efectos de pulsación

### 🚫 Detección de Colisiones (10 puntos)
- Sistema de colisión que previene que la cámara/nave atraviese planetas
- Margen de seguridad automático basado en el tamaño de cada cuerpo celeste
- Empuje automático fuera de zonas de colisión

### 🎨 Shaders Creativos Avanzados
#### Shader de Estrella
- Gradiente de temperatura del núcleo a la superficie
- Efectos de pulsación temporal
- Capas de actividad solar y llamaradas

#### Shader de Planeta Rocoso
- Patrones de terreno con cráteres
- Variación de altitudes (tierras altas/bajas)
- Efectos de superficie realistas

#### Shader de Gigante Gaseoso
- Bandas atmosféricas dinámicas basadas en latitud
- Sistemas de tormentas y turbulencias
- Variación de composición atmosférica
- Coloración estilo Júpiter

#### Shader de Nave Espacial
- Apariencia metálica con detalles de paneles
- Patrones de desgaste y envejecimiento
- Efectos de luces de motor pulsantes

#### Shader de Órbitas
- Líneas translúcidas con efectos de pulsación
- Gradiente de distancia para mejor visibilidad

## 🏆 Puntuación del Proyecto

| Criterio | Puntos | Estado |
|----------|---------|--------|
| **Estética del sistema completo** | 30 | ✅ Completado |
| **Performance apropiado** | 20 | ✅ Optimizado |
| **Planetas/estrellas/lunas (5x10)** | 50 | ✅ 6 cuerpos celestes |
| **Teletransporte instantáneo** | 10 | ✅ Sistema de warp |
| **Animación de teletransporte** | 10 | ✅ Efectos visuales |
| **Nave siguiendo cámara** | 30 | ✅ Nave 3D completa |
| **Skybox con estrellas** | 10 | ✅ Campo estelar |
| **Detección de colisiones** | 10 | ✅ Sistema completo |
| **Movimiento 3D de cámara** | 40 | ✅ Dos modos |
| **Órbitas renderizadas** | 20 | ✅ Visualización |
| **TOTAL** | **230** | ✅ **COMPLETO** |

## 🎯 Controles Completos

### Navegación
```
Teclas de dirección  → Mover cámara orbital / Rotar vista libre
WASD                → Zoom orbital / Movimiento libre
Espacio            → Subir (modo libre)
Shift Izquierdo    → Bajar (modo libre)
C                  → Alternar modo cámara
```

### Teletransporte
```
0 → Sol      3 → Tierra
1 → Mercurio  4 → Marte  
2 → Venus     5 → Júpiter
```

### Visualización
```
O → Mostrar/ocultar órbitas
ESC → Salir del programa
```

## 🔧 Instalación y Ejecución

### Prerrequisitos
- Rust (versión estable más reciente)
- Git

### Instalación
```bash
git clone https://github.com/FelipeAP04/Space_travel.git
cd Space_travel
```

### Ejecutar
```bash
cargo run --release
```

## 🚀 Características Técnicas

- **Lenguaje**: Rust
- **Renderizado**: Software rendering con shaders procedurales
- **Matemáticas**: nalgebra-glm para operaciones 3D
- **Ventana**: minifb para gestión de ventana y entrada
- **Modelo 3D**: Carga de archivos .obj con tobj

## 🌌 Referencias Culturales

- **Luna Basketboll**: Inspirada en la icónica luna de basketball de "Regular Show"
- **Gigante Gaseoso**: Basado en el estilo visual de "Treasure Planet"

## 📸 Capturas del Sistema

Para completar la documentación, ejecutar el programa y tomar las siguientes capturas:

1. **Vista general del sistema solar completo** - mostrando todos los planetas en órbita
2. **Sol con efectos de shader** - destacando las pulsaciones y efectos de emisión
3. **Planetas rocosos** - mostrando detalles de superficie y crateres
4. **Gigante gaseoso** - exhibiendo bandas atmosféricas
5. **Luna de basketball** - la característica luna única
6. **Nave espacial** - en diferentes ángulos siguiendo la cámara
7. **Modo libre 3D** - demostrando la navegación libre
8. **Órbitas visibles** - mostrando las trayectorias orbitales
9. **Teletransporte** - efectos de transición entre planetas

## 🎥 Video Demostrativo

*[Aquí se incluirá un enlace al video que muestra la cámara explorando todo el sistema solar]*

## 📜 Licencia

Proyecto académico desarrollado para el curso de Gráficos por Computadora.

---

**¡Explora el cosmos desde la comodidad de tu computadora! 🌌✨**