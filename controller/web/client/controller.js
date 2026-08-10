// ============================================
// ONA CONTROLLER
// GLOBAL MULTI-TOUCH INPUT ENGINE v3
// ============================================

console.log("=================================");
console.log("ONA CONTROLLER");
console.log("MULTI-TOUCH ENGINE v3");
console.log("=================================");


// ============================================
// ELEMENTOS
// ============================================

const controller = document.getElementById("controller");
const zone = document.getElementById("joystick-zone");
const joystick = document.getElementById("joystick");

const buttons = [
    ...document.querySelectorAll("[data-button]")
];

[
    [".shoulder.l1", "L1"],
    [".shoulder.l2", "L2"],
    [".shoulder.r1", "R1"],
    [".shoulder.r2", "R2"],
    [".system-button:first-child", "SELECT"],
    [".system-button:last-child", "START"]
].forEach(([selector, name]) => {
    const button = document.querySelector(selector);

    if (button) {
        button.dataset.button = name;
        buttons.push(button);
    }
});


// ============================================
// ONA CORE WEBSOCKET
// ============================================

const pairingParams = new URLSearchParams(window.location.search);
const sessionId = pairingParams.get("id");
const token = pairingParams.get("token");

console.log("[ONA CONTROLLER] Current URL:", window.location.href);
console.log("[ONA CONTROLLER] sessionId:", sessionId);
console.log("[ONA CONTROLLER] token:", token);

const websocketScheme =
    window.location.protocol === "https:"
        ? "wss"
        : "ws";

const websocketPort = window.location.port || "8080";
const wsUrl = `${websocketScheme}://${window.location.hostname}:${websocketPort}/ws`;

console.log("[ONA CONTROLLER] WebSocket URL:", wsUrl);
console.log("[ONA CONTROLLER] WebSocket connecting...");

const controllerSocket = new WebSocket(wsUrl);

function sendControllerMessage(message) {
    if (controllerSocket.readyState === WebSocket.OPEN) {
        controllerSocket.send(JSON.stringify(message));
    }
}

controllerSocket.addEventListener(
    "open",
    () => {
        console.log("[ONA CONTROLLER] WebSocket open");

        sendControllerMessage({
            type: "controller_connected",
            sessionId,
            token
        });
    }
);

controllerSocket.addEventListener(
    "error",
    (event) => console.error("[ONA CONTROLLER] WebSocket error:", event)
);

controllerSocket.addEventListener(
    "close",
    (event) => console.warn("[ONA CONTROLLER] WebSocket close:", event.code, event.reason)
);


// ============================================
// ESTADO MULTI-TOUCH
// ============================================
//
// Cada dedo es completamente independiente.
//
// dedo 1 -> joystick
// dedo 2 -> A
// dedo 3 -> B
// dedo 4 -> X
//
// ============================================

const activeTouches = new Map();


// ============================================
// ESTADO DEL JOYSTICK
// ============================================

let joystickTouchId = null;


// ============================================
// ESTADO DE BOTONES
// ============================================

const pressedButtons = new Map();


// ============================================
// PROTECCIÓN CONTRA ZOOM / GESTOS
// ============================================

document.addEventListener(
    "gesturestart",
    (event) => {
        event.preventDefault();
    },
    {
        passive: false
    }
);

document.addEventListener(
    "gesturechange",
    (event) => {
        event.preventDefault();
    },
    {
        passive: false
    }
);

document.addEventListener(
    "gestureend",
    (event) => {
        event.preventDefault();
    },
    {
        passive: false
    }
);


// ============================================
// UTILIDAD
// DETECTAR SI UN PUNTO ESTÁ DENTRO
// DE UN ELEMENTO
// ============================================

function pointInsideElement(x, y, element) {

    const rect =
        element.getBoundingClientRect();

    return (
        x >= rect.left &&
        x <= rect.right &&
        y >= rect.top &&
        y <= rect.bottom
    );
}


// ============================================
// BUSCAR BOTÓN
// ============================================

function getButtonAt(x, y) {

    for (const button of buttons) {

        if (
            pointInsideElement(
                x,
                y,
                button
            )
        ) {

            return button;

        }

    }

    return null;
}


// ============================================
// ACTUALIZAR JOYSTICK
// ============================================

function updateJoystick(clientX, clientY) {

    const rect =
        zone.getBoundingClientRect();


    const centerX =
        rect.left +
        rect.width / 2;


    const centerY =
        rect.top +
        rect.height / 2;


    let x =
        clientX -
        centerX;


    let y =
        clientY -
        centerY;


    const maxDistance =
        rect.width * 0.27;


    const distance =
        Math.sqrt(
            x * x +
            y * y
        );


    if (
        distance > maxDistance &&
        distance !== 0
    ) {

        x =
            (x / distance) *
            maxDistance;


        y =
            (y / distance) *
            maxDistance;

    }


    const normalizedX =
        x / maxDistance;


    const normalizedY =
        y / maxDistance;


    joystick.style.transform =
        `translate(
            calc(-50% + ${x}px),
            calc(-50% + ${y}px)
        )`;


    console.log(
        "JOYSTICK:",
        normalizedX.toFixed(2),
        normalizedY.toFixed(2)
    );

    sendControllerMessage({
        type: "input",
        control: "JOYSTICK",
        x: normalizedX,
        y: normalizedY
    });

}


// ============================================
// RESET JOYSTICK
// ============================================

function resetJoystick() {

    joystickTouchId = null;

    joystick.style.transform =
        "translate(-50%, -50%)";

    console.log(
        "JOYSTICK RESET"
    );

    sendControllerMessage({ type: "input", control: "JOYSTICK", x: 0, y: 0 });
}


// ============================================
// PRESIONAR BOTÓN
// ============================================

function pressButton(button, touchId) {

    const name =
        button.dataset.button;


    // Guardamos qué dedo pertenece
    // a qué botón.

    pressedButtons.set(
        touchId,
        {
            button: button,
            name: name
        }
    );


    button.classList.add(
        "virtual-pressed"
    );


    console.log(
        "BUTTON DOWN:",
        name,
        "TOUCH:",
        touchId
    );

    sendControllerMessage({ type: "input", button: name, state: "down" });
}


// ============================================
// SOLTAR BOTÓN
// ============================================

function releaseButton(touchId) {

    const state =
        pressedButtons.get(
            touchId
        );


    if (!state) {
        return;
    }


    state.button.classList.remove(
        "virtual-pressed"
    );


    console.log(
        "BUTTON UP:",
        state.name,
        "TOUCH:",
        touchId
    );

    sendControllerMessage({ type: "input", button: state.name, state: "up" });


    pressedButtons.delete(
        touchId
    );
}


// ============================================
// ASIGNAR TOUCH
// ============================================

function assignTouch(touch) {

    const id =
        touch.identifier;


    const x =
        touch.clientX;


    const y =
        touch.clientY;


    console.log(
        "TOUCH START:",
        id,
        x,
        y
    );


    // ========================================
    // JOYSTICK
    // ========================================

    if (
        joystickTouchId === null &&
        pointInsideElement(
            x,
            y,
            zone
        )
    ) {

        joystickTouchId =
            id;


        activeTouches.set(
            id,
            {
                type: "joystick"
            }
        );


        updateJoystick(
            x,
            y
        );


        console.log(
            "ASSIGNED:",
            id,
            "-> JOYSTICK"
        );


        return;
    }


    // ========================================
    // BOTÓN
    // ========================================

    const button =
        getButtonAt(
            x,
            y
        );


    if (button) {

        activeTouches.set(
            id,
            {
                type: "button",
                button: button
            }
        );


        pressButton(
            button,
            id
        );


        console.log(
            "ASSIGNED:",
            id,
            "-> BUTTON",
            button.dataset.button
        );


        return;
    }


    // ========================================
    // TOQUE SIN FUNCIÓN
    // ========================================

    activeTouches.set(
        id,
        {
            type: "none"
        }
    );


    console.log(
        "ASSIGNED:",
        id,
        "-> NONE"
    );
}


// ============================================
// LIBERAR TOUCH
// ============================================

function releaseTouch(touch) {

    const id =
        touch.identifier;


    const state =
        activeTouches.get(
            id
        );


    if (!state) {
        return;
    }


    // ========================================
    // JOYSTICK
    // ========================================

    if (
        state.type ===
        "joystick"
    ) {

        if (
            joystickTouchId === id
        ) {

            resetJoystick();

        }

    }


    // ========================================
    // BOTÓN
    // ========================================

    if (
        state.type ===
        "button"
    ) {

        releaseButton(
            id
        );

    }


    // ========================================
    // ELIMINAR DEDO
    // ========================================

    activeTouches.delete(
        id
    );


    console.log(
        "TOUCH RELEASE:",
        id
    );
}


// ============================================
// TOUCH START
// ============================================
//
// IMPORTANTE:
//
// Cada nuevo dedo se procesa
// independientemente.
//
// ============================================

document.addEventListener(
    "touchstart",
    (event) => {

        event.preventDefault();


        for (
            const touch
            of event.changedTouches
        ) {

            assignTouch(
                touch
            );

        }


        console.log(
            "ACTIVE FINGERS:",
            event.touches.length
        );

    },
    {
        passive: false,
        capture: true
    }
);


// ============================================
// TOUCH MOVE
// ============================================

document.addEventListener(
    "touchmove",
    (event) => {

        event.preventDefault();


        for (
            const touch
            of event.changedTouches
        ) {

            const id =
                touch.identifier;


            const state =
                activeTouches.get(
                    id
                );


            if (!state) {
                continue;
            }


            // =================================
            // JOYSTICK
            // =================================

            if (
                state.type ===
                "joystick"
            ) {

                updateJoystick(
                    touch.clientX,
                    touch.clientY
                );

            }


            // =================================
            // BOTÓN
            // =================================
            //
            // El botón mantiene su estado.
            //
            // Aunque el dedo se mueva,
            // sigue perteneciendo al botón
            // original.
            //

            if (
                state.type ===
                "button"
            ) {

                continue;

            }

        }

    },
    {
        passive: false,
        capture: true
    }
);


// ============================================
// TOUCH END
// ============================================

document.addEventListener(
    "touchend",
    (event) => {

        event.preventDefault();


        for (
            const touch
            of event.changedTouches
        ) {

            releaseTouch(
                touch
            );

        }


        console.log(
            "ACTIVE FINGERS:",
            event.touches.length
        );

    },
    {
        passive: false,
        capture: true
    }
);


// ============================================
// TOUCH CANCEL
// ============================================

document.addEventListener(
    "touchcancel",
    (event) => {

        event.preventDefault();


        for (
            const touch
            of event.changedTouches
        ) {

            releaseTouch(
                touch
            );

        }


        console.log(
            "TOUCH CANCEL"
        );

    },
    {
        passive: false,
        capture: true
    }
);


// ============================================
// EVITAR MENÚ CONTEXTUAL
// ============================================

document.addEventListener(
    "contextmenu",
    (event) => {

        event.preventDefault();

    }
);


// ============================================
// BLOQUEAR SELECCIÓN
// ============================================

document.addEventListener(
    "selectstart",
    (event) => {

        event.preventDefault();

    }
);


// ============================================
// DIAGNÓSTICO
// ============================================

console.log(
    "ONA GLOBAL MULTITOUCH READY"
);

console.log(
    "Joystick + Buttons independent"
);
