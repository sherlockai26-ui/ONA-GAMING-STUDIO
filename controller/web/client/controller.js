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
const customizeToggle = document.getElementById("customize-toggle");
const customizePanel = document.getElementById("customize-panel");

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

const reconnectBackoffMs = [500, 1000, 2000, 3000, 5000];
let controllerSocket = null;
let reconnectAttempt = 0;
let reconnectTimer = null;
let heartbeatTimer = null;
let reconnectEnabled = true;
let playerId = null;
let lastTouchEndAt = 0;

function sendControllerMessage(message) {
    if (controllerSocket?.readyState === WebSocket.OPEN) {
        controllerSocket.send(JSON.stringify(message));
    }
}

function setControllerConnectionState(label) {
    document.body.dataset.connectionState = label.toLowerCase();
    const status = document.getElementById("connection-status");

    if (status) {
        status.textContent = label;
    }
}

function connectWebSocket() {
    console.log("[ONA CONTROLLER] WebSocket connecting...");
    controllerSocket = new WebSocket(wsUrl);

    controllerSocket.addEventListener(
        "open",
        () => {
            console.log("[ONA CONTROLLER] WebSocket open");

            sendControllerMessage({
                type: "controller_connected",
                sessionId,
                token
            });
            startHeartbeat();
        }
    );

    controllerSocket.addEventListener(
        "message",
        (event) => {
            let message = null;

            try {
                message = JSON.parse(event.data);
            }
            catch {
                return;
            }

            if (message.type === "controller_authenticated") {
                playerId = message.playerId;
                reconnectAttempt = 0;
                console.log("[CONTROLLER] reconnected");
                console.log("[CONTROLLER] session resumed player=", playerId);
                setControllerConnectionState("CONNECTED");
                return;
            }

            if (message.type === "controller_auth_rejected") {
                reconnectEnabled = false;
                stopHeartbeat();
                console.warn("[CONTROLLER] reconnect rejected", message.reason);
                if (
                    String(message.reason || "")
                        .toUpperCase()
                        .includes("INVALID")
                ) {
                    window.localStorage.removeItem("onaControllerSession");
                    window.localStorage.removeItem("onaControllerPairing");
                    setControllerConnectionState("PAIRING REQUIRED");
                    console.warn("[CONTROLLER] invalid stored pairing cleared; layout preserved");
                    return;
                }
                setControllerConnectionState("SESSION EXPIRED");
                return;
            }
        }
    );

    controllerSocket.addEventListener(
        "error",
        (event) => {
            console.error("[ONA CONTROLLER] WebSocket error:", event);
        }
    );

    controllerSocket.addEventListener(
        "close",
        (event) => {
            console.warn("[ONA CONTROLLER] WebSocket close:", event.code, event.reason);
            console.warn("[CONTROLLER] websocket lost");
            stopHeartbeat();
            neutralizeLocalInputState();
            scheduleReconnect();
        }
    );
}

function scheduleReconnect() {
    if (!reconnectEnabled || reconnectTimer) {
        return;
    }

    setControllerConnectionState("RECONNECTING...");
    const delay =
        reconnectBackoffMs[Math.min(reconnectAttempt, reconnectBackoffMs.length - 1)];

    reconnectAttempt += 1;
    console.log("[CONTROLLER] reconnect attempt=", reconnectAttempt);

    reconnectTimer =
        window.setTimeout(
            () => {
                reconnectTimer = null;
                connectWebSocket();
            },
            delay
        );
}

function startHeartbeat() {
    stopHeartbeat();
    heartbeatTimer =
        window.setInterval(
            () => {
                sendControllerMessage({
                    type: "ping",
                    timestamp: Date.now()
                });
            },
            5000
        );
}

function stopHeartbeat() {
    if (heartbeatTimer) {
        window.clearInterval(heartbeatTimer);
        heartbeatTimer = null;
    }
}

connectWebSocket();

const activePointers = new Map();
const activeButtonCounts = new Map();
let joystickPointerId = null;
let controllerCustomizing = false;
let controllerUiMode = "NORMAL";
let selectedCustomizationId = "joystick";
let customizationPointer = null;

const controllerCustomizationStorageKey =
    "onaControllerCustomizationV1";

const customizableControls = [
    ["joystick", zone],
    ...buttons.map((button) => [button.dataset.button, button])
].filter(([, element]) => Boolean(element));

const defaultCustomization = {
    palette: "blue",
    controls: Object.fromEntries(
        customizableControls.map(([id]) => [
            id,
            {
                x: 0,
                y: 0,
                scale: 1
            }
        ])
    )
};

let controllerCustomization =
    loadControllerCustomization();
let draftControllerCustomization =
    null;

function cloneCustomization(customization) {
    return JSON.parse(JSON.stringify(customization));
}

function loadControllerCustomization() {
    try {
        const stored =
            JSON.parse(
                window.localStorage.getItem(controllerCustomizationStorageKey) || "null"
            );

        return mergeCustomization(stored);
    }
    catch {
        return mergeCustomization(null);
    }
}

function mergeCustomization(stored) {
    return {
        palette:
            stored?.palette || defaultCustomization.palette,
        controls:
            Object.fromEntries(
                customizableControls.map(([id]) => {
                    const control =
                        stored?.controls?.[id] ||
                        defaultCustomization.controls[id];

                    return [
                        id,
                        {
                            x:
                                clamp(Number(control.x) || 0, -180, 180),
                            y:
                                clamp(Number(control.y) || 0, -90, 90),
                            scale:
                                clamp(Number(control.scale) || 1, 0.75, 1.35)
                        }
                    ];
                })
            )
    };
}

function saveControllerCustomization() {
    window.localStorage.setItem(
        controllerCustomizationStorageKey,
        JSON.stringify(controllerCustomization)
    );
}

function clamp(value, min, max) {
    return Math.min(max, Math.max(min, value));
}

function applyControllerCustomization() {
    const activeCustomization =
        draftControllerCustomization || controllerCustomization;

    document.body.dataset.controllerPalette =
        activeCustomization.palette;

    for (const [id, element] of customizableControls) {
        element.classList.add("ona-customizable");
        element.classList.toggle(
            "customization-selected",
            controllerUiMode === "EDIT_LAYOUT" && selectedCustomizationId === id
        );
        const control =
            activeCustomization.controls[id] ||
            defaultCustomization.controls[id];

        element.style.setProperty("--ona-control-x", `${control.x}px`);
        element.style.setProperty("--ona-control-y", `${control.y}px`);
        element.style.setProperty("--ona-control-scale", String(control.scale));
    }
}

function setControllerUiMode(mode) {
    controllerUiMode =
        mode || "NORMAL";
    controllerCustomizing =
        controllerUiMode !== "NORMAL";

    console.log(`[CONTROLLER UI] mode=${controllerUiMode}`);

    if (controllerUiMode === "EDIT_LAYOUT") {
        draftControllerCustomization =
            cloneCustomization(controllerCustomization);
        neutralizeLocalInputState();
    }

    if (controllerUiMode === "NORMAL") {
        draftControllerCustomization =
            null;
    }

    document.body.classList.toggle(
        "controller-customizing",
        controllerCustomizing
    );

    if (customizePanel) {
        customizePanel.hidden =
            !controllerCustomizing;
    }

    applyControllerCustomization();
}

function customizationTargetAt(x, y) {
    const element =
        document.elementFromPoint(x, y);
    const target =
        element?.closest?.(".ona-customizable");

    return customizableControls.find(([, control]) => control === target) || null;
}

function startCustomizationDrag(event) {
    if (controllerUiMode !== "EDIT_LAYOUT") {
        return false;
    }

    const target =
        customizationTargetAt(event.clientX, event.clientY);

    if (!target) {
        return false;
    }

    const [id, element] =
        target;
    const control =
        draftControllerCustomization.controls[id];

    selectedCustomizationId =
        id;
    console.log(`[CONTROLLER UI] selected=${id}`);
    applyControllerCustomization();
    customizationPointer = {
        pointerId:
            event.pointerId,
        id,
        element,
        startX:
            event.clientX,
        startY:
            event.clientY,
        originalX:
            control.x,
        originalY:
            control.y
    };

    capturePointer(element, event.pointerId);
    return true;
}

function moveCustomizationDrag(event) {
    if (
        !customizationPointer ||
        customizationPointer.pointerId !== event.pointerId
    ) {
        return false;
    }

    const control =
        draftControllerCustomization.controls[customizationPointer.id];

    control.x =
        clamp(customizationPointer.originalX + event.clientX - customizationPointer.startX, -180, 180);
    control.y =
        clamp(customizationPointer.originalY + event.clientY - customizationPointer.startY, -90, 90);
    console.log(
        `[CONTROLLER UI] drag ${customizationPointer.id} x=${control.x.toFixed(0)} y=${control.y.toFixed(0)}`
    );
    applyControllerCustomization();
    return true;
}

function endCustomizationDrag(event) {
    if (
        !customizationPointer ||
        customizationPointer.pointerId !== event.pointerId
    ) {
        return false;
    }

    releasePointer(customizationPointer.element, event.pointerId);
    customizationPointer =
        null;
    return true;
}

function applyCustomizationPreset(name) {
    draftControllerCustomization =
        mergeCustomization(null);

    if (name === "compact") {
        for (const control of Object.values(draftControllerCustomization.controls)) {
            control.scale = 0.88;
        }
    }

    if (name === "large") {
        for (const control of Object.values(draftControllerCustomization.controls)) {
            control.scale = 1.18;
        }
    }

    if (name === "left") {
        draftControllerCustomization.controls.joystick.x = 120;
        for (const button of ["A", "B", "X", "Y"]) {
            draftControllerCustomization.controls[button].x = -120;
        }
    }

    applyControllerCustomization();
}

customizeToggle?.addEventListener(
    "pointerdown",
    (event) => {
        event.preventDefault();
        event.stopPropagation();
        setControllerUiMode(
            controllerUiMode === "NORMAL"
                ? "EDIT_LAYOUT"
                : "NORMAL"
        );
    },
    { passive: false }
);

customizePanel?.addEventListener(
    "pointerdown",
    (event) => {
        event.preventDefault();
        event.stopPropagation();

        const action =
            event.target?.dataset?.customizeAction;

        if (!action) {
            return;
        }

        if (action === "done") {
            controllerCustomization =
                cloneCustomization(draftControllerCustomization || controllerCustomization);
            saveControllerCustomization();
            console.log("[CONTROLLER UI] layout saved");
            setControllerUiMode("NORMAL");
            return;
        }

        if (action === "cancel") {
            console.log("[CONTROLLER UI] layout cancelled");
            setControllerUiMode("NORMAL");
            return;
        }

        if (action.startsWith("preset-")) {
            applyCustomizationPreset(action.replace("preset-", ""));
            return;
        }

        if (action === "palette-blue" || action === "palette-red") {
            const targetCustomization =
                draftControllerCustomization || controllerCustomization;
            targetCustomization.palette =
                action.replace("palette-", "");
            applyControllerCustomization();
            return;
        }

        if (action === "size-up" || action === "size-down") {
            const targetCustomization =
                draftControllerCustomization || controllerCustomization;
            const control =
                targetCustomization.controls[selectedCustomizationId];

            if (control) {
                control.scale =
                    clamp(control.scale + (action === "size-up" ? 0.05 : -0.05), 0.75, 1.35);
                console.log(
                    `[CONTROLLER UI] resize ${selectedCustomizationId} size=${control.scale.toFixed(2)}`
                );
                applyControllerCustomization();
            }
            return;
        }

        if (action === "reset") {
            draftControllerCustomization =
                mergeCustomization(null);
            applyControllerCustomization();
        }
    },
    { passive: false }
);

applyControllerCustomization();

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
    "touchmove",
    (event) => {
        if (!event.target?.closest?.(".customize-panel")) {
            event.preventDefault();
        }
    },
    { passive: false }
);

document.addEventListener(
    "touchend",
    (event) => {
        const now =
            Date.now();

        if (now - lastTouchEndAt < 320) {
            event.preventDefault();
        }

        lastTouchEndAt =
            now;
    },
    { passive: false }
);

document.addEventListener(
    "selectstart",
    (event) => event.preventDefault()
);

window.addEventListener(
    "pageshow",
    () => {
        window.scrollTo(0, 0);
    }
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

function neutralizeLocalInputState() {
    for (const pointerId of [...activePointers.keys()]) {
        const state = activePointers.get(pointerId);

        if (state?.type === "button") {
            state.element?.classList?.remove("virtual-pressed");
        }

        activePointers.delete(pointerId);
    }

    activeButtonCounts.clear();
    joystickPointerId = null;
    joystick.style.transform = "translate(-50%, -50%)";
    console.log("[CONTROLLER] local input state neutralized");
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
        if (controllerUiMode !== "NORMAL") {
            startCustomizationDrag(event);
            return;
        }
        if (startCustomizationDrag(event)) {
            return;
        }
        assignPointer(event);
    },
    { passive: false }
);

controller.addEventListener(
    "pointermove",
    (event) => {
        event.preventDefault();
        if (controllerUiMode !== "NORMAL") {
            moveCustomizationDrag(event);
            return;
        }
        if (moveCustomizationDrag(event)) {
            return;
        }
        movePointer(event);
    },
    { passive: false }
);

controller.addEventListener(
    "pointerup",
    (event) => {
        event.preventDefault();
        if (controllerUiMode !== "NORMAL") {
            endCustomizationDrag(event);
            return;
        }
        if (endCustomizationDrag(event)) {
            return;
        }
        releaseActivePointer(event);
    },
    { passive: false }
);

controller.addEventListener(
    "pointercancel",
    (event) => {
        event.preventDefault();
        if (controllerUiMode !== "NORMAL") {
            endCustomizationDrag(event);
            return;
        }
        if (endCustomizationDrag(event)) {
            return;
        }
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
