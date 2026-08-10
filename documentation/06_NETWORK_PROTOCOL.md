# Protocolo de Red ONA

## 1. Introducción

Este documento define la arquitectura de comunicación de red utilizada por **ONA Gaming Studio**, estableciendo cómo interactúan los diferentes componentes del ecosistema:

* ONA Host (PC donde se ejecutan los juegos).
* ONA Controller (dispositivo móvil utilizado como periférico inteligente).
* ONA Backend (servicios en la nube).
* ONA Developer Platform.
* ONA Store.
* Servicios de comunicación en tiempo real.

La red de ONA está diseñada bajo un principio fundamental:

> **La comunicación relacionada con el juego debe tener la menor latencia posible, mientras que los servicios administrativos deben priorizar confiabilidad y seguridad.**

ONA separa las comunicaciones según su propósito:

* **Tiempo real:** Controladores, voz y eventos de juego.
* **Servicios:** Cuentas, tienda, matchmaking, guardados y actualizaciones.
* **Administración:** Telemetría, análisis y soporte.

---

# 2. Principios de diseño

La arquitectura de red de ONA sigue los siguientes principios:

| Principio               | Descripción                                                                        |
| ----------------------- | ---------------------------------------------------------------------------------- |
| Baja latencia           | Los controles deben responder en tiempo real.                                      |
| Seguridad               | Toda comunicación sensible debe estar cifrada.                                     |
| Escalabilidad           | La arquitectura debe funcionar desde una red doméstica hasta millones de usuarios. |
| Tolerancia a fallos     | Las desconexiones deben recuperarse automáticamente.                               |
| Eficiencia              | Reducir consumo de ancho de banda y procesamiento.                                 |
| Separación de servicios | Cada comunicación utiliza el protocolo adecuado.                                   |

---

# 3. Arquitectura de red general

```
                         INTERNET
                             |
                             |
                    ┌─────────────────┐
                    │   ONA CLOUD     │
                    │                 │
                    │ Accounts        │
                    │ Store           │
                    │ Matchmaking     │
                    │ Voice           │
                    │ Cloud Saves     │
                    └────────┬────────┘
                             |
                           TLS 1.3
                             |
                             |
        ┌────────────────────┴────────────────────┐
        │                                         │
        ▼                                         ▼

┌──────────────────┐                    ┌──────────────────┐
│   ONA HOST PC    │                    │ Developer Tools  │
│                  │                    │                  │
│ ONA Core         │                    │ SDK              │
│ ONA Runtime      │                    │ Publisher        │
│ ONA Shell        │                    │ Analytics        │
└────────┬─────────┘                    └──────────────────┘
         |
         |
      LAN/WiFi
         |
         |
 ┌───────┴──────────────────────────────┐
 │                                      │
 ▼                                      ▼

ONA Controller 1                  ONA Controller 10

UDP Binary                         UDP Binary
<10ms latency                      <10ms latency
```

---

# 4. Capas de comunicación

ONA utiliza diferentes protocolos dependiendo del tipo de información.

| Comunicación       | Protocolo      | Uso                      |
| ------------------ | -------------- | ------------------------ |
| Control móvil-PC   | UDP Binario    | Entrada de controles     |
| Voz                | WebRTC + DTLS  | Comunicación de voz      |
| Descubrimiento LAN | mDNS / Bonjour | Encontrar dispositivos   |
| Emparejamiento     | QR + Token     | Vinculación segura       |
| Servicios nube     | HTTPS/TLS      | Datos administrativos    |
| Tiempo real nube   | WebSocket      | Eventos en vivo          |
| Descargas          | HTTPS/TCP      | Juegos y actualizaciones |

---

# 5. Comunicación local LAN

La comunicación dentro de una red doméstica es el núcleo de la experiencia ONA.

## Objetivo

Permitir que hasta 10 teléfonos funcionen como controles simultáneos con latencia mínima.

## Protocolo principal

**UDP Binario**

Características:

* Sin negociación constante.
* Paquetes pequeños.
* Baja sobrecarga.
* Prioridad sobre otros servicios.

Ejemplo:

```
ONA Controller
      |
      |
 UDP Packet
      |
      ▼
ONA Core Receiver
      |
      |
Input Translation
      |
      ▼
Videojuego
```

---

# 6. Comunicación con servicios en la nube

Los servicios externos utilizan protocolos orientados a seguridad y confiabilidad.

## HTTPS / TLS 1.3

Utilizado para:

* Inicio de sesión.
* Gestión de cuentas.
* Compras.
* Licencias.
* Descargas.
* Actualizaciones.

Ejemplo:

```
ONA Host

    HTTPS

      |

ONA Cloud API
```

---

## WebSocket

Utilizado para comunicación persistente:

* Estado de amigos.
* Invitaciones.
* Eventos.
* Matchmaking.
* Notificaciones.

---

# 7. Descubrimiento automático de dispositivos

ONA utiliza **mDNS / Bonjour** para detectar dispositivos dentro de la red local sin configuración manual.

## Funcionamiento

```
ONA Host inicia servicio

        ↓

Publica:
ONA-GAMING-HOST.local

        ↓

Teléfono busca dispositivos

        ↓

Encuentra PC disponible

        ↓

Usuario confirma conexión
```

Información publicada:

```json
{
"name":"ONA Gaming Host",
"service":"_ona._udp",
"version":"1.0",
"port":5000
}
```

---

# 8. Protocolo de emparejamiento QR + Token

El QR permite una conexión rápida y segura.

## Flujo

```
PC genera sesión

        ↓

Crea QR temporal

        ↓

Usuario escanea QR

        ↓

Teléfono recibe:

- IP
- Puerto
- Token temporal

        ↓

Envía AUTH

        ↓

PC valida

        ↓

Control conectado
```

Ejemplo:

```json
{
"type":"ONA_PAIR",
"host":"ONA-PC",
"ip":"192.168.1.50",
"port":5000,
"token":"8F92KD31",
"expires":300
}
```

Características:

* Token temporal.
* Expiración automática.
* Nuevo token por sesión.

---

# 9. Gestión de conectividad

ONA monitorea constantemente el estado de la conexión.

Estados:

| Estado       | Descripción          |
| ------------ | -------------------- |
| CONNECTING   | Intentando conexión  |
| CONNECTED    | Comunicación activa  |
| UNSTABLE     | Pérdida parcial      |
| RECONNECTING | Recuperando conexión |
| DISCONNECTED | Sin conexión         |

---

# 10. Calidad de servicio (QoS)

ONA prioriza tráfico según importancia.

Orden:

```
1. Input del jugador
2. Voz
3. Eventos del juego
4. Servicios sociales
5. Telemetría
6. Descargas
```

Durante una partida:

* Los controles nunca deben competir con descargas.
* Las actualizaciones se pausan si afectan la experiencia.
* La telemetría reduce frecuencia automáticamente.

---

# 11. Latencia y optimización

## Objetivo

Latencia LAN:

```
< 10 ms
```

## Técnicas utilizadas:

### Frecuencia de actualización

Modos:

* Normal: 60 Hz.
* Alto rendimiento: 120 Hz.

---

### Compresión delta

Solo se envían cambios.

Ejemplo:

Estado anterior:

```
Joystick X = 100
```

Nuevo estado:

```
Joystick X = 105
```

Solo se transmite:

```
+5
```

---

### Interpolación

El receptor puede compensar pérdidas pequeñas:

```
Paquete 1
    |
Paquete perdido
    |
Paquete 3

Interpolación del movimiento
```

---

# 12. Manejo de desconexiones

ONA debe recuperarse automáticamente.

Objetivo:

```
Reconexión < 1 segundo
```

Proceso:

```
Se pierde conexión

        ↓

Control envía heartbeat

        ↓

Busca nuevamente ONA Host

        ↓

Revalida token

        ↓

Continúa sesión
```

El juego no debe cerrarse por una desconexión temporal del controlador.

---

# 13. Seguridad de red

## Comunicación externa

Protegida mediante:

* HTTPS.
* TLS 1.3.
* Certificados válidos.

---

## Voz

Protección:

* WebRTC.
* DTLS.
* SRTP.

---

## Comunicación local

Protección:

* Token de sesión.
* Validación de paquetes.
* Cifrado opcional ChaCha20.

---

# 14. Protocolo de voz ONA

La voz utiliza WebRTC.

Ventajas:

* Baja latencia.
* Cancelación de ruido.
* Adaptación automática de bitrate.
* Comunicación P2P.

Arquitectura:

```
Jugador A

   WebRTC

Jugador B


Si falla P2P:

        ↓

TURN Server
```

El procesamiento inicial ocurre en el teléfono:

* Captura del micrófono.
* Cancelación de ruido.
* Codificación Opus.

---

# 15. Protocolo de matchmaking

El matchmaking utiliza servicios en la nube.

Flujo:

```
Jugador busca partida

        ↓

ONA Matchmaking Server

        ↓

Busca jugadores compatibles

        ↓

Forma sesión

        ↓

Entrega información de conexión
```

Datos considerados:

* Región.
* Latencia.
* Nivel.
* Preferencias.
* Disponibilidad.

---

# 16. Protocolo de guardado en la nube

Los guardados utilizan comunicación segura HTTPS.

Flujo:

```
Juego finaliza

        ↓

ONA Runtime captura guardado

        ↓

Cifra datos

        ↓

Envía Cloud Save

        ↓

Servidor sincroniza
```

Características:

* Sincronización automática.
* Versionado.
* Recuperación ante conflictos.

---

# 17. Diagramas de flujo principales

## Controlador móvil

```
Teléfono inicia ONA Controller

          ↓

Busca Host

          ↓

Emparejamiento QR

          ↓

AUTH Token

          ↓

UDP Input Stream

          ↓

Juego recibe comandos
```

---

## Inicio de juego

```
Usuario selecciona juego

          ↓

ONA Core valida licencia

          ↓

Runtime inicia juego

          ↓

Controller Service activa UDP

          ↓

Jugadores conectan

          ↓

Partida inicia
```

---

## Servicios Cloud

```
ONA Host

     |

 HTTPS/TLS

     |

ONA Cloud

     |

Servicios:
- Cuenta
- Store
- Amigos
- Matchmaking
- Saves
```

---

# 18. Escalabilidad futura

La arquitectura permite evolucionar hacia:

* Servidores regionales.
* Juego en la nube.
* Streaming remoto.
* Redes P2P optimizadas.
* Infraestructura global.
* Integración con dispositivos ONA dedicados.

---

# 19. Conclusión

El protocolo de red de ONA está diseñado para cumplir la misión principal de la plataforma:

> **Convertir cualquier pantalla compatible en una experiencia de consola, utilizando dispositivos existentes como periféricos inteligentes y manteniendo una experiencia rápida, segura y accesible.**

La red ONA separa correctamente la comunicación crítica del juego de los servicios secundarios, permitiendo que la plataforma pueda crecer desde una habitación con una PC y teléfonos móviles hasta un ecosistema global de videojuegos.

---

*ONA Gaming Studio - Network Protocol Document v1.0*
*Status: Approved for development*
*Fecha: 2026-08-03*
