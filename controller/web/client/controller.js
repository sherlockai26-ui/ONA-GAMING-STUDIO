// ============================================
// ONA CONTROLLER
// POINTER-ID MULTI-TOUCH INPUT ENGINE v8
// ============================================

console.log("=================================");
console.log("ONA CONTROLLER");
console.log("POINTER MULTI-TOUCH ENGINE v8");
console.log("=================================");

const CONTROLLER_UI_BUILD = "controller-ui-2026-08-16-demo-stabilization-v8";
const CONTROLLER_BUILD_LABEL = "v8";
const CONTROLLER_PROFILE_VERSION = 2;
const CONTROLLER_LAYOUT_SCHEMA = "DEFAULT_CONTROLLER_LAYOUT_V2";
const DEVICE_ID_KEY = "ona.controller.deviceId.v1";
const PROFILE_KEY_PREFIX = "ona.controller.devices.";
const PROFILE_KEY_SUFFIX = ".profile";
const PROFILE_LEGACY_SUFFIX = ".legacy";
const UI_MODES = Object.freeze({
    NORMAL: "NORMAL",
    CUSTOMIZE_MENU: "CUSTOMIZE_MENU",
    EDIT_LAYOUT: "EDIT_LAYOUT",
    COLORS: "COLORS",
    PRESETS: "PRESETS"
});

const PALETTES = Object.freeze({
    cyan: { label: "CYAN", hue: 185 },
    blue: { label: "BLUE", hue: 212 },
    purple: { label: "PURPLE", hue: 274 },
    green: { label: "GREEN", hue: 145 },
    red: { label: "RED", hue: 0 },
    orange: { label: "ORANGE", hue: 30 },
    pink: { label: "PINK", hue: 322 }
});

const SIZE_MIN = 0.7;
const SIZE_MAX = 1.4;
const JOYSTICK_SIZE_MIN = 0.78;
const JOYSTICK_SIZE_MAX = 1.35;
const START_SELECT_MIN_CENTER_DISTANCE = 0.2;
const DEFAULT_CONTROLLER_LAYOUT = Object.freeze({
    controls: {
        JOYSTICK: { x: 0.24, y: 0.6, scale: 1 },
        Y: { x: 0.78, y: 0.35, scale: 1 },
        X: { x: 0.68, y: 0.55, scale: 1 },
        B: { x: 0.88, y: 0.55, scale: 1 },
        A: { x: 0.78, y: 0.75, scale: 1 },
        L1: { x: 0.08, y: 0.12, scale: 1 },
        L2: { x: 0.19, y: 0.12, scale: 1 },
        R1: { x: 0.81, y: 0.12, scale: 1 },
        R2: { x: 0.92, y: 0.12, scale: 1 },
        SELECT: { x: 0.38, y: 0.3, scale: 1 },
        START: { x: 0.62, y: 0.3, scale: 1 }
    }
});

console.log(`[CONTROLLER UI] BUILD VERSION ${CONTROLLER_UI_BUILD}`);

const controller = document.getElementById("controller");
const zone = document.getElementById("joystick-zone");
const joystick = document.getElementById("joystick");
const customizeToggle = document.getElementById("customize-toggle");
const customizePanel = document.getElementById("customize-panel");
const customizeTitle = document.getElementById("customize-title");
const customizeActions = document.getElementById("customize-actions");
const selectedControlLabel = document.getElementById("selected-control-label");
const selectedControlSize = document.getElementById("selected-control-size");
const sizeSlider = document.getElementById("control-size-slider");
const diagnosticsPanel = document.getElementById("controller-diagnostics");

[
    [".shoulder.l1", "L1"],
    [".shoulder.l2", "L2"],
    [".shoulder.r1", "R1"],
    [".shoulder.r2", "R2"],
    ["#select-button", "SELECT"],
    ["#start-button", "START"]
].forEach(([selector, name]) => {
    const button = document.querySelector(selector);

    if (button) {
        button.dataset.button = name;
    }
});

const buttons = [...new Set(document.querySelectorAll("[data-button]"))];
const controlElements = new Map(
    [
        ["JOYSTICK", zone],
        ...buttons.map((button) => [button.dataset.button, button])
    ].filter(([, element]) => Boolean(element))
);

const pairingParams = new URLSearchParams(window.location.search);
const ONA_DIAGNOSTICS =
    pairingParams.get("diag") === "1";
const sessionId = pairingParams.get("id");
const token = pairingParams.get("token");
const deviceId = loadOrCreateDeviceId();
let deviceProfile = loadDeviceProfile(deviceId);

console.log("[ONA CONTROLLER] Current URL:", window.location.href);
console.log("[ONA CONTROLLER] sessionId:", sessionId);
console.log("[ONA CONTROLLER] token:", redactToken(token));
console.log("[ONA CONTROLLER] deviceId:", deviceId);

document.body.classList.toggle("controller-diagnostics-enabled", ONA_DIAGNOSTICS);
document.body.dataset.controllerBuild = CONTROLLER_BUILD_LABEL;

const websocketScheme = window.location.protocol === "https:" ? "wss" : "ws";
const websocketPort = window.location.port || "8080";
const wsUrl = `${websocketScheme}://${window.location.hostname}:${websocketPort}/ws`;

console.log("[ONA CONTROLLER] WebSocket URL:", wsUrl);
diagnoseEditElement("startup");

const reconnectBackoffMs = [500, 1000, 2000, 3000, 5000];
let controllerSocket = null;
let reconnectAttempt = 0;
let reconnectTimer = null;
let heartbeatTimer = null;
let reconnectEnabled = true;
let playerId = null;
let lastTouchEndAt = 0;

const activePointers = new Map();
const activeButtonCounts = new Map();
let joystickPointerId = null;
let controllerUiMode = UI_MODES.NORMAL;
let selectedControlId = "JOYSTICK";
let draftProfile = null;
let editorPointer = null;
let editPointerId = null;
let editActivatedAt = 0;
let lastLayoutDiagnostics = null;
let lastEditDiagnostics = null;
let controllerReady = false;
let viewportRenderTimer = null;

function sendControllerMessage(message) {
    if (!controllerReady && message.type === "input") {
        return;
    }

    if (controllerUiMode !== UI_MODES.NORMAL && message.type === "input") {
        return;
    }

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

    controllerSocket.addEventListener("open", () => {
        console.log("[ONA CONTROLLER] WebSocket open");

        sendControllerMessage({
            type: "controller_connected",
            sessionId,
            token,
            deviceId
        });
        startHeartbeat();
    });

    controllerSocket.addEventListener("message", (event) => {
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
            console.log("[CONTROLLER] authenticated deviceId=", message.deviceId || deviceId);
            setControllerConnectionState("CONNECTED");
            return;
        }

        if (message.type === "controller_auth_rejected") {
            reconnectEnabled = false;
            stopHeartbeat();
            console.warn("[CONTROLLER] reconnect rejected", message.reason);

            if (String(message.reason || "").toUpperCase().includes("INVALID")) {
                window.localStorage.removeItem("onaControllerSession");
                window.localStorage.removeItem("onaControllerPairing");
                setControllerConnectionState("PAIRING REQUIRED");
                console.warn("[CONTROLLER] invalid stored pairing cleared; device profile preserved");
                return;
            }

            setControllerConnectionState("SESSION EXPIRED");
        }
    });

    controllerSocket.addEventListener("error", (event) => {
        console.error("[ONA CONTROLLER] WebSocket error:", event);
    });

    controllerSocket.addEventListener("close", (event) => {
        console.warn("[ONA CONTROLLER] WebSocket close:", event.code, event.reason);
        console.warn("[CONTROLLER] websocket lost");
        stopHeartbeat();
        neutralizeLocalInputState(true);
        scheduleReconnect();
    });
}

function scheduleReconnect() {
    if (!reconnectEnabled || reconnectTimer) {
        return;
    }

    setControllerConnectionState("RECONNECTING...");
    const delay = reconnectBackoffMs[Math.min(reconnectAttempt, reconnectBackoffMs.length - 1)];

    reconnectAttempt += 1;
    console.log("[CONTROLLER] reconnect attempt=", reconnectAttempt);

    reconnectTimer = window.setTimeout(() => {
        reconnectTimer = null;
        connectWebSocket();
    }, delay);
}

function startHeartbeat() {
    stopHeartbeat();
    heartbeatTimer = window.setInterval(() => {
        sendControllerMessage({
            type: "ping",
            timestamp: Date.now()
        });
    }, 5000);
}

function stopHeartbeat() {
    if (heartbeatTimer) {
        window.clearInterval(heartbeatTimer);
        heartbeatTimer = null;
    }
}

function loadOrCreateDeviceId() {
    const stored = safeLocalStorageGet(DEVICE_ID_KEY);

    if (stored && /^ona-device-[0-9a-f-]{16,}$/i.test(stored)) {
        return stored;
    }

    const id = `ona-device-${secureUuid()}`;
    safeLocalStorageSet(DEVICE_ID_KEY, id);
    return id;
}

function secureUuid() {
    if (window.crypto?.randomUUID) {
        return window.crypto.randomUUID();
    }

    if (!window.crypto?.getRandomValues) {
        throw new Error("ONA Controller Device Identity V1 requires Web Crypto support.");
    }

    const bytes = new Uint8Array(16);
    window.crypto.getRandomValues(bytes);

    bytes[6] = (bytes[6] & 0x0f) | 0x40;
    bytes[8] = (bytes[8] & 0x3f) | 0x80;
    const hex = [...bytes].map((byte) => byte.toString(16).padStart(2, "0"));
    return `${hex.slice(0, 4).join("")}-${hex.slice(4, 6).join("")}-${hex.slice(6, 8).join("")}-${hex.slice(8, 10).join("")}-${hex.slice(10).join("")}`;
}

function profileStorageKey(id) {
    return `${PROFILE_KEY_PREFIX}${id}${PROFILE_KEY_SUFFIX}`;
}

function redactToken(value) {
    if (!value) {
        return "none";
    }

    const text = String(value);
    return `${text.slice(0, 4)}****${text.slice(-4)}`;
}

function safeLocalStorageGet(key) {
    try {
        return window.localStorage.getItem(key);
    }
    catch {
        return null;
    }
}

function safeLocalStorageSet(key, value) {
    try {
        window.localStorage.setItem(key, value);
    }
    catch (error) {
        console.warn("[CONTROLLER PROFILE] storage write failed:", error);
    }
}

function defaultProfile(id) {
    return {
        version: CONTROLLER_PROFILE_VERSION,
        deviceId: id,
        displayName: "ONA Controller",
        layoutPreset: "DEFAULT",
        layoutSchema: CONTROLLER_LAYOUT_SCHEMA,
        customValid: false,
        customLayout: defaultLayout(),
        palette: "cyan",
        glowIntensity: 1,
        preferences: {
            selectedControlId: "JOYSTICK"
        }
    };
}

function defaultLayout() {
    return clone(DEFAULT_CONTROLLER_LAYOUT);
}

function loadDeviceProfile(id) {
    try {
        const parsed = JSON.parse(safeLocalStorageGet(profileStorageKey(id)) || "null");
        return normalizeProfile(parsed, id);
    }
    catch {
        return normalizeProfile(null, id);
    }
}

function saveDeviceProfile(profile) {
    safeLocalStorageSet(profileStorageKey(profile.deviceId), JSON.stringify(normalizeProfile(profile, profile.deviceId)));
}

function normalizeProfile(profile, id) {
    const fallback = defaultProfile(id);
    const layoutPreset = normalizePreset(profile?.layoutPreset || fallback.layoutPreset);
    const normalized = {
        version: CONTROLLER_PROFILE_VERSION,
        deviceId: id,
        displayName: typeof profile?.displayName === "string" ? profile.displayName : fallback.displayName,
        layoutPreset,
        layoutSchema: CONTROLLER_LAYOUT_SCHEMA,
        customValid: false,
        customLayout: defaultLayout(),
        palette: PALETTES[profile?.palette] ? profile.palette : fallback.palette,
        glowIntensity: clamp(Number(profile?.glowIntensity) || fallback.glowIntensity, 0.6, 1.4),
        preferences: {
            selectedControlId:
                controlElements.has(profile?.preferences?.selectedControlId)
                    ? profile.preferences.selectedControlId
                    : fallback.preferences.selectedControlId
        }
    };

    if (profileHasConfirmedCustomLayout(profile)) {
        const validation = validateLayoutContract(profile.customLayout);

        if (validation.valid) {
            normalized.layoutPreset = layoutPreset === "DEFAULT" ? "CUSTOM" : layoutPreset;
            normalized.customValid = true;
            normalized.customLayout = normalizeLayout(profile.customLayout);
        }
        else {
            backupLegacyProfile(id, profile, `invalid_confirmed_custom_layout:${validation.reasons.join(",")}`);
            console.warn(
                `[CONTROLLER LAYOUT] confirmed custom layout rejected; loading DEFAULT reason=${validation.reasons.join(",")}`
            );
        }
    }
    else if (profileContainsLegacyLayout(profile)) {
        backupLegacyProfile(id, profile, "legacy_or_unconfirmed_layout");
        console.warn("[CONTROLLER LAYOUT] legacy/unconfirmed layout ignored; loading DEFAULT");
    }

    validateControllerLayout(normalized);
    return normalized;
}

function profileHasConfirmedCustomLayout(profile) {
    return (
        profile?.customValid === true &&
        profile?.layoutSchema === CONTROLLER_LAYOUT_SCHEMA &&
        Boolean(profile?.customLayout?.controls)
    );
}

function profileContainsLegacyLayout(profile) {
    if (!profile || typeof profile !== "object") {
        return false;
    }

    if (
        profile.version === CONTROLLER_PROFILE_VERSION &&
        profile.layoutSchema === CONTROLLER_LAYOUT_SCHEMA &&
        profile.layoutPreset === "DEFAULT" &&
        profile.customValid === false
    ) {
        return false;
    }

    return Boolean(
        profile.customLayout ||
        profile.controls ||
        profile.START ||
        profile.SELECT ||
        profile.JOYSTICK
    );
}

function backupLegacyProfile(id, profile, reason) {
    if (!profile || typeof profile !== "object") {
        return;
    }

    const key = `${profileStorageKey(id)}${PROFILE_LEGACY_SUFFIX}.${CONTROLLER_UI_BUILD}`;

    if (safeLocalStorageGet(key)) {
        return;
    }

    safeLocalStorageSet(
        key,
        JSON.stringify({
            reason,
            savedAt: new Date().toISOString(),
            profile
        })
    );
    console.warn(`[CONTROLLER LAYOUT] legacy profile copied key=${key} reason=${reason}`);
}

function normalizePreset(preset) {
    return ["DEFAULT", "COMPACT", "LARGE_BUTTONS", "LEFT_HANDED", "CUSTOM"].includes(preset)
        ? preset
        : "DEFAULT";
}

function normalizeLayout(layout) {
    const fallback = defaultLayout();
    const controls = {};

    for (const id of controlElements.keys()) {
        const control = layout?.controls?.[id] || layout?.[id] || null;
        const defaultControl = fallback.controls[id];
        const minSize = id === "JOYSTICK" ? JOYSTICK_SIZE_MIN : SIZE_MIN;
        const maxSize = id === "JOYSTICK" ? JOYSTICK_SIZE_MAX : SIZE_MAX;
        const x = Number(control?.x);
        const y = Number(control?.y);
        const scale = Number(control?.scale);

        controls[id] = {
            x: Number.isFinite(x) ? clamp(x, 0.05, 0.95) : defaultControl.x,
            y: Number.isFinite(y) ? clamp(y, 0.08, 0.9) : defaultControl.y,
            scale: Number.isFinite(scale) ? clamp(scale, minSize, maxSize) : defaultControl.scale
        };
    }

    return { controls };
}

function validateLayoutContract(layout) {
    const reasons = [];
    const controls = layout?.controls;

    if (!controls || typeof controls !== "object") {
        return {
            valid: false,
            reasons: ["missing_controls"]
        };
    }

    for (const id of controlElements.keys()) {
        const control = controls[id];
        const minSize = id === "JOYSTICK" ? JOYSTICK_SIZE_MIN : SIZE_MIN;
        const maxSize = id === "JOYSTICK" ? JOYSTICK_SIZE_MAX : SIZE_MAX;

        if (!control) {
            reasons.push(`missing_${id}`);
            continue;
        }

        const x = Number(control.x);
        const y = Number(control.y);
        const scale = Number(control.scale);

        if (!Number.isFinite(x) || x < 0.05 || x > 0.95) {
            reasons.push(`invalid_${id}_x`);
        }

        if (!Number.isFinite(y) || y < 0.08 || y > 0.9) {
            reasons.push(`invalid_${id}_y`);
        }

        if (!Number.isFinite(scale) || scale < minSize || scale > maxSize) {
            reasons.push(`invalid_${id}_scale`);
        }
    }

    const start = controls.START;
    const select = controls.SELECT;

    if (start && select) {
        const distance = Math.hypot(Number(start.x) - Number(select.x), Number(start.y) - Number(select.y));

        if (!Number.isFinite(distance) || distance < START_SELECT_MIN_CENTER_DISTANCE) {
            reasons.push("start_select_too_close");
        }
    }

    return {
        valid: reasons.length === 0,
        reasons
    };
}

function validateControllerLayout(profile) {
    const validation = validateLayoutContract(profile.customLayout);

    if (!validation.valid) {
        profile.layoutPreset = "DEFAULT";
        profile.customValid = false;
        profile.customLayout = defaultLayout();
        console.warn(
            `[CONTROLLER LAYOUT] invalid active layout replaced with DEFAULT reason=${validation.reasons.join(",")}`
        );
    }

    const controls = profile.customLayout.controls;

    console.log(
        `[CONTROLLER LAYOUT] START x=${controls.START.x.toFixed(3)} y=${controls.START.y.toFixed(3)} scale=${controls.START.scale.toFixed(2)}`
    );
    console.log(
        `[CONTROLLER LAYOUT] SELECT x=${controls.SELECT.x.toFixed(3)} y=${controls.SELECT.y.toFixed(3)} scale=${controls.SELECT.scale.toFixed(2)}`
    );

}

function clone(value) {
    return JSON.parse(JSON.stringify(value));
}

function clamp(value, min, max) {
    const number = Number.isFinite(value) ? value : min;
    return Math.min(max, Math.max(min, number));
}

function activeProfile() {
    return draftProfile || deviceProfile;
}

function controllerSafeArea() {
    const rect = controller.getBoundingClientRect();
    const style = window.getComputedStyle(controller);
    const paddingLeft = Number.parseFloat(style.paddingLeft) || 0;
    const paddingRight = Number.parseFloat(style.paddingRight) || 0;
    const paddingTop = Number.parseFloat(style.paddingTop) || 0;
    const paddingBottom = Number.parseFloat(style.paddingBottom) || 0;
    const visualOffsetLeft = window.visualViewport?.offsetLeft || 0;
    const visualOffsetTop = window.visualViewport?.offsetTop || 0;

    return {
        usableLeft: rect.left + visualOffsetLeft + paddingLeft,
        usableTop: rect.top + visualOffsetTop + paddingTop,
        usableWidth: Math.max(1, rect.width - paddingLeft - paddingRight),
        usableHeight: Math.max(1, rect.height - paddingTop - paddingBottom)
    };
}

function projectControlToViewport(control, safeArea = controllerSafeArea()) {
    const x = clamp(Number(control?.x), 0.05, 0.95);
    const y = clamp(Number(control?.y), 0.08, 0.9);

    return {
        x,
        y,
        left: safeArea.usableLeft + safeArea.usableWidth * x,
        top: safeArea.usableTop + safeArea.usableHeight * y
    };
}

function applyProfileToController() {
    const profile = activeProfile();
    const palette = PALETTES[profile.palette] || PALETTES.cyan;
    const safeArea = controllerSafeArea();

    document.body.dataset.controllerPalette = profile.palette;
    document.body.style.setProperty("--ona-accent-hue", String(palette.hue));
    document.body.style.setProperty("--ona-glow-intensity", String(profile.glowIntensity));

    for (const [id, element] of controlElements) {
        const control = profile.customLayout.controls[id];
        const projected = projectControlToViewport(control, safeArea);
        element.classList.add("ona-customizable");
        element.classList.toggle(
            "customization-selected",
            controllerUiMode === UI_MODES.EDIT_LAYOUT && selectedControlId === id
        );
        element.style.setProperty("--ona-control-left", `${projected.left}px`);
        element.style.setProperty("--ona-control-top", `${projected.top}px`);
        element.style.setProperty("--ona-control-scale", String(control.scale));

        if (id === "START" || id === "SELECT") {
            console.log(
                `[CONTROLLER LAYOUT] ${id} raw x=${control.x.toFixed(3)} y=${control.y.toFixed(3)} projected left=${Math.round(projected.left)} top=${Math.round(projected.top)}`
            );
        }
    }

    renderCustomizePanel();
    updateControllerDiagnostics("apply-profile");
}

function rectSummary(rect) {
    if (!rect) {
        return "none";
    }

    return `${Math.round(rect.left)},${Math.round(rect.top)} ${Math.round(rect.width)}x${Math.round(rect.height)}`;
}

function rectCenter(rect) {
    return {
        x: rect.left + rect.width / 2,
        y: rect.top + rect.height / 2
    };
}

function rectsOverlap(a, b) {
    return !(
        a.right <= b.left ||
        b.right <= a.left ||
        a.bottom <= b.top ||
        b.bottom <= a.top
    );
}

function measureStartSelectLayout() {
    const startElement = controlElements.get("START");
    const selectElement = controlElements.get("SELECT");
    const profile = activeProfile();
    const safeArea = controllerSafeArea();
    const startRaw = profile.customLayout.controls.START;
    const selectRaw = profile.customLayout.controls.SELECT;
    const startProjected = projectControlToViewport(startRaw, safeArea);
    const selectProjected = projectControlToViewport(selectRaw, safeArea);

    if (!startElement || !selectElement) {
        return {
            valid: false,
            reason: "missing_start_select_elements"
        };
    }

    const startRect = startElement.getBoundingClientRect();
    const selectRect = selectElement.getBoundingClientRect();
    const startStyle = window.getComputedStyle(startElement);
    const selectStyle = window.getComputedStyle(selectElement);
    const startCenter = rectCenter(startRect);
    const selectCenter = rectCenter(selectRect);
    const distance = Math.hypot(startCenter.x - selectCenter.x, startCenter.y - selectCenter.y);
    const overlap = rectsOverlap(startRect, selectRect);

    return {
        valid: !overlap,
        uniqueElements: startElement !== selectElement,
        startRaw,
        selectRaw,
        startProjected,
        selectProjected,
        startStyle: {
            position: startStyle.position,
            left: startStyle.left,
            top: startStyle.top,
            right: startStyle.right,
            bottom: startStyle.bottom,
            transform: startStyle.transform,
            zIndex: startStyle.zIndex
        },
        selectStyle: {
            position: selectStyle.position,
            left: selectStyle.left,
            top: selectStyle.top,
            right: selectStyle.right,
            bottom: selectStyle.bottom,
            transform: selectStyle.transform,
            zIndex: selectStyle.zIndex
        },
        startRect,
        selectRect,
        distance,
        overlap
    };
}

function updateControllerDiagnostics(reason = "update") {
    const update = () => {
        lastLayoutDiagnostics = measureStartSelectLayout();

        if (lastLayoutDiagnostics.overlap && activeProfile().layoutPreset === "DEFAULT") {
            console.error("[CONTROLLER LAYOUT] DEFAULT_START_SELECT_OVERLAP");
        }

        document.body.dataset.controllerLayoutValid =
            String(Boolean(lastLayoutDiagnostics.valid));

        console.log(
            `[CONTROLLER LAYOUT] DOM reason=${reason} START rect=${rectSummary(lastLayoutDiagnostics.startRect)} SELECT rect=${rectSummary(lastLayoutDiagnostics.selectRect)} distance=${Math.round(lastLayoutDiagnostics.distance || 0)} overlap=${Boolean(lastLayoutDiagnostics.overlap)}`
        );
        console.log(
            `[CONTROLLER LAYOUT] START raw x=${lastLayoutDiagnostics.startRaw?.x} y=${lastLayoutDiagnostics.startRaw?.y} projected left=${Math.round(lastLayoutDiagnostics.startProjected?.left || 0)} top=${Math.round(lastLayoutDiagnostics.startProjected?.top || 0)} style left=${lastLayoutDiagnostics.startStyle?.left} top=${lastLayoutDiagnostics.startStyle?.top} transform=${lastLayoutDiagnostics.startStyle?.transform}`
        );
        console.log(
            `[CONTROLLER LAYOUT] SELECT raw x=${lastLayoutDiagnostics.selectRaw?.x} y=${lastLayoutDiagnostics.selectRaw?.y} projected left=${Math.round(lastLayoutDiagnostics.selectProjected?.left || 0)} top=${Math.round(lastLayoutDiagnostics.selectProjected?.top || 0)} style left=${lastLayoutDiagnostics.selectStyle?.left} top=${lastLayoutDiagnostics.selectStyle?.top} transform=${lastLayoutDiagnostics.selectStyle?.transform}`
        );
        console.log(
            `[CONTROLLER LAYOUT] START/SELECT unique=${Boolean(lastLayoutDiagnostics.uniqueElements)}`
        );

        renderDiagnosticsPanel();
    };

    if (window.requestAnimationFrame) {
        window.requestAnimationFrame(update);
    }
    else {
        update();
    }
}

function validateControllerReadyLayout() {
    lastLayoutDiagnostics = measureStartSelectLayout();

    if (!lastLayoutDiagnostics.valid || !lastLayoutDiagnostics.uniqueElements) {
        return {
            valid: false,
            reason: `start_select invalid overlap=${Boolean(lastLayoutDiagnostics.overlap)} unique=${Boolean(lastLayoutDiagnostics.uniqueElements)}`
        };
    }

    for (const [id, element] of controlElements) {
        const rect = element.getBoundingClientRect();

        if (
            !Number.isFinite(rect.left) ||
            !Number.isFinite(rect.top) ||
            rect.width <= 0 ||
            rect.height <= 0
        ) {
            return {
                valid: false,
                reason: `invalid_rect_${id}`
            };
        }
    }

    return {
        valid: true,
        reason: "ok"
    };
}

function markControllerReady() {
    controllerReady = true;
    document.body.classList.remove("controller-booting");
    document.body.classList.add("controller-ready");
    console.log("[CONTROLLER BOOT] READY");
}

function waitForStableViewportFrame() {
    return new Promise((resolve) => {
        window.requestAnimationFrame(() => {
            window.requestAnimationFrame(resolve);
        });
    });
}

async function bootstrapController() {
    console.log("[CONTROLLER BOOT] start");
    console.log("[CONTROLLER BOOT] device profile loaded");
    await waitForStableViewportFrame();
    console.log(
        `[CONTROLLER BOOT] safe area ${JSON.stringify(controllerSafeArea())}`
    );
    applyProfileToController();
    await waitForStableViewportFrame();
    console.log("[CONTROLLER BOOT] layout projected");
    const validation = validateControllerReadyLayout();

    if (!validation.valid) {
        console.warn(`[CONTROLLER BOOT] validation failed reason=${validation.reason}`);
        deviceProfile.customValid = false;
        deviceProfile.layoutPreset = "DEFAULT";
        deviceProfile.customLayout = defaultLayout();
        applyProfileToController();
        await waitForStableViewportFrame();
    }

    const finalValidation = validateControllerReadyLayout();

    if (!finalValidation.valid) {
        console.error(`[CONTROLLER BOOT] validation failed reason=${finalValidation.reason}`);
    }
    else {
        console.log("[CONTROLLER BOOT] validation pass");
    }

    renderDiagnosticsPanel();
    markControllerReady();
    connectWebSocket();
}

function renderDiagnosticsPanel() {
    if (!diagnosticsPanel || !ONA_DIAGNOSTICS) {
        return;
    }

    const layout = lastLayoutDiagnostics || {};
    const edit = lastEditDiagnostics || {};

    diagnosticsPanel.textContent = [
        `build ${CONTROLLER_BUILD_LABEL}`,
        `mode ${controllerUiMode}`,
        `START raw ${layout.startRaw?.x ?? "?"}/${layout.startRaw?.y ?? "?"}`,
        `SELECT raw ${layout.selectRaw?.x ?? "?"}/${layout.selectRaw?.y ?? "?"}`,
        `START projected ${Math.round(layout.startProjected?.left || 0)},${Math.round(layout.startProjected?.top || 0)}`,
        `SELECT projected ${Math.round(layout.selectProjected?.left || 0)},${Math.round(layout.selectProjected?.top || 0)}`,
        `START rect ${rectSummary(layout.startRect)}`,
        `SELECT rect ${rectSummary(layout.selectRect)}`,
        `distance ${Math.round(layout.distance || 0)}px`,
        `overlap ${Boolean(layout.overlap)}`,
        `unique ${Boolean(layout.uniqueElements)}`,
        `EDIT top ${edit.topElement || "unknown"}`,
        `EDIT rect ${rectSummary(edit.rect)}`,
        `TOP rect ${rectSummary(edit.topRect)}`,
        `TOP pe ${edit.topPointerEvents || "?"}`,
        `TOP z ${edit.topZIndex || "?"}`,
        `EDIT event ${edit.reason || "none"}`
    ].join("\n");
}

function setControllerUiMode(mode) {
    const previousMode = controllerUiMode;
    controllerUiMode = mode || UI_MODES.NORMAL;
    console.log(`[CONTROLLER UI] mode ${previousMode} -> ${controllerUiMode}`);
    console.log(`[CONTROLLER UI] mode=${controllerUiMode}`);

    if (controllerUiMode === UI_MODES.EDIT_LAYOUT) {
        console.log("[CONTROLLER UI] entering EDIT_LAYOUT");
        draftProfile = clone(deviceProfile);
        selectedControlId = draftProfile.preferences.selectedControlId || selectedControlId;
        neutralizeLocalInputState(true);
    }

    if (controllerUiMode !== UI_MODES.EDIT_LAYOUT) {
        editorPointer = null;
    }

    if (controllerUiMode === UI_MODES.NORMAL) {
        draftProfile = null;
        selectedControlId = deviceProfile.preferences.selectedControlId || selectedControlId;
    }

    document.body.classList.toggle("controller-customizing", controllerUiMode !== UI_MODES.NORMAL);
    document.body.dataset.controllerUiMode = controllerUiMode;

    if (customizePanel) {
        customizePanel.hidden = controllerUiMode === UI_MODES.NORMAL;
    }

    applyProfileToController();
}

function renderCustomizePanel() {
    if (!customizeActions || !customizeTitle) {
        return;
    }

    customizeActions.innerHTML = "";

    const actionsByMode = {
        [UI_MODES.CUSTOMIZE_MENU]: [
            ["edit-layout", "EDIT LAYOUT"],
            ["colors", "COLORS"],
            ["presets", "PRESETS"],
            ["reset-default", "RESET DEFAULT"],
            ["back", "BACK"]
        ],
        [UI_MODES.EDIT_LAYOUT]: [
            ["done", "DONE"],
            ["cancel", "CANCEL"],
            ["reset-draft", "RESET"],
            ["back-menu", "MENU"]
        ],
        [UI_MODES.COLORS]: [
            ...Object.values(PALETTES).map((palette) => [`palette-${palette.label.toLowerCase()}`, palette.label]),
            ["back-menu", "BACK"]
        ],
        [UI_MODES.PRESETS]: [
            ["preset-default", "DEFAULT"],
            ["preset-compact", "COMPACT"],
            ["preset-large-buttons", "LARGE BUTTONS"],
            ["preset-left-handed", "LEFT HANDED"],
            ["back-menu", "BACK"]
        ]
    };

    const titles = {
        [UI_MODES.CUSTOMIZE_MENU]: "CUSTOMIZE CONTROLLER",
        [UI_MODES.EDIT_LAYOUT]: "EDIT MODE",
        [UI_MODES.COLORS]: "COLORS",
        [UI_MODES.PRESETS]: "PRESETS"
    };

    customizeTitle.textContent = titles[controllerUiMode] || "CUSTOMIZE CONTROLLER";

    for (const [action, label] of actionsByMode[controllerUiMode] || []) {
        const button = document.createElement("button");
        button.type = "button";
        button.dataset.customizeAction = action;
        button.textContent = label;
        customizeActions.appendChild(button);
    }

    const profile = activeProfile();
    const selected = profile.customLayout.controls[selectedControlId];
    const editorToolsVisible = controllerUiMode === UI_MODES.EDIT_LAYOUT;

    if (selectedControlLabel) {
        selectedControlLabel.textContent = editorToolsVisible ? selectedControlId : "";
    }

    if (selectedControlSize) {
        selectedControlSize.textContent = editorToolsVisible && selected ? `${selected.scale.toFixed(2)}x` : "";
    }

    if (sizeSlider) {
        sizeSlider.hidden = !editorToolsVisible;
        sizeSlider.disabled = !editorToolsVisible;
        if (selected) {
            sizeSlider.min = selectedControlId === "JOYSTICK" ? JOYSTICK_SIZE_MIN : SIZE_MIN;
            sizeSlider.max = selectedControlId === "JOYSTICK" ? JOYSTICK_SIZE_MAX : SIZE_MAX;
            sizeSlider.value = selected.scale;
        }
    }
}

function handleCustomizeAction(action) {
    if (!action) {
        return;
    }

    if (action === "edit-layout") {
        setControllerUiMode(UI_MODES.EDIT_LAYOUT);
        return;
    }

    if (action === "colors") {
        setControllerUiMode(UI_MODES.COLORS);
        return;
    }

    if (action === "presets") {
        setControllerUiMode(UI_MODES.PRESETS);
        return;
    }

    if (action === "back" || action === "cancel") {
        console.log("[CONTROLLER UI] layout cancelled");
        setControllerUiMode(UI_MODES.NORMAL);
        return;
    }

    if (action === "back-menu") {
        setControllerUiMode(UI_MODES.CUSTOMIZE_MENU);
        return;
    }

    if (action === "done") {
        const confirmed = clone(draftProfile || deviceProfile);
        const validation = validateLayoutContract(confirmed.customLayout);

        if (!validation.valid) {
            console.warn(
                `[CONTROLLER UI] layout not saved reason=${validation.reasons.join(",")}`
            );
            return;
        }

        confirmed.layoutPreset = "CUSTOM";
        confirmed.layoutSchema = CONTROLLER_LAYOUT_SCHEMA;
        confirmed.customValid = true;
        confirmed.preferences.selectedControlId = selectedControlId;
        deviceProfile = normalizeProfile(confirmed, deviceId);
        saveDeviceProfile(deviceProfile);
        console.log("[CONTROLLER UI] layout saved");
        setControllerUiMode(UI_MODES.NORMAL);
        return;
    }

    if (action === "reset-default" || action === "reset-draft") {
        const target = draftProfile || deviceProfile;
        target.layoutPreset = "DEFAULT";
        target.layoutSchema = CONTROLLER_LAYOUT_SCHEMA;
        target.customValid = false;
        target.customLayout = defaultLayout();
        target.preferences.selectedControlId = "JOYSTICK";
        selectedControlId = "JOYSTICK";

        if (!draftProfile) {
            saveDeviceProfile(target);
        }

        applyProfileToController();
        return;
    }

    if (action.startsWith("palette-")) {
        const palette = action.replace("palette-", "");
        const target = draftProfile || deviceProfile;

        if (PALETTES[palette]) {
            target.palette = palette;
            if (!draftProfile) {
                saveDeviceProfile(target);
            }
            applyProfileToController();
        }
        return;
    }

    if (action.startsWith("preset-")) {
        const preset = action.replace("preset-", "");
        applyPreset(preset);
    }
}

function applyPreset(name) {
    const target = draftProfile || deviceProfile;
    target.layoutSchema = CONTROLLER_LAYOUT_SCHEMA;
    target.customValid = true;
    target.customLayout = defaultLayout();

    if (name === "compact") {
        target.layoutPreset = "COMPACT";
        for (const control of Object.values(target.customLayout.controls)) {
            control.scale = 0.86;
        }
    } else if (name === "large-buttons") {
        target.layoutPreset = "LARGE_BUTTONS";
        for (const [id, control] of Object.entries(target.customLayout.controls)) {
            control.scale = id === "JOYSTICK" ? 1.08 : 1.24;
        }
    } else if (name === "left-handed") {
        target.layoutPreset = "LEFT_HANDED";
        target.customLayout.controls.JOYSTICK.x = 0.76;
        for (const id of ["A", "B", "X", "Y"]) {
            target.customLayout.controls[id].x = 1 - target.customLayout.controls[id].x;
        }
    } else {
        target.layoutPreset = "DEFAULT";
    }

    validateControllerLayout(target);

    if (!draftProfile) {
        saveDeviceProfile(target);
    }

    applyProfileToController();
}

function startEditorDrag(event) {
    if (controllerUiMode !== UI_MODES.EDIT_LAYOUT || editorPointer) {
        return false;
    }

    const target = event.target?.closest?.(".ona-customizable");

    if (!target) {
        return false;
    }

    const entry = [...controlElements.entries()].find(([, element]) => element === target);

    if (!entry) {
        return false;
    }

    const [id, element] = entry;
    selectedControlId = id;
    draftProfile.preferences.selectedControlId = id;
    const control = draftProfile.customLayout.controls[id];
    const safeArea = controllerSafeArea();

    editorPointer = {
        pointerId: event.pointerId,
        id,
        element,
        safeArea,
        startX: event.clientX,
        startY: event.clientY,
        originalX: control.x,
        originalY: control.y
    };

    console.log(`[CONTROLLER UI] selected=${id}`);
    capturePointer(element, event.pointerId);
    applyProfileToController();
    return true;
}

function moveEditorDrag(event) {
    if (!editorPointer || editorPointer.pointerId !== event.pointerId) {
        return false;
    }

    const control = draftProfile.customLayout.controls[editorPointer.id];
    const deltaX = (event.clientX - editorPointer.startX) / editorPointer.safeArea.usableWidth;
    const deltaY = (event.clientY - editorPointer.startY) / editorPointer.safeArea.usableHeight;

    control.x = clamp(editorPointer.originalX + deltaX, 0.05, 0.95);
    control.y = clamp(editorPointer.originalY + deltaY, 0.08, 0.9);
    draftProfile.layoutPreset = "CUSTOM";
    console.log(
        `[CONTROLLER UI] drag ${editorPointer.id} x=${control.x.toFixed(3)} y=${control.y.toFixed(3)}`
    );
    applyProfileToController();
    return true;
}

function endEditorDrag(event) {
    if (!editorPointer || editorPointer.pointerId !== event.pointerId) {
        return false;
    }

    releasePointer(editorPointer.element, event.pointerId);
    editorPointer = null;
    return true;
}

function setEditPressed(pressed, reason) {
    customizeToggle?.classList.toggle("edit-pressed", pressed);
    lastEditDiagnostics = {
        ...(lastEditDiagnostics || {}),
        pressed,
        reason
    };
    renderDiagnosticsPanel();
}

function openCustomizeMenuFromEdit(source) {
    const now = Date.now();

    if (now - editActivatedAt < 360) {
        console.log(`[CONTROLLER UI] EDIT duplicate ignored source=${source}`);
        return;
    }

    editActivatedAt = now;
    console.log(`[CONTROLLER UI] EDIT open source=${source}`);
    setControllerUiMode(
        controllerUiMode === UI_MODES.NORMAL
            ? UI_MODES.CUSTOMIZE_MENU
            : UI_MODES.NORMAL
    );
}

function handleEditPressStart(event, source) {
    event.preventDefault();
    event.stopPropagation();
    editPointerId = event.pointerId ?? "touch";
    console.log(`[CONTROLLER UI] EDIT ${source} down`);
    setEditPressed(true, `${source}-down`);
    diagnoseEditElement(`${source}-down`, event);

    if (typeof editPointerId === "number") {
        capturePointer(customizeToggle, editPointerId);
    }
}

function handleEditPressEnd(event, source) {
    event.preventDefault();
    event.stopPropagation();

    const pointerId = event.pointerId ?? "touch";

    if (editPointerId !== null && editPointerId !== pointerId) {
        return;
    }

    console.log(`[CONTROLLER UI] EDIT ${source} up`);

    if (
        event.clientX !== undefined &&
        event.clientY !== undefined &&
        !pointInsideElement(event.clientX, event.clientY, customizeToggle)
    ) {
        console.log("[CONTROLLER UI] EDIT pointerup outside ignored");
        editPointerId = null;
        setEditPressed(false, `${source}-up-outside`);
        return;
    }

    if (typeof pointerId === "number") {
        releasePointer(customizeToggle, pointerId);
    }

    editPointerId = null;
    setEditPressed(false, `${source}-up`);
    openCustomizeMenuFromEdit(source);
}

if ("PointerEvent" in window) {
    customizeToggle?.addEventListener(
        "pointerdown",
        (event) => handleEditPressStart(event, "pointer"),
        { passive: false }
    );

    customizeToggle?.addEventListener(
        "pointerup",
        (event) => handleEditPressEnd(event, "pointer"),
        { passive: false }
    );

    customizeToggle?.addEventListener(
        "pointercancel",
        (event) => {
            event.preventDefault();
            event.stopPropagation();
            releasePointer(customizeToggle, event.pointerId);
            editPointerId = null;
            setEditPressed(false, "pointer-cancel");
        },
        { passive: false }
    );
}
else {
    customizeToggle?.addEventListener(
        "touchstart",
        (event) => handleEditPressStart(event, "touch"),
        { passive: false }
    );

    customizeToggle?.addEventListener(
        "touchend",
        (event) => handleEditPressEnd(event, "touch"),
        { passive: false }
    );
}

customizeToggle?.addEventListener(
    "click",
    (event) => {
        event.preventDefault();
        event.stopPropagation();
        console.log("[CONTROLLER UI] EDIT click");

        if (!("PointerEvent" in window)) {
            openCustomizeMenuFromEdit("click");
        }
    },
    { passive: false }
);

customizePanel?.addEventListener(
    "pointerdown",
    (event) => {
        event.stopPropagation();

        if (event.target?.matches?.("input")) {
            return;
        }

        event.preventDefault();
        handleCustomizeAction(event.target?.dataset?.customizeAction);
    },
    { passive: false }
);

sizeSlider?.addEventListener("input", (event) => {
    if (controllerUiMode !== UI_MODES.EDIT_LAYOUT || !draftProfile) {
        return;
    }

    const control = draftProfile.customLayout.controls[selectedControlId];

    if (!control) {
        return;
    }

    const minSize = selectedControlId === "JOYSTICK" ? JOYSTICK_SIZE_MIN : SIZE_MIN;
    const maxSize = selectedControlId === "JOYSTICK" ? JOYSTICK_SIZE_MAX : SIZE_MAX;
    control.scale = clamp(Number(event.target.value), minSize, maxSize);
    draftProfile.layoutPreset = "CUSTOM";
    console.log(`[CONTROLLER UI] resize ${selectedControlId} size=${control.scale.toFixed(2)}`);
    applyProfileToController();
});

document.addEventListener("gesturestart", (event) => event.preventDefault(), { passive: false });
document.addEventListener("gesturechange", (event) => event.preventDefault(), { passive: false });
document.addEventListener("gestureend", (event) => event.preventDefault(), { passive: false });
document.addEventListener("contextmenu", (event) => event.preventDefault());

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
        const now = Date.now();

        if (now - lastTouchEndAt < 320) {
            event.preventDefault();
        }

        lastTouchEndAt = now;
    },
    { passive: false }
);

document.addEventListener("selectstart", (event) => event.preventDefault());

window.addEventListener("pageshow", () => {
    window.scrollTo(0, 0);
    scheduleControllerReproject("pageshow");
});

function scheduleControllerReproject(reason) {
    if (viewportRenderTimer) {
        window.clearTimeout(viewportRenderTimer);
    }

    viewportRenderTimer = window.setTimeout(async () => {
        viewportRenderTimer = null;
        await waitForStableViewportFrame();
        applyProfileToController();
        console.log(`[CONTROLLER BOOT] layout reprojected reason=${reason}`);
    }, 80);
}

window.addEventListener("resize", () => {
    scheduleControllerReproject("resize");
});

window.visualViewport?.addEventListener("resize", () => {
    scheduleControllerReproject("visualViewport.resize");
});

function pointInsideElement(x, y, element) {
    const rect = element.getBoundingClientRect();

    return x >= rect.left && x <= rect.right && y >= rect.top && y <= rect.bottom;
}

function getButtonAt(x, y) {
    const element = document.elementFromPoint(x, y);
    const directButton = element?.closest?.("[data-button]");

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

    joystick.style.transform = `translate(calc(-50% + ${x}px), calc(-50% + ${y}px))`;

    console.log("[ONA Controller] INPUT SENT joystick", normalizedX.toFixed(2), normalizedY.toFixed(2));

    sendControllerMessage({
        type: "input",
        control: "JOYSTICK",
        x: normalizedX,
        y: normalizedY
    });
}

function resetJoystick(sendNeutral = true) {
    joystickPointerId = null;
    joystick.style.transform = "translate(-50%, -50%)";

    if (sendNeutral) {
        console.log("[ONA Controller] INPUT SENT joystick 0.00 0.00");
        sendControllerMessage({
            type: "input",
            control: "JOYSTICK",
            x: 0,
            y: 0
        });
    }
}

function neutralizeLocalInputState(sendReleasePackets = false) {
    const releasedButtons = new Set();
    const joystickWasActive = joystickPointerId !== null;

    for (const [pointerId, state] of [...activePointers.entries()]) {
        if (state?.type === "button") {
            const name = state.element?.dataset?.button;
            state.element?.classList?.remove("virtual-pressed");
            if (name) {
                releasedButtons.add(name);
            }
        }

        releasePointer(state.element || controller, pointerId);
        activePointers.delete(pointerId);
    }

    activeButtonCounts.clear();
    resetJoystick(false);

    if (sendReleasePackets && joystickWasActive) {
        sendControllerMessage({ type: "input", control: "JOYSTICK", x: 0, y: 0 });
    }

    if (sendReleasePackets) {
        for (const button of releasedButtons) {
            sendControllerMessage({ type: "input", button, state: "up" });
        }
    }

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
        element?.setPointerCapture?.(pointerId);
    }
    catch (error) {
        console.warn("[ONA Controller] setPointerCapture failed:", pointerId, error);
    }
}

function releasePointer(element, pointerId) {
    try {
        if (element?.hasPointerCapture?.(pointerId)) {
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

    if (joystickPointerId === null && pointInsideElement(x, y, zone)) {
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
        cancelled ? "[ONA Controller] POINTER CANCEL" : "[ONA Controller] POINTER UP",
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

        if (!controllerReady) {
            return;
        }

        if (controllerUiMode !== UI_MODES.NORMAL) {
            startEditorDrag(event);
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

        if (!controllerReady) {
            return;
        }

        if (controllerUiMode !== UI_MODES.NORMAL) {
            moveEditorDrag(event);
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

        if (!controllerReady) {
            return;
        }

        if (controllerUiMode !== UI_MODES.NORMAL) {
            endEditorDrag(event);
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

        if (!controllerReady) {
            return;
        }

        if (controllerUiMode !== UI_MODES.NORMAL) {
            endEditorDrag(event);
            return;
        }

        releaseActivePointer(event, true);
    },
    { passive: false }
);

window.addEventListener("blur", () => {
    neutralizeLocalInputState(true);
});

bootstrapController().then(() => {
    diagnoseEditElement("after-render");
});

console.log("ONA POINTER MULTITOUCH READY");
console.log("Joystick + buttons independent by pointerId");

function describeElement(element) {
    if (!element) {
        return "none";
    }

    if (element.id) {
        return `#${element.id}`;
    }

    if (typeof element.className === "string" && element.className.trim()) {
        return `.${element.className.trim().replace(/\s+/g, ".")}`;
    }

    return element.tagName || "unknown";
}

function elementDiagnostics(element) {
    if (!element) {
        return {
            label: "none",
            rect: null,
            pointerEvents: "none",
            zIndex: "auto"
        };
    }

    const style = window.getComputedStyle(element);

    return {
        label: describeElement(element),
        tag: element.tagName || "unknown",
        id: element.id || "",
        className: typeof element.className === "string" ? element.className : "",
        rect: element.getBoundingClientRect(),
        pointerEvents: style.pointerEvents,
        zIndex: style.zIndex,
        position: style.position
    };
}

function diagnoseEditElement(reason, event = null) {
    window.requestAnimationFrame?.(() => {
        const element = document.getElementById("customize-toggle");
        console.log(`[CONTROLLER UI] build=${CONTROLLER_UI_BUILD}`);
        console.log(`[CONTROLLER UI] edit element found=${Boolean(element)}`);

        if (!element) {
            return;
        }

        const rect = element.getBoundingClientRect();
        const centerX = rect.left + rect.width / 2;
        const centerY = rect.top + rect.height / 2;
        const topElement = document.elementFromPoint(centerX, centerY);
        const style = window.getComputedStyle(element);
        const eventTopElement =
            event?.clientX !== undefined && event?.clientY !== undefined
                ? document.elementFromPoint(event.clientX, event.clientY)
                : null;
        const topDiagnostics =
            elementDiagnostics(eventTopElement || topElement);

        lastEditDiagnostics = {
            reason,
            rect,
            topElement: topDiagnostics.label,
            topRect: topDiagnostics.rect,
            topPointerEvents: topDiagnostics.pointerEvents,
            topZIndex: topDiagnostics.zIndex,
            topPosition: topDiagnostics.position,
            pointerEvents: style.pointerEvents,
            zIndex: style.zIndex,
            disabled: element.disabled
        };

        console.log(
            `[CONTROLLER UI] edit rect=${Math.round(rect.left)},${Math.round(rect.top)},${Math.round(rect.width)},${Math.round(rect.height)} reason=${reason}`
        );
        console.log(
            `[CONTROLLER UI] edit pointerEvents=${style.pointerEvents} zIndex=${style.zIndex} disabled=${element.disabled}`
        );
        console.log(
            `[CONTROLLER UI] edit elementFromPoint=${topDiagnostics.label} rect=${rectSummary(topDiagnostics.rect)} zIndex=${topDiagnostics.zIndex} pointerEvents=${topDiagnostics.pointerEvents} position=${topDiagnostics.position}`
        );
        renderDiagnosticsPanel();
    });
}

document.addEventListener(
    "pointerdown",
    (event) => {
        const element = document.getElementById("customize-toggle");

        if (!element) {
            return;
        }

        const rect = element.getBoundingClientRect();

        if (
            event.clientX < rect.left ||
            event.clientX > rect.right ||
            event.clientY < rect.top ||
            event.clientY > rect.bottom
        ) {
            return;
        }

        const topElement = document.elementFromPoint(event.clientX, event.clientY);

        if (topElement?.closest?.("#customize-toggle")) {
            return;
        }

        const topDiagnostics =
            elementDiagnostics(topElement);

        lastEditDiagnostics = {
            reason: "blocked-pointerdown",
            rect,
            topElement: topDiagnostics.label,
            topRect: topDiagnostics.rect,
            topPointerEvents: topDiagnostics.pointerEvents,
            topZIndex: topDiagnostics.zIndex,
            topPosition: topDiagnostics.position,
            pointerEvents: window.getComputedStyle(element).pointerEvents,
            zIndex: window.getComputedStyle(element).zIndex,
            disabled: element.disabled
        };
        console.warn(
            `[CONTROLLER UI] EDIT blocked by ${topDiagnostics.label} rect=${rectSummary(topDiagnostics.rect)} zIndex=${topDiagnostics.zIndex} pointerEvents=${topDiagnostics.pointerEvents}`
        );
        renderDiagnosticsPanel();
    },
    true
);
