```markdown
# ONA WEB CONTROLLER ARCHITECTURE

## Documento: documentation/12_WEB_CONTROLLER_ARCHITECTURE.md

---

# 1. Introducción al controlador web ONA

El ONA Web Controller es el sistema que permite transformar cualquier dispositivo móvil moderno en un controlador inalámbrico universal para la plataforma ONA Gaming Studio.

A diferencia de los sistemas tradicionales donde el usuario debe instalar una aplicación específica, ONA utiliza una arquitectura basada en navegador que permite conectarse inmediatamente mediante un enlace o código QR.

El objetivo principal es eliminar la barrera de entrada:

- Sin instalación.
- Sin configuración compleja.
- Compatible con Android, iOS y otros dispositivos con navegador moderno.
- Acceso inmediato como controlador.
- Soporte para múltiples jugadores simultáneos.

El teléfono no funciona como una consola independiente, sino como un periférico inteligente conectado a ONA Core.

---

# 2. Filosofía de diseño

El controlador ONA sigue los siguientes principios:

| Principio | Descripción |
|-----------|-------------|
| Acceso inmediato | El usuario puede jugar sin descargar aplicaciones. |
| Universalidad | Compatible con cualquier dispositivo con navegador moderno. |
| Baja latencia | Comunicación optimizada para videojuegos. |
| Simplicidad | El usuario solo necesita escanear un QR. |
| Seguridad | Cada sesión utiliza autenticación temporal. |
| Escalabilidad | Soporte desde un jugador hasta múltiples jugadores. |

---

# 3. Arquitectura general

La arquitectura del controlador web está compuesta por tres elementos principales:

```

┌──────────────────────────────┐
│       DISPOSITIVO MÓVIL       │
│                              │
│  Navegador Web                │
│                              │
│  ONA Controller Client        │
│                              │
│  - UI Control                 │
│  - Joystick virtual           │
│  - Sensores                   │
│  - Voz                        │
│  - Gestión de conexión        │
└──────────────┬───────────────┘
│
│ WebRTC DataChannel
│
▼
┌──────────────────────────────┐
│        ONA CONTROLLER         │
│          GATEWAY              │
│                              │
│  - Validación de sesión       │
│  - Decodificación paquetes    │
│  - Control jugadores          │
└──────────────┬───────────────┘
│
▼
┌──────────────────────────────┐
│          ONA CORE             │
│                              │
│  - Input Manager              │
│  - Virtual Gamepad            │
│  - Game Runtime               │
└──────────────┬───────────────┘
│
▼
┌──────────────────────────────┐
│          VIDEOJUEGO           │
└──────────────────────────────┘

```

---

# 4. Compatibilidad de dispositivos

ONA Web Controller está diseñado para funcionar en:

## Navegadores compatibles

### Android

- Google Chrome.
- Microsoft Edge.
- Firefox.

### iOS

- Safari.
- Chrome basado en WebKit.

### Escritorio

- Chrome.
- Edge.
- Firefox.

---

# 5. Flujo de conexión

El proceso completo de conexión es:

```

Usuario inicia ONA en PC

```
    ↓
```

ONA Core genera sesión

```
    ↓
```

ONA genera código QR único

```
    ↓
```

Jugador escanea QR

```
    ↓
```

Navegador abre ONA Controller

```
    ↓
```

Usuario acepta permisos

```
    ↓
```

Se establece conexión segura

```
    ↓
```

Jugador recibe identidad

```
    ↓
```

Control disponible

```

---

# 6. Arquitectura interna del cliente web

El controlador web está dividido en módulos independientes.

```

ONA Controller Web

│
├── Client Core
│
├── UI Controller
│
├── Input Engine
│
├── Sensor Manager
│
├── Voice Module
│
├── Network Module
│
└── Device Capability Manager

```

---

# 7. UI Controller

Responsable de la interfaz visual del control.

Funciones:

- Mostrar botones virtuales.
- Mostrar joysticks.
- Mostrar información del jugador.
- Cambiar distribución del control.
- Adaptarse a orientación horizontal o vertical.

Modos:

## Horizontal

Diseñado para videojuegos tradicionales.

Incluye:

- Stick izquierdo.
- Stick derecho.
- D-Pad.
- Botones principales.
- Gatillos.

## Vertical

Diseñado para juegos casuales.

Incluye:

- Botones grandes.
- Gestos.
- Controles simplificados.

---

# 8. Input Engine

El Input Engine captura todas las acciones del jugador.

Tipos de entrada:

## Botones

Ejemplo:

```

A
B
X
Y
START
SELECT
L1
R1
L2
R2

```

## Joysticks

Valores:

```

X = -1.0 a 1.0

Y = -1.0 a 1.0

```

## Gestos

Soporta:

- Toques.
- Deslizamientos.
- Pulsación larga.
- Movimiento del dispositivo.

---

# 9. Sensor Manager

ONA puede utilizar sensores disponibles del dispositivo.

Sensores compatibles:

- Acelerómetro.
- Giroscopio.
- Orientación.
- Vibración.

Ejemplos:

Juegos de carreras:

```

Movimiento del teléfono
↓
Giroscopio
↓
Control de dirección

```

Juegos de movimiento:

```

Acelerómetro
↓
Evento del juego

```

El uso de sensores requiere autorización del usuario.

---

# 10. Voice Module

El navegador puede participar en comunicación de voz.

Arquitectura:

```

Micrófono móvil

```
    ↓
```

Captura local

```
    ↓
```

WebRTC Audio Channel

```
    ↓
```

ONA Voice Service

```
    ↓
```

Otros jugadores

```

Características:

- Comunicación en tiempo real.
- Bajo consumo de datos.
- Cancelación de ruido.
- Control de volumen.

---

# 11. Network Module

El módulo de red administra todas las comunicaciones.

Protocolos:

## Comunicación principal

WebRTC DataChannel

Características:

- Baja latencia.
- Comunicación segura.
- Cifrado DTLS.
- Comunicación directa cuando sea posible.

## Respaldo

WebSocket Seguro (WSS)

Uso:

- Redes restrictivas.
- Firewalls.
- Compatibilidad adicional.

---

# 12. Emparejamiento mediante QR

El QR contiene información temporal:

Ejemplo:

```

ONA SESSION

ID:
8A72F1D9

HOST:
LOCAL

TOKEN:
TEMPORAL

EXPIRATION:
60 seconds

```

El QR nunca contiene:

- Contraseñas.
- Datos personales.
- Licencias.
- Información permanente.

---

# 13. Gestión de jugadores múltiples

ONA soporta hasta:

```

10 jugadores simultáneos

```

Cada jugador recibe:

- ID único.
- Perfil temporal.
- Color identificador.
- Configuración independiente.

Ejemplo:

```

Jugador 1
Control Azul

Jugador 2
Control Rojo

Jugador 3
Control Verde

```

---

# 14. Seguridad

La seguridad del controlador web utiliza:

## Tokens temporales

Cada conexión recibe:

- Token único.
- Tiempo limitado.
- Asociación con una sesión.

## Validación de paquetes

ONA valida:

- Origen.
- Integridad.
- Sesión activa.
- Frecuencia de envío.

## Protección contra ataques

Incluye:

- Rate limiting.
- Paquetes inválidos descartados.
- Expiración automática.
- Bloqueo de conexiones sospechosas.

---

# 15. Rendimiento y latencia

Objetivos:

| Parámetro | Objetivo |
|-|-|
| Entrada local | <20 ms |
| Comunicación LAN | <10 ms |
| Reconexión | <1 segundo |
| Frecuencia controles | 60-120 Hz |

Optimización:

- Paquetes binarios.
- Envío diferencial.
- Eliminación de datos redundantes.
- Priorización de eventos críticos.

Ejemplo:

Movimiento del joystick:

```

Valor anterior:

X=0.2

Nuevo valor:

X=0.21

Enviar únicamente cambio

```

---

# 16. Manejo de desconexiones

Si un jugador pierde conexión:

```

Control pierde señal

```
    ↓
```

ONA detecta pérdida

```
    ↓
```

Mantiene sesión temporal

```
    ↓
```

Espera reconexión

```
    ↓
```

Jugador continúa

```

Tiempo objetivo:

```

< 1 segundo

```

---

# 17. Integración con ONA Runtime

El controlador web nunca interactúa directamente con el juego.

Flujo:

```

ONA Controller

```
    ↓
```

ONA Core

```
    ↓
```

Virtual Gamepad

```
    ↓
```

Juego

```

Ventajas:

- Mayor seguridad.
- Compatibilidad con cualquier juego.
- Independencia del motor gráfico.

---

# 18. Futuras extensiones

## Aplicaciones nativas opcionales

Aunque el navegador será la plataforma principal, en el futuro pueden existir aplicaciones nativas para:

- Menor latencia.
- Funciones avanzadas.
- Integración profunda.

## Controladores físicos ONA

Posible desarrollo de:

- Mandos Bluetooth.
- Accesorios oficiales.
- Controles especializados.

## Nuevos dispositivos

Compatibilidad futura:

- Smart TVs.
- Tablets.
- Wearables.
- Realidad aumentada.

---

# 19. Conclusión

ONA Web Controller representa uno de los pilares fundamentales del ecosistema ONA Gaming Studio.

Al utilizar el navegador como plataforma universal, ONA elimina la necesidad de hardware adicional y permite que cualquier persona pueda participar en una experiencia de consola utilizando únicamente:

- Una pantalla.
- Una computadora.
- Un teléfono.

El controlador deja de ser un accesorio físico y se convierte en un servicio universal disponible inmediatamente.

---

ONA Gaming Studio  
Web Controller Architecture v1.0  
Status: Approved for development
```
