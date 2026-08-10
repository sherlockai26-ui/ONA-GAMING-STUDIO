```markdown
# Modo de Visualización ONA

## 1. Introducción al modo de visualización

ONA Gaming Studio está diseñado para transformar una computadora convencional en una experiencia similar a una consola de videojuegos conectada a una televisión o monitor externo.

El sistema debe adaptarse automáticamente al entorno donde será utilizado, permitiendo al usuario disfrutar de una experiencia de consola sin importar si el dispositivo principal es una computadora de escritorio, laptop o equipo dedicado.

ONA soporta dos experiencias principales:

- **Modo Escritorio**: Diseñado para equipos utilizados como computadora tradicional.
- **Modo Consola**: Diseñado para televisores y pantallas externas, ofreciendo una interfaz optimizada para jugar desde distancia utilizando dispositivos móviles como controles.

El objetivo es que conectar una pantalla externa convierta automáticamente la computadora en una consola ONA, sin configuraciones complejas ni reinicios.

---

# 2. Detección automática de pantallas

ONA monitorea constantemente los eventos del sistema relacionados con conexiones y desconexiones de dispositivos de visualización.

La detección debe funcionar independientemente del tipo de conexión física utilizada.

## Tipos de conexión compatibles

| Conexión | Video | Audio | Compatibilidad |
|----------|-------|-------|----------------|
| HDMI | ✅ | ✅ | Recomendado |
| DisplayPort | ✅ | ✅ | Recomendado |
| USB-C Display Alt Mode | ✅ | ✅ | Compatible |
| VGA directo | ✅ | ❌ | Compatible con audio externo |
| VGA → HDMI activo | ✅ | Depende del adaptador | Compatible |
| DVI | ✅ | ❌ | Compatible con audio externo |

ONA no depende directamente del puerto físico, sino de la información proporcionada por el sistema operativo mediante los controladores gráficos.

---

# 3. Identificación de pantalla

Cuando una nueva pantalla es detectada, ONA obtiene información mediante el sistema operativo:

- Nombre del fabricante.
- Modelo del monitor o televisión.
- Resolución máxima.
- Frecuencia soportada.
- Tipo de conexión.
- Estado HDR (si existe).
- Capacidades de audio disponibles.

Ejemplos:

```

Pantalla detectada:

Nombre:
Samsung TV

Resolución:
3840x2160

Frecuencia:
60Hz

Audio:
HDMI Audio disponible

Modo recomendado:
Consola

```

---

# 4. Modo consola vs modo escritorio

## 4.1 Modo escritorio

Pensado para:

- Computadoras personales.
- Desarrollo.
- Configuración avanzada.
- Uso combinado trabajo/juego.

Características:

- Ventana tradicional.
- Acceso al sistema operativo.
- Teclado y mouse como métodos principales.
- Menús compactos.

---

## 4.2 Modo consola

Pensado para:

- Televisores.
- Monitores grandes.
- Salas de entretenimiento.

Características:

- Pantalla completa.
- Interfaz tipo consola.
- Navegación mediante controles móviles.
- Elementos grandes y visibles.
- Ocultamiento del escritorio del sistema operativo.
- Inicio automático de ONA Shell.

El usuario debe percibir la computadora como una consola dedicada.

---

# 5. Activación automática al conectar una TV

Cuando ONA detecta una pantalla externa:

1. Analiza las características del dispositivo.
2. Determina si corresponde a una televisión o monitor secundario.
3. Muestra una sugerencia:

```

Nueva pantalla detectada:

Samsung Smart TV
3840x2160 60Hz

¿Deseas activar Modo Consola?

```

Opciones:

- Activar siempre.
- Activar solo esta vez.
- No preguntar nuevamente.

---

# 6. Transición entre modos

La transición debe realizarse sin reiniciar el sistema.

Proceso:

```

Pantalla conectada
↓
ONA detecta dispositivo
↓
Analiza capacidades
↓
Ajusta resolución
↓
Configura audio
↓
Inicia interfaz consola
↓
Activa controles móviles
↓
Usuario listo para jugar

```

El cambio debe tardar menos de 5 segundos.

---

# 7. Configuración de resolución y frecuencia

ONA adapta automáticamente la interfaz según las capacidades de la pantalla.

Resoluciones soportadas:

- 1280x720
- 1920x1080
- 2560x1440
- 3840x2160 (4K)
- Resoluciones futuras superiores.

Configuración automática:

- Resolución nativa recomendada.
- Frecuencia máxima estable.
- Escalado de interfaz.
- Ajuste de tamaño de elementos.

Ejemplo:

```

TV detectada:

Resolución:
3840x2160

Interfaz:
Escala 150%

Frecuencia:
60Hz

HDR:
Disponible

```

---

# 8. Interfaz optimizada para televisión

La interfaz ONA Shell debe estar diseñada para verse desde varios metros de distancia.

Características:

- Fuentes grandes.
- Alto contraste.
- Botones amplios.
- Navegación simple.
- Animaciones fluidas.
- Información mínima necesaria.

Diseño:

```

+--------------------------------+
|                                |
|          Biblioteca             |
|                                |
|  [Juego 1] [Juego 2] [Juego 3] |
|                                |
|  [Juego 4] [Juego 5] [Juego 6] |
|                                |
+--------------------------------+

```

---

# 9. Ocultamiento del sistema operativo

En modo consola:

ONA debe ocultar la experiencia tradicional del sistema operativo.

Acciones:

- Ocultar escritorio.
- Ocultar barra de tareas.
- Bloquear notificaciones externas.
- Evitar ventanas emergentes.
- Mantener ONA Shell como interfaz principal.

El usuario debe interactuar únicamente con ONA.

---

# 10. Navegación con controlador móvil

El teléfono funciona como controlador principal.

Funciones:

- Navegación por menús.
- Selección de juegos.
- Control multimedia.
- Chat de voz.
- Gestión de usuarios.

La interfaz debe poder utilizarse completamente sin teclado ni mouse.

Ejemplo:

```

Teléfono:

↑ ↓ ← →
Mover selección

A:
Aceptar

B:
Regresar

HOME:
Abrir menú ONA

```

---

# 11. Audio en modo consola

ONA debe administrar correctamente la salida de audio dependiendo del tipo de conexión.

## HDMI / DisplayPort

Transmiten:

- Video.
- Audio digital.

Configuración:

```

ONA Audio Output:

HDMI TV
Activo

```

---

## VGA

VGA solamente transmite video.

Opciones:

1. Cable auxiliar 3.5 mm desde PC hacia TV.
2. Adaptador VGA → HDMI activo con entrada de audio.
3. Bocinas externas conectadas al equipo.

ONA detectará la salida disponible mediante el sistema operativo.

Ejemplo:

```

Pantalla:
VGA Adapter

Audio:
Realtek 3.5mm

Salida seleccionada:
Altavoces externos

```

---

# 12. Múltiples pantallas

ONA soporta configuraciones con varias pantallas.

Ejemplos:

## Laptop + TV

```

Laptop:
Modo escritorio

TV:
Modo consola

```

## PC + Monitor + TV

```

Monitor:
Configuración

TV:
Juego principal

```

El usuario puede seleccionar la pantalla principal desde configuración.

---

# 13. Experiencia de usuario en modo consola

Al iniciar:

```

Encender PC
↓
ONA inicia automáticamente
↓
Detecta TV
↓
Carga Shell
↓
Conecta controles móviles
↓
Usuario selecciona juego
↓
Jugar

```

El objetivo es eliminar pasos innecesarios.

---

# 14. Modo ahorro de energía

Cuando ONA detecta inactividad:

Puede activar:

- Reducción de brillo.
- Suspensión de interfaz.
- Pausa de animaciones.
- Reducción de consumo GPU.

Durante reproducción:

- Mantiene rendimiento máximo.
- Evita suspensión automática.

---

# 15. Compatibilidad con diferentes tamaños

ONA debe funcionar desde:

- Monitores pequeños.
- Laptops.
- Televisores de 32 pulgadas.
- Televisores 4K.
- Pantallas ultrapanorámicas.
- Proyectores.

La interfaz se adapta mediante escalado dinámico.

---

# 16. Futuras extensiones

## Proyectores

Soporte para:

- Salas de juego.
- Eventos.
- Experiencias multijugador.

## Pantallas táctiles

Permitir:

- Navegación directa.
- Juegos híbridos.
- Interfaces secundarias.

## Realidad aumentada y realidad virtual

Preparación futura para:

- Cascos VR.
- Pantallas inmersivas.
- Controladores avanzados.

---

# 17. Arquitectura técnica

## Linux

Detección:

- udev.
- DRM/KMS.
- X11.
- Wayland.

## Windows

Detección:

- WM_DISPLAYCHANGE.
- DirectX Display APIs.

## Audio

Detección:

- HDMI Audio.
- DisplayPort Audio.
- Dispositivos ALSA/WASAPI.

---

# 18. Principios del modo visualización ONA

ONA debe cumplir:

- Detectar automáticamente cualquier pantalla compatible.
- No depender del tipo de cable utilizado.
- Priorizar HDMI y DisplayPort para experiencia completa.
- Mantener compatibilidad con hardware antiguo.
- Convertir cualquier PC compatible en una consola.
- Proporcionar una experiencia simple para cualquier usuario.

---

# Conclusión

El sistema de visualización de ONA es una pieza fundamental para cumplir la visión de convertir cualquier computadora en una consola universal.

La conexión de una pantalla externa debe ser suficiente para transformar la experiencia: conectar, encender, seleccionar y jugar.

ONA elimina la barrera entre una PC tradicional y una consola de videojuegos.

---

*ONA Gaming Studio - Display Mode Architecture v1.0*  
*Status: Approved for development*  
*Fecha: 2026-08-03*
```
