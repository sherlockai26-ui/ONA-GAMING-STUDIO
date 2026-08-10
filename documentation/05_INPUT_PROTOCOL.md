```markdown
# ONA Input Protocol v1.0

## 1. Introducción al protocolo

ONA Input Protocol define el sistema de comunicación entre el dispositivo móvil **ONA Controller** y la computadora **ONA Host**, donde se ejecuta **ONA Core**.

Este protocolo permite transformar un teléfono inteligente en un controlador universal de videojuegos, utilizando sus capacidades integradas:

- Pantalla táctil.
- Sensores de movimiento.
- Micrófono.
- Altavoz.
- Vibración.
- Conectividad inalámbrica.

El objetivo principal es ofrecer una experiencia equivalente a un controlador físico tradicional, pero con mayor flexibilidad, personalización y accesibilidad.

ONA adopta un modelo donde:

```

Teléfono móvil
│
│  Eventos de entrada
│  Sensores
│  Voz
▼
ONA Core (PC)
│
│  Traducción de eventos
▼
Videojuego

```

El teléfono no funciona como un simple control remoto.

Es un **periférico inteligente**, encargado de procesar información localmente y enviar únicamente eventos optimizados a la computadora.

---

# 2. Objetivos de diseño

## 2.1 Baja latencia

El protocolo está diseñado para lograr:

```

Latencia objetivo:

< 10 ms dentro de una red LAN

```

Para lograrlo:

- Se utiliza UDP.
- Los paquetes son binarios.
- No se utilizan estructuras pesadas como JSON.
- Se evita comunicación innecesaria.

---

## 2.2 Alta eficiencia

El protocolo busca minimizar:

- Tamaño de paquetes.
- Uso de red.
- Consumo energético.
- Procesamiento.

Características:

- Cabecera fija.
- Payload compacto.
- Compresión delta.
- Envío únicamente de cambios.

---

## 2.3 Seguridad

El protocolo incorpora:

- Emparejamiento seguro.
- Tokens temporales.
- Validación de paquetes.
- Cifrado opcional.

---

## 2.4 Escalabilidad

El protocolo debe soportar:

- 1 jugador.
- Multijugador local.
- Hasta 10 dispositivos móviles simultáneos.

---

# 3. Arquitectura de comunicación

ONA utiliza comunicación directa entre teléfono y PC.

## Modelo general

```

┌──────────────────────┐
│   ONA Controller     │
│     Smartphone       │
│                      │
│ - Touch Input        │
│ - Sensors            │
│ - Voice Processing   │
│ - Encryption         │
└──────────┬───────────┘
│
│ UDP Binary
│
▼
┌──────────────────────┐
│      ONA Core        │
│         PC           │
│                      │
│ - UDP Receiver       │
│ - Validation         │
│ - Input Mapping      │
│ - Game Translation   │
└──────────┬───────────┘
│
▼
┌──────────────────────┐
│      Videojuego      │
└──────────────────────┘

```

---

# 4. Transporte de comunicación

## Protocolo principal

ONA utiliza:

```

UDP

```

Razones:

- Menor latencia.
- Sin confirmaciones innecesarias.
- Ideal para entradas en tiempo real.

---

## Protocolos secundarios

| Función | Protocolo |
|-|-|
| Entrada de controles | UDP Binario |
| Voz | UDP/WebRTC |
| Configuración | WebSocket |
| Emparejamiento | HTTPS/QR |

---

# 5. Formato del paquete

Todos los mensajes utilizan una estructura binaria.

Formato general:

```

+----------------+
| HEADER         |
+----------------+
| PAYLOAD        |
+----------------+

```

---

# 5.1 Cabecera

Tamaño:

```

8 bytes

```

Estructura:

| Offset | Campo | Tamaño | Descripción |
|-|-|-|-|
| 0 | Magic | 2 bytes | Identificador ONA |
| 2 | Version | 1 byte | Versión protocolo |
| 3 | Type | 1 byte | Tipo de mensaje |
| 4 | Player ID | 1 byte | Jugador asignado |
| 5 | Sequence | 2 bytes | Número consecutivo |
| 7 | Checksum | 1 byte | Validación |

---

## Magic

Valor:

```

0x4F4E

```

Representa:

```

ON

```

Permite identificar paquetes ONA.

---

# 5.2 Payload

El contenido depende del tipo de mensaje.

Ejemplo:

```

HEADER

*

INPUT DATA

*

SENSOR DATA

```

---

# 6. Tipos de mensaje

## Tabla general

| Tipo | Nombre | Uso |
|-|-|-|
| 0x01 | INPUT | Entradas del jugador |
| 0x02 | VIBRATE | Vibración del teléfono |
| 0x03 | VOICE | Comunicación de voz |
| 0x04 | SENSOR | Datos de movimiento |
| 0x05 | PING | Medición de latencia |
| 0x06 | PONG | Respuesta |
| 0x07 | AUTH | Autenticación |
| 0x08 | CONFIG | Configuración |

---

# 6.1 INPUT (0x01)

Mensaje principal del protocolo.

Transporta:

- Botones.
- Joysticks.
- Gatillos.

Frecuencia:

```

60 Hz estándar

120 Hz modo competitivo

```

---

# Payload INPUT

| Campo | Tamaño |
|-|-|
| Buttons | 2 bytes |
| Left X | 2 bytes |
| Left Y | 2 bytes |
| Right X | 2 bytes |
| Right Y | 2 bytes |
| Triggers | 2 bytes |

Total:

```

12 bytes

```

---

# 6.2 VIBRATE (0x02)

Enviado desde PC hacia teléfono.

Permite:

- Vibración.
- Retroalimentación háptica.

Payload:

```

Intensity
Duration

```

---

# 6.3 VOICE (0x03)

Transporta audio comprimido.

Características:

- Procesamiento local en teléfono.
- Codec Opus.
- Bajo consumo.

El teléfono:

- Captura audio.
- Comprime.
- Envía paquetes.

La PC:

- Distribuye.
- Sincroniza.

---

# 6.4 SENSOR (0x04)

Transporta:

- Giroscopio.
- Acelerómetro.

Usado para:

- Movimiento.
- Juegos compatibles.
- Realidad aumentada futura.

---

# 6.5 PING / PONG

Usado para medir:

- Latencia.
- Calidad de conexión.

Proceso:

```

PC → PING

Teléfono → PONG

```

---

# 6.6 AUTH (0x07)

Utilizado durante conexión inicial.

Incluye:

- Token.
- Identificación.
- Sesión.

---

# 6.7 CONFIG (0x08)

Permite modificar:

- Distribución de botones.
- Sensibilidad.
- Perfil del control.
- Configuración visual.

---

# 7. Mapeo de botones y ejes

ONA utiliza un sistema universal similar a controles tradicionales.

## Botones principales

| Bit | Botón |
|-|-|
| 0 | A |
| 1 | B |
| 2 | X |
| 3 | Y |
| 4 | L1 |
| 5 | R1 |
| 6 | L3 |
| 7 | R3 |
| 8 | START |
| 9 | SELECT |
| 10 | HOME |

---

## Joysticks

Valores:

```

-32768 a 32767

```

Representación:

```

Centro:

0

Máximo izquierda:

-32768

Máximo derecha:

32767

```

---

# 8. Emparejamiento mediante QR

ONA utiliza QR para simplificar la conexión.

## Flujo:

```

ONA Host genera sesión

```
    ↓
```

Genera código QR

```
    ↓
```

Jugador escanea con teléfono

```
    ↓
```

ONA Controller obtiene datos

```
    ↓
```

Envía AUTH

```
    ↓
```

Jugador conectado

````

---

## Información del QR

Ejemplo:

```json
{
"type":"ONA_PAIR",
"device":"ONA-PC-001",
"ip":"192.168.1.25",
"port":5000,
"token":"A82F91KD",
"expires":1734567890
}
````

---

# 9. Autenticación y token

Cada conexión utiliza un token temporal.

Características:

* Generado por ONA Host.
* Expiración automática.
* Único por sesión.

Ejemplo:

```
Jugador 1

Token:

A82F91KD
```

---

Proceso:

```
Teléfono

envía AUTH + token

        ↓

PC valida

        ↓

Asigna Player ID

        ↓

Permite INPUT
```

---

# 10. Gestión de múltiples jugadores

ONA soporta:

```
Hasta 10 jugadores simultáneos
```

Cada dispositivo recibe:

```
Player ID

1 - 10
```

Ejemplo:

```
Jugador 1 → Player ID 01

Jugador 2 → Player ID 02

Jugador 3 → Player ID 03
```

---

ONA administra:

* Estado individual.
* Batería.
* Conexión.
* Perfil.

---

# 11. Latencia y optimización

## Frecuencia de actualización

Modos:

| Modo        | Frecuencia |
| ----------- | ---------- |
| Normal      | 60 Hz      |
| Competitivo | 120 Hz     |

---

# 11.1 Compresión Delta

ONA no envía información repetida.

Ejemplo:

Sin compresión:

```
Joystick X = 0

Joystick X = 0

Joystick X = 0
```

Con delta:

```
Solo enviar cuando cambia
```

---

# 11.2 Interpolación

ONA Core puede compensar pérdida de paquetes mediante:

* Predicción.
* Interpolación.
* Corrección temporal.

---

# 11.3 Priorización

Orden de prioridad:

1. INPUT.
2. VOICE.
3. SENSOR.
4. CONFIG.

---

# 12. Voz integrada

La voz es procesada principalmente en el teléfono.

El teléfono realiza:

* Captura.
* Cancelación de ruido.
* Compresión.

La PC recibe:

* Audio comprimido.
* Información de sesión.

Tecnologías:

* Opus.
* WebRTC.

Ventajas:

* Menor carga en PC.
* Mejor escalabilidad.
* Menor latencia.

---

# 13. Seguridad del protocolo

## Protección básica

Incluye:

* Magic validation.
* Sequence validation.
* Checksum.

---

## Protección avanzada

Opcionalmente:

* ChaCha20.
* AES-256.
* Claves derivadas del token.

---

## Prevención de ataques

Medidas:

* Tokens temporales.
* Expiración de sesiones.
* Bloqueo de paquetes inválidos.
* Límite de dispositivos.

---

# 14. Implementación de referencia

## Teléfono

Responsabilidades:

```
Capturar entrada

Procesar sensores

Comprimir datos

Enviar UDP

Procesar voz
```

---

## PC / ONA Core

Responsabilidades:

```
Recibir paquetes

Validar

Asignar jugador

Traducir eventos

Enviar al juego
```

---

# 15. Esquema general

```
                Smartphone

             Touch / Sensors

                    │

                    ▼

             ONA Controller

                    │

              UDP Binary

                    │

                    ▼

                ONA Core

                    │

          Input Translation Layer

                    │

                    ▼

               Videojuego
```

---

# 16. Conclusión

ONA Input Protocol permite convertir cualquier teléfono inteligente en un controlador universal de videojuegos.

El diseño prioriza:

* Baja latencia.
* Eficiencia.
* Seguridad.
* Escalabilidad.
* Personalización.

El teléfono no reemplaza simplemente un control físico.

Se convierte en un periférico inteligente capaz de evolucionar junto con la plataforma.

---

**ONA Gaming Studio**

**Input Protocol Document v1.0**

**Status: Approved for Development**

```
```
