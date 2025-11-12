# Sistema Solar - Laboratorio de Shaders Creativos# Solar System - Creative Shader Lab# Solar System - Creative Shader Lab



Un renderizador 3D por software que muestra cuerpos celestes creativos usando únicamente shaders procedurales - sin texturas ni materiales externos.



## Descripción del ProyectoA 3D software renderer showcasing creative celestial bodies using only procedural shaders - no textures or external materials.A 3D software renderer showcasing creative celestial bodies using only procedural shaders - no textures or external materials.



Este proyecto implementa un sistema solar completo con tres cuerpos celestes distintos, cada uno con shaders procedurales únicos que crean efectos visuales realistas y creativos a través de algoritmos matemáticos.



## Cuerpos Celestes## Project Overview## Features



### 1. Estrella (Sol) ⭐

**Complejidad del Shader: 4 Capas**

- **Capa 1**: Gradiente de temperatura del núcleo basado en la distancia desde el centroThis project implements a complete solar system with three distinct celestial bodies, each featuring unique procedural shaders that create realistic and creative visual effects through mathematical algorithms.- ✨ **Multi-Model Rendering**: Support for different 3D models simultaneously

- **Capa 2**: Efecto de pulsación usando ondas sinusoidales basadas en tiempo para simular actividad solar realista

- **Capa 3**: Zonas de temperatura creando núcleo blanco caliente a superficie roja más fría- 🌌 **Complete Solar System**: Sun, planets, and moons with realistic orbital mechanics

- **Capa 4**: Llamaradas solares y actividad superficial usando ruido multidimensional

## Celestial Bodies- 🎥 **Interactive Camera**: 3D camera controls with rotation and zoom

**Características Visuales:**

- Núcleo blanco-amarillo brillante (simulación de temperatura >5500K)- � **Animated Skybox**: Dynamic starfield background with twinkling stars

- Capa media naranja (3000-5500K)

- Capa externa roja (<3000K)### 1. Star (Sun) ⭐- ☀️ **Sun-Based Lighting**: Realistic lighting system with the sun as light source

- Efecto de pulsación dinámico simulando actividad solar

- Auto-iluminado (no recibe iluminación de otras fuentes)**Shader Complexity: 4 Layers**- ⚙️ **Parametrized Rendering Pipeline**: Each model has its own transformation matrix



### 2. Planeta Rocoso (Terrestre) 🌍- **Layer 1**: Core temperature gradient based on distance from center- 🎨 **Shader System**: Parametrized shaders supporting multiple render modes

**Complejidad del Shader: 4 Capas**

- **Capa 1**: Altura del terreno base usando ruido basado en posición para elevación- **Layer 2**: Pulsing effect using time-based sine waves for realistic solar activity

- **Capa 2**: Patrones de cráteres simulando geología de impacto

- **Capa 3**: Venas minerales y variación de composición superficial- **Layer 3**: Temperature zones creating hot white core to cooler red surface## Solar System Components

- **Capa 4**: Cálculos de rugosidad superficial basados en vectores normales

- **Layer 4**: Solar flares and surface activity using multi-dimensional noise

**Características Visuales:**

- Áreas montañosas ricas en hierro (coloración marrón-rojiza)### ☀️ **Sun** (Center)

- Terreno montañoso gris-marrón

- Regiones más oscuras de tierras bajas y cráteres de impacto**Visual Features:**- **Model**: Planet.obj (large scale)

- Mapeo realista de elevación del terreno

- Cálculos completos de iluminación plana- Bright white-yellow core (>5500K temperature simulation)- **Color**: Gold (#FFD700)



### 3. Gigante Gaseoso (Joviano) - Planeta del Tesoro 🪐- Orange mid-layer (3000-5500K)- **Behavior**: Self-illuminated light source with rotation

**Complejidad del Shader: 4 Capas**

- **Capa 1**: Bandas atmosféricas basadas en latitud (bandas estilo Júpiter)- Red outer layer (<3000K) 

- **Capa 2**: Sistemas de tormentas y patrones de turbulencia

- **Capa 3**: Variación de composición gaseosa afectando la distribución de color- Dynamic pulsing effect simulating solar activity### 🌍 **Planet** 

- **Capa 4**: Efectos de profundidad atmosférica para renderizado volumétrico realista

- Self-illuminated (doesn't receive lighting from other sources)- **Model**: Planet.obj (smaller scale)

**Características Visuales:**

- Inspirado en el **Planeta del Tesoro** de Disney- **Color**: Royal Blue (#4169E1)

- Zonas atmosféricas de color crema/blanco claro

- Regiones de cinturones marrones oscuros### 2. Rocky Planet (Terrestrial) 🌍- **Orbit**: Medium distance from sun

- Áreas de tormenta rojizas (inspiradas en la Gran Mancha Roja)

- Turbulencia atmosférica dinámica**Shader Complexity: 4 Layers**- **Lighting**: Receives dynamic lighting from sun

- Paleta de colores realista de gigante gaseoso

- **Layer 1**: Base terrain height using position-based noise for elevation- **Moon**: Has a moon orbiting around it

### 4. Luna Rocosa - Basketball de Un Show Más 🌙

**Características Visuales:**- **Layer 2**: Crater patterns simulating impact geology

- Inspirada en la **Luna de Basketball** de "Un Show Más" (Regular Show)

- Usa el mismo shader del planeta rocoso con parámetros diferentes- **Layer 3**: Mineral veins and surface composition variation### 🌙 **Moon**

- Patrones de cráteres a menor escala

- Coloración gris-marrón típica de cuerpos sin atmósfera- **Layer 4**: Surface roughness calculations based on normal vectors- **Model**: basketmoon.obj

- Orbita alrededor del Planeta Rocoso

- **Color**: Silver (#C0C0C0)

## Implementación Técnica

**Visual Features:**- **Behavior**: Orbits around the Planet

### Sistema de Iluminación

Implementa **Flat Shading** como se especifica en los requerimientos:- Iron-rich highland areas (reddish-brown coloring)- **Lighting**: Shows lunar phases based on sun position

- Calcula normales de triángulo usando producto cruz de vectores de borde

- Fuente de luz puntual (Sol) en el centro del sistema- Grayish-brown highland terrain

- Dirección de luz calculada desde posición de luz hasta centro del triángulo

- Intensidad = dot(normal, lightDirection) con limitación apropiada- Darker lowland and impact crater regions### 🪐 **Third Planet**

- Fragment shader aplica intensidad al color según referencia: `fragment.color * fragment.intensity`

- Realistic terrain elevation mapping- **Model**: trasureP.obj

### Pipeline de Gráficos

1. **Vertex Shader**: Transformación Modelo → Mundo → Vista → Clip → Espacio de pantalla- Full flat-shading lighting calculations- **Color**: Blue Violet (#8A2BE2)

2. **Rasterización**: Rasterización de triángulos con coordenadas baricéntricas

3. **Fragment Shader**: Aplica intensidad de iluminación y efectos procedurales- **Orbit**: Outer orbit, slower movement

4. **Prueba de Profundidad**: Z-buffer para renderizado 3D correcto

### 3. Gas Giant (Jovian) 🪐- **Lighting**: Distance-based illumination from sun

### Arquitectura de Shaders

Cada cuerpo celeste usa un `ShaderType` específico:**Shader Complexity: 4 Layers**

- `ShaderType::Star` - Renderizado basado en emisión para el sol

- `ShaderType::RockyPlanet` - Características de terreno y geología- **Layer 1**: Atmospheric bands based on latitude (Jupiter-like banding)### 🌌 **Skybox**

- `ShaderType::GasGiant` - Efectos atmosféricos y gaseosos

- **Layer 2**: Storm systems and turbulence patterns- **Type**: Large sphere containing the solar system

### Sistema de Cámara

- **Cámara LookAt**: Orbita alrededor del centro del sistema solar- **Layer 3**: Gas composition variation affecting color distribution- **Effect**: Procedural starfield with twinkling animation

- **Coordenadas Esféricas**: Movimiento orbital suave

- **Controles**: Teclas de dirección para orbitar, A/S para zoom- **Layer 4**: Atmospheric depth effects for realistic volume rendering- **Features**: 

- **Proyección**: Proyección perspectiva con transformación de viewport apropiada

  - Pseudo-random star distribution

## Paleta de Colores

**Visual Features:**  - Time-based twinkling effect

### Fondo

- **Púrpura Oscuro del Espacio**: `#2D1B69` - Crea un ambiente atmosférico del espacio profundo- Light cream/white atmospheric zones  - Subtle space color variation



### Cuerpos Celestes- Dark brown belt regions  - No lighting influence (stars are self-illuminated)

- **Sol**: Dorado/blanco/naranja/rojo dinámico basado en simulación de temperatura

- **Planeta Rocoso**: Marrones, grises y rojo-hierro basados en características geológicas- Reddish storm areas (Great Red Spot inspired)

- **Gigante Gaseoso**: Colores crema, marrón y tormentas rojizas (inspirado en Júpiter/Planeta del Tesoro)

- **Luna**: Superficie rocosa gris-marrón (Luna de Basketball)- Dynamic atmospheric turbulence## Controls



## Requerimientos Cumplidos- Realistic gas giant color palette



✅ **3 Cuerpos Celestes Requeridos**: Estrella, Planeta Rocoso, Gigante Gaseoso  - **Arrow Keys**: Orbit camera around the sun (horizontal and vertical movement)

✅ **Renderizado Solo con Shaders**: No se usaron texturas ni materiales externos  

✅ **Implementación de Flat Shading**: Siguiendo la referencia proporcionada exactamente  ### 4. Rocky Moon 🌙- **A/S**: Zoom in/out (change distance from the sun)

✅ **Diseño Creativo**: 4+ capas por shader para máxima complejidad  

✅ **Fondo**: Ambiente espacial púrpura oscuro  **Visual Features:**- **Escape**: Exit

✅ **Mecánica Orbital**: Rotación y traslación implementadas  

✅ **Características Adicionales**: Luna rocosa orbitando el planeta terrestre  - Uses the same Rocky Planet shader with different parameters



## Desglose de Complejidad de Shaders- Smaller scale crater patterns### Camera System



| Cuerpo Celeste | Capas | Características de Complejidad |- Grayish-brown coloration typical of airless bodiesThe camera uses a **LookAt system** that always focuses on the center of the sun:

|-----------------|-------|---------------------------------|

| Estrella | 4 | Gradiente temperatura + Pulsación + Zonas + Actividad solar |- Orbits around the Rocky Planet- **Eye Position**: Camera position in 3D space (calculated from spherical coordinates)

| Planeta Rocoso | 4 | Altura terreno + Cráteres + Minerales + Rugosidad superficie |

| Gigante Gaseoso | 4 | Bandas atmosféricas + Tormentas + Composición + Profundidad |- **Target**: Always pointing at the sun's center

| **Total** | **12** | **Máxima complejidad lograda** |

## Technical Implementation- **Orbital Movement**: Camera moves in spherical coordinates around the sun

## Controles

- **Smooth Controls**: Natural orbital camera movement with elevation constraints

- **Teclas de Dirección**: Orbitar cámara alrededor del sistema solar

- **A**: Acercar al centro### Lighting System

- **S**: Alejar del centro  

- **Escape**: Salir de la aplicaciónImplements **Flat Shading** as specified in the requirements:## Technical Implementation



## Especificaciones Técnicas- Calculates triangle normals using cross product of edge vectors



- **Lenguaje**: Rust- Point light source (Sun) at system center### Parametrized Shader System

- **Gráficos**: Renderizador por software personalizado (sin OpenGL/Vulkan)

- **Resolución**: 800x600 píxeles- Light direction calculated from light position to triangle centerFollowing the recommended approach for multiple model rendering with shader parametrization:

- **Framework**: minifb para ventanas, nalgebra-glm para matemáticas

- **Renderizado**: Pipeline 3D completo con Z-buffering- Intensity = dot(normal, lightDirection) with proper clamping



## Instalación y Ejecución- Fragment shader applies intensity to color as per reference: `fragment.color * fragment.intensity````rust



```bashpub enum ShaderType {

# Clonar repositorio

git clone https://github.com/FelipeAP04/Space_travel.git### Graphics Pipeline    Skybox,    // For starfield background

cd Space_travel

1. **Vertex Shader**: Model → World → View → Clip → Screen space transformation    Planet,    // For celestial bodies with lighting

# Ejecutar la aplicación

cargo run --release2. **Rasterization**: Triangle rasterization with barycentric coordinates}

```

3. **Fragment Shader**: Applies lighting intensity and procedural effects

## Estructura del Proyecto

4. **Depth Testing**: Z-buffer for proper 3D renderingpub struct Uniforms {

```

src/    model_matrix: Mat4,

├── main.rs           # Aplicación principal y definiciones de cuerpos celestes

├── shaders.rs        # Todas las implementaciones de shaders e iluminación### Shader Architecture    light_position: Vec3,

├── triangle.rs       # Rasterización y cálculos de flat shading  

├── framebuffer.rs    # Buffer de pantalla y pruebas de profundidadEach celestial body uses a specific `ShaderType`:    is_light_source: bool,

├── camera.rs         # Sistema de cámara LookAt

├── vertex.rs         # Estructura de vértices y transformaciones- `ShaderType::Star` - Emission-based rendering for the sun    shader_type: ShaderType,  // Determines shader behavior

├── fragment.rs       # Estructura de fragmentos con intensidad de iluminación

├── color.rs          # Utilidades y operaciones de color- `ShaderType::RockyPlanet` - Terrain and geological features    time: f32,               // For animations

└── obj.rs           # Utilidades de carga de modelos 3D

```- `ShaderType::GasGiant` - Atmospheric and gas effects}



## Referencias Culturales```



- **Luna de Basketball**: Inspirada en el episodio "The Power" de Regular Show donde la luna se convierte en una pelota de basketball### Camera System

- **Planeta del Tesoro**: El gigante gaseoso está inspirado en la estética del planeta Montressor de la película "Treasure Planet" de Disney

- **LookAt Camera**: Orbits around the solar system center### Lighting System

Este proyecto demuestra técnicas avanzadas de shaders procedurales manteniendo la restricción de renderizado solo por software sin texturas o materiales externos.

- **Spherical Coordinates**: Smooth orbital movement- **Sun**: Self-illuminated, always bright (avoids trapped light problem)

## Capturas de Pantalla

- **Controls**: Arrow keys for orbit, A/S for zoom- **Other Objects**: Receive dynamic lighting from sun position

*[Las imágenes se agregaran después de tomar las capturas del sistema solar completo y cada planeta individual]*

- **Projection**: Perspective projection with proper viewport transformation- **Distance Attenuation**: Objects farther from sun appear dimmer

### Sistema Solar Completo

![Sistema Solar Completo](screenshots/solar_system_complete.png)- **Surface Angle**: Lambert diffuse lighting based on surface normals



### Estrella (Sol)## Color Palette

![Sol](screenshots/sun.png)

### LookAt Camera Implementation

### Planeta Rocoso

![Planeta Rocoso](screenshots/rocky_planet.png)### BackgroundThe camera system implements the OpenGL LookAt function with the following components:



### Gigante Gaseoso (Planeta del Tesoro)- **Dark Purple Space**: `#2D1B69` - Creates an atmospheric deep space environment

![Gigante Gaseoso](screenshots/gas_giant.png)

```rust

### Luna de Basketball

![Luna](screenshots/basketball_moon.png)### Celestial Bodies// LookAt matrix calculation (from camera.rs)

- **Sun**: Dynamic gold/white/orange/red based on temperature simulationpub fn look_at_matrix(eye: Vec3, at: Vec3, up: Vec3) -> Mat4 {

- **Rocky Planet**: Browns, grays, and iron-red based on geological features      let mut zaxis = normalize(&(at - eye));    // Forward vector

- **Gas Giant**: Cream, brown, and reddish storm colors (Jupiter-inspired)    let xaxis = normalize(&cross(&zaxis, &up)); // Right vector  

- **Moon**: Grayish-brown rocky surface    let yaxis = cross(&xaxis, &zaxis);         // Up vector

    zaxis = -zaxis; // Right-handed coordinate system

## Requirements Fulfilled    

    // Create view matrix (inverse transformation)

✅ **3 Required Celestial Bodies**: Star, Rocky Planet, Gas Giant      Mat4::new(

✅ **Shader-Only Rendering**: No textures or external materials used          xaxis.x, xaxis.y, xaxis.z, -dot(&xaxis, &eye),

✅ **Flat Shading Implementation**: Following provided reference exactly          yaxis.x, yaxis.y, yaxis.z, -dot(&yaxis, &eye),

✅ **Creative Design**: 4+ layers per shader for maximum complexity          zaxis.x, zaxis.y, zaxis.z, -dot(&zaxis, &eye),

✅ **Background**: Dark purple space environment          0.0,     0.0,     0.0,     1.0,

✅ **Orbital Mechanics**: Rotation and translation implemented      )

✅ **Additional Features**: Rocky moon orbiting the terrestrial planet  }

```

## Shader Complexity Breakdown

**Camera Features:**

| Celestial Body | Layers | Complexity Features |- **Always focuses on sun**: Target is fixed to the solar system center

|----------------|--------|-------------------|- **Spherical coordinates**: Position calculated using distance, theta (azimuth), phi (elevation)

| Star | 4 | Temperature gradient + Pulsing + Zones + Solar activity |- **View matrix**: Proper 3D transformation from world to camera space

| Rocky Planet | 4 | Terrain height + Craters + Minerals + Surface roughness |- **Right-handed system**: Consistent with OpenGL conventions

| Gas Giant | 4 | Atmospheric bands + Storms + Composition + Depth |

| **Total** | **12** | **Maximum complexity achieved** |### Rendering Pipeline Enhancement

1. **Model Matrix**: Transforms object from local to world space

## Controls2. **View Matrix**: Transforms from world to camera space (LookAt)

3. **Shader Processing**: Objects rendered with proper camera perspective

- **Arrow Keys**: Orbit camera around solar system

- **A**: Zoom in closer to center## Running the Project

- **S**: Zoom out from center  

- **Escape**: Exit application```bash

cd solar_system

## Technical Specificationscargo run --release

```

- **Language**: Rust

- **Graphics**: Custom software renderer (no OpenGL/Vulkan)## Project Structure

- **Resolution**: 800x600 pixels

- **Framework**: minifb for windowing, nalgebra-glm for mathematics- `src/main.rs`: Enhanced main application with multi-model support

- **Rendering**: Complete 3D pipeline with Z-buffering- `src/camera.rs`: LookAt camera system with spherical coordinate movement

- `src/shaders.rs`: Parametrized shader system with skybox and planet shaders

## Installation and Running- `src/skybox.rs`: Skybox geometry generation (sphere and cube options)

- `src/framebuffer.rs`: Frame buffer implementation

```bash- `src/vertex.rs`: Vertex data structures

# Clone repository- `src/obj.rs`: OBJ model loader

git clone [repository-url]- `assets/models/`: Multiple 3D model files

cd solar_system

## Architecture Highlights

# Run the application

cargo run --release- **Parametrized Shaders**: Switch between skybox and planet rendering modes

```- **Skybox System**: Procedural star generation with animation

- **Multiple Model Support**: Different OBJ files for varied celestial bodies

## Project Structure- **Advanced Lighting**: Sun-based illumination with realistic effects

- **Camera System**: Interactive 3D camera with smooth controls

```- **Orbital Mechanics**: Realistic planetary and lunar movement patterns
src/
├── main.rs           # Main application and celestial body definitions
├── shaders.rs        # All shader implementations and lighting
├── triangle.rs       # Rasterization and flat shading calculations  
├── framebuffer.rs    # Screen buffer and depth testing
├── camera.rs         # LookAt camera system
├── vertex.rs         # Vertex structure and transformations
├── fragment.rs       # Fragment structure with lighting intensity
├── color.rs          # Color utilities and operations
└── obj.rs           # 3D model loading utilities
```

This project demonstrates advanced procedural shader techniques while maintaining the constraint of software-only rendering without external textures or materials.