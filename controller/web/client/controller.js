// ============================================
// ONA CONTROLLER
// POINTER-ID MULTI-TOUCH INPUT ENGINE v4
// ============================================

console.log("=================================");
console.log("ONA CONTROLLER");
console.log("POINTER MULTI-TOUCH ENGINE v4");
console.log("=================================");

const controller = document.getElementById("controller");
const zone = document.getElementById("joystick-zone");
const joystick = document.getElementById("joystick");

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
    }
});

const buttons =
    [...new Set(document.querySelectorAll("[data-button]"))];

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

const activePointers = new Map();
const activeButtonCounts = new Map();
let joystickPointerId = null;

document.addEventListener(
    "gesturestart",
    (event) => event.preventDefault(),
    { passive: false }
);

document.addEventListener(
    "gesturechange",
    (event) => event.preventDefault(),
    { passive: false }
);

document.addEventListener(
    "gestureend",
    (event) => event.preventDefault(),
    { passive: false }
);

document.addEventListener(
    "contextmenu",
    (event) => event.preventDefault()
);

document.addEventListener(
    "selectstart",
    (event) => event.preventDefault()
);

function pointInsideElement(x, y, element) {
    const rect = element.getBoundingClientRect();

    return (
        x >= rect.left &&
        x <= rect.right &&
        y >= rect.top &&
        y <= rect.bottom
    );
}

function getButtonAt(x, y) {
    const element =
        document.elementFromPoint(x, y);

    const directButton =
        element?.closest?.("[data-button]");

    if (directButton && buttons.includes(directButton)) {
        return directButton;
    }

    return buttons.find((button) => pointInsideElement(x, y, button)) || null;
}

function updateJoystick(clientX, clientY) {
    const rect = zone.getBoundingClientRect();
    const centerX = rect.left + rect.width / 2;
    const centerY = rect.top + rect.height / 2;
    let x = clientX - centerX;
    let y = clientY - centerY;
    const maxDistance = rect.width * 0.27;
    const distance = Math.sqrt(x * x + y * y);

    if (distance > maxDistance && distance !== 0) {
        x = (x / distance) * maxDistance;
        y = (y / distance) * maxDistance;
    }

    const normalizedX = x / maxDistance;
    const normalizedY = y / maxDistance;

    joystick.style.transform =
        `translate(calc(-50% + ${x}px), calc(-50% + ${y}px))`;

    console.log(
        "[ONA Controller] INPUT SENT joystick",
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

function resetJoystick() {
    joystickPointerId = null;
    joystick.style.transform = "translate(-50%, -50%)";

    console.log("[ONA Controller] INPUT SENT joystick 0.00 0.00");

    sendControllerMessage({
        type: "input",
        control: "JOYSTICK",
        x: 0,
        y: 0
    });
}

function pressButton(button, pointerId) {
    const name = button.dataset.button;
    const count = activeButtonCounts.get(name) || 0;

    activeButtonCounts.set(name, count + 1);
    button.classList.add("virtual-pressed");

    if (count === 0) {
        console.log("[ONA Controller] INPUT SENT", name, "down", "pointerId=", pointerId);
        sendControllerMessage({ type: "input", button: name, state: "down" });
    }
}

function releaseButton(button, pointerId) {
    const name = button.dataset.button;
    const count = activeButtonCounts.get(name) || 0;

    if (count <= 1) {
        activeButtonCounts.delete(name);
        button.classList.remove("virtual-pressed");
        console.log("[ONA Controller] INPUT SENT", name, "up", "pointerId=", pointerId);
        sendControllerMessage({ type: "input", button: name, state: "up" });
        return;
    }

    activeButtonCounts.set(name, count - 1);
}

function capturePointer(element, pointerId) {
    try {
        element.setPointerCapture(pointerId);
    }
    catch (error) {
        console.warn("[ONA Controller] setPointerCapture failed:", pointerId, error);
    }
}

function releasePointer(element, pointerId) {
    try {
        if (element.hasPointerCapture?.(pointerId)) {
            element.releasePointerCapture(pointerId);
        }
    }
    catch (error) {
        console.warn("[ONA Controller] releasePointerCapture failed:", pointerId, error);
    }
}

function assignPointer(event) {
    const pointerId = event.pointerId;
    const x = event.clientX;
    const y = event.clientY;

    console.log(
        "[ONA Controller] POINTER DOWN",
        "pointerId=",
        pointerId,
        "isPrimary=",
        event.isPrimary,
        "x=",
        Math.round(x),
        "y=",
        Math.round(y)
    );

    if (
        joystickPointerId === null &&
        pointInsideElement(x, y, zone)
    ) {
        joystickPointerId = pointerId;
        activePointers.set(pointerId, {
            type: "joystick",
            element: zone
        });
        capturePointer(zone, pointerId);
        updateJoystick(x, y);
        console.log("[ONA Controller] POINTER ASSIGNED", pointerId, "JOYSTICK");
        return;
    }

    const button = getButtonAt(x, y);

    if (button) {
        activePointers.set(pointerId, {
            type: "button",
            element: button
        });
        capturePointer(button, pointerId);
        pressButton(button, pointerId);
        console.log("[ONA Controller] POINTER ASSIGNED", pointerId, button.dataset.button);
        return;
    }

    activePointers.set(pointerId, {
        type: "none",
        element: controller
    });
    capturePointer(controller, pointerId);
    console.log("[ONA Controller] POINTER ASSIGNED", pointerId, "NONE");
}

function movePointer(event) {
    const state = activePointers.get(event.pointerId);

    if (!state || state.type !== "joystick") {
        return;
    }

    updateJoystick(event.clientX, event.clientY);
}

function releaseActivePointer(event, cancelled = false) {
    const pointerId = event.pointerId;
    const state = activePointers.get(pointerId);

    console.log(
        cancelled
            ? "[ONA Controller] POINTER CANCEL"
            : "[ONA Controller] POINTER UP",
        "pointerId=",
        pointerId
    );

    if (!state) {
        return;
    }

    if (state.type === "joystick" && joystickPointerId === pointerId) {
        resetJoystick();
    }

    if (state.type === "button") {
        releaseButton(state.element, pointerId);
    }

    releasePointer(state.element, pointerId);
    activePointers.delete(pointerId);
}

controller.addEventListener(
    "pointerdown",
    (event) => {
        event.preventDefault();
        assignPointer(event);
    },
    { passive: false }
);

controller.addEventListener(
    "pointermove",
    (event) => {
        event.preventDefault();
        movePointer(event);
    },
    { passive: false }
);

controller.addEventListener(
    "pointerup",
    (event) => {
        event.preventDefault();
        releaseActivePointer(event);
    },
    { passive: false }
);

controller.addEventListener(
    "pointercancel",
    (event) => {
        event.preventDefault();
        releaseActivePointer(event, true);
    },
    { passive: false }
);

window.addEventListener(
    "blur",
    () => {
        for (const pointerId of [...activePointers.keys()]) {
            const state = activePointers.get(pointerId);

            if (state?.type === "joystick") {
                resetJoystick();
            }

            if (state?.type === "button") {
                releaseButton(state.element, pointerId);
            }

            activePointers.delete(pointerId);
        }
    }
);

console.log("ONA POINTER MULTITOUCH READY");
console.log("Joystick + buttons independent by pointerId");
