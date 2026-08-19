// =========================================================
// ONA GAMING STUDIO
// ONA SHELL
// =========================================================

console.log("ONA Shell initializing...");

const ONA_RUNTIME_BUILD =
    "PRIMARY-HWND-START-TOGGLE-01";

console.log("[ONA BUILD]", ONA_RUNTIME_BUILD);

const tauri = window.__TAURI__;

if (!tauri?.core?.invoke) {
    throw new Error("Tauri global API is unavailable.");
}

const { invoke } = tauri.core;

// =========================================================
// ELEMENTOS
// =========================================================

const bootScreen =
    document.getElementById("boot-screen");

const intro =
    document.getElementById("ona-intro");

const shell =
    document.getElementById("ona-shell");

const controllerScreen =
    document.getElementById("controller-screen");

const profileScreen =
    document.getElementById("profile-screen");

const mainMenu =
    document.getElementById("main-menu");

const controllersScreen =
    document.getElementById("controllers-screen");

const gameLibraryScreen =
    document.getElementById("game-library-screen");

const storeScreen =
    document.getElementById("store-screen");

const settingsScreen =
    document.getElementById("settings-screen");

const controllerLabScreen =
    document.getElementById("controller-lab-screen");

const systemMenu =
    document.getElementById("system-menu");

const controllerStatus =
    document.getElementById("controller-status");

const connectionText =
    document.getElementById("connection-text");

const connectionDot =
    document.getElementById("connection-dot");

const playerCount =
    document.getElementById("player-count");

const profileGrid =
    document.getElementById("profile-grid");

const profileControllerText =
    document.getElementById("profile-controller-text");

const profilePlayerNumber =
    document.getElementById("profile-player-number");

const currentProfileAvatar =
    document.getElementById("current-profile-avatar");

const currentProfileName =
    document.getElementById("current-profile-name");

const homePlayerAvatar =
    document.getElementById("home-player-avatar");

const homePlayerName =
    document.getElementById("home-player-name");

const homeContinueCard =
    document.getElementById("home-continue-card");

const homeContinueTitle =
    document.getElementById("home-continue-title");

const homeContinueMeta =
    document.getElementById("home-continue-meta");

const homeContinueAction =
    document.getElementById("home-continue-action");

const homeRecentRow =
    document.getElementById("home-recent-row");

const libraryPlayerAvatar =
    document.getElementById("library-player-avatar");

const libraryPlayerName =
    document.getElementById("library-player-name");

const storePlayerAvatar =
    document.getElementById("store-player-avatar");

const storePlayerName =
    document.getElementById("store-player-name");

const settingsCategories =
    document.getElementById("settings-categories");

const settingsPanel =
    document.getElementById("settings-panel");

const resumeButton =
    document.getElementById("resume-button");

const homeButton =
    document.getElementById("home-button");

const quickControllersButton =
    document.getElementById("quick-controllers-button");

const minimizeButton =
    document.getElementById("minimize-button");

const restartButton =
    document.getElementById("restart-button");

const exitButton =
    document.getElementById("exit-button");

const importGameButton =
    document.getElementById("import-game-button");

const libraryStatus =
    document.getElementById("library-status");

const gameGrid =
    document.getElementById("game-grid");

const importGameOverlay =
    document.getElementById("import-game-overlay");

const importSourceCount =
    document.getElementById("import-source-count");

const importStatus =
    document.getElementById("import-status");

const importResults =
    document.getElementById("import-results");

const importDetails =
    document.getElementById("import-details");

const importActions =
    document.getElementById("import-actions");

const gameLifecycleOverlay =
    document.getElementById("game-lifecycle-overlay");

const gameLifecycleTitle =
    document.getElementById("game-lifecycle-title");

const gameLifecycleName =
    document.getElementById("game-lifecycle-name");

const gameLifecycleStatus =
    document.getElementById("game-lifecycle-status");

const gameLifecycleDetails =
    document.getElementById("game-lifecycle-details");

const gameOptionsOverlay =
    document.getElementById("game-options-overlay");

const gameOptionsTitle =
    document.getElementById("game-options-title");

const gameOptionsStatus =
    document.getElementById("game-options-status");

const gameOptionsDetails =
    document.getElementById("game-options-details");

const gameOptionsActions =
    document.getElementById("game-options-actions");

const controllersStatus =
    document.getElementById("controllers-status");

const controllersPlayerName =
    document.getElementById("controllers-player-name");

const calibrateControllerButton =
    document.getElementById("calibrate-controller-button");

const changeProfileButton =
    document.getElementById("change-profile-button");

const addControllerButton =
    document.getElementById("add-controller-button");

const labConnectionDot =
    document.getElementById("lab-connection-dot");

const labConnectionText =
    document.getElementById("lab-connection-text");

const labStickDot =
    document.getElementById("lab-stick-dot");

const labStickX =
    document.getElementById("lab-stick-x");

const labStickY =
    document.getElementById("lab-stick-y");

const labRawStickX =
    document.getElementById("lab-raw-stick-x");

const labRawStickY =
    document.getElementById("lab-raw-stick-y");

const labButtons =
    document.getElementById("lab-buttons");

const labDeadzone =
    document.getElementById("lab-deadzone");

const labDeadzoneValue =
    document.getElementById("lab-deadzone-value");

const labSensitivity =
    document.getElementById("lab-sensitivity");

const labSensitivityValue =
    document.getElementById("lab-sensitivity-value");

const labCenterButton =
    document.getElementById("lab-center-button");

const labSaveProfileButton =
    document.getElementById("lab-save-profile-button");

const labProfileName =
    document.getElementById("lab-profile-name");

const labLastInput =
    document.getElementById("lab-last-input");

const labBridgeStatus =
    document.getElementById("lab-bridge-status");

const labSaveStatus =
    document.getElementById("lab-save-status");


// =========================================================
// ONA STATE
// =========================================================

const ONA_STATE = {

    BOOT:
        "boot",

    WAITING_CONTROLLER:
        "waiting-controller",

    PROFILE_SELECT:
        "profile-select",

    MAIN_MENU:
        "main-menu",

    HOME:
        "main-menu",

    GAME_LIBRARY:
        "game-library",

    STORE:
        "store",

    SETTINGS:
        "settings",

    CONTROLLERS:
        "controllers",

    CONTROLLER_LAB:
        "controller-lab",

    QUICK_MENU:
        "quick-menu",

    GAME_RUNNING:
        "game-running"

};


let currentState =
    ONA_STATE.BOOT;

let stateBeforeSystemMenu =
    ONA_STATE.WAITING_CONTROLLER;

let uiNavigationLocked =
    false;

const UI_NAVIGATION_THRESHOLD =
    0.35;

const UI_NAVIGATION_NEUTRAL_THRESHOLD =
    0.2;

const ConsolePresentationState = {
    IDLE:
        "Idle",
    PREPARING:
        "Preparing",
    LAUNCHING:
        "Launching",
    WAITING_FOR_READY:
        "WaitingForReady",
    RUNNING:
        "Running",
    RETURNING:
        "Returning",
    FAILED:
        "Failed"
};

const GameSessionState = {
    IDLE:
        "Idle",
    NONE:
        "None",
    LAUNCHING:
        "Launching",
    RUNNING:
        "Running",
    RUNNING_FOREGROUND:
        "RunningForeground",
    BACKGROUND:
        "Background",
    SYSTEM_OVERLAY:
        "SystemOverlay",
    MINIMIZED:
        "Minimized",
    STOPPING:
        "Stopping",
    EXITED:
        "Exited",
    RETURNING:
        "Returning",
    FAILED:
        "Failed"
};

const PresentationOwner = {
    ONA_SHELL:
        "ONA_SHELL",
    ONA_TRANSITION_GUARD:
        "ONA_TRANSITION_GUARD",
    GAME:
        "GAME",
    ONA_SYSTEM_OVERLAY:
        "ONA_SYSTEM_OVERLAY",
    ONA_MINIMIZED:
        "ONA_MINIMIZED"
};

let consolePresentationState =
    ConsolePresentationState.IDLE;

let gameSessionState =
    GameSessionState.IDLE;

let presentationOwner =
    PresentationOwner.ONA_SHELL;

let gameInputForwardingEnabled =
    false;

let presentationInvariantWarningsSuppressed =
    false;


// =========================================================
// PLAYER STATE
// =========================================================

let playerCountValue = 0;

let selectedPlayer = 1;

let currentProfile = {

    player:
        1,

    id:
        "guest",

    name:
        "PLAYER 1",

    type:
        "GUEST"

};

let installedGames = [];

let selectedGameIndex = 0;

let importBrowserOpen =
    false;

let importBrowserState =
    "idle";

let scannedInstallPackages = [];

let invalidInstallPackages = [];

let selectedInstallPackageIndex = 0;

let installedPackageProfile =
    null;

let pendingInstallPackage =
    null;

let gameOptionsOpen =
    false;

let gameOptionsMode =
    "menu";

let selectedGameOptionIndex =
    0;

let selectedOptionsGame =
    null;

let runningGamePollTimer =
    null;

let runningGameId =
    null;

let activeGameSession =
    null;

let preMinimizePresentationState =
    null;

let restoringConsoleExperience =
    false;

let presentationTransitionId =
    0;

let activePresentationTransition =
    null;

let activeGameExitTrace =
    null;

let activeExitTransaction =
    null;

let activeSessionGeneration =
    0;

const SystemMenuRequestState = Object.freeze({
    CLOSED:
        "CLOSED",
    OPENING:
        "OPENING",
    OPEN:
        "OPEN",
    CLOSING:
        "CLOSING"
});

const RestoreTarget = Object.freeze({
    GAME:
        "GAME",
    SYSTEM_OVERLAY:
        "SYSTEM_OVERLAY",
    ONA_SHELL:
        "ONA_SHELL"
});

const VALID_SYSTEM_MENU_SOURCES = new Set([
    "HOLD_START",
    "START_OVERLAY",
    "ESC_DOM",
    "ESC_NATIVE"
]);

let systemMenuRequestState =
    SystemMenuRequestState.CLOSED;

let nativeShellMinimizeRestore =
    false;

const presentationWatchdogWarnings =
    new Set();

let pendingPlayAfterCloseGame =
    null;

let uiOperationState =
    "idle";

let onaExitConfirmOpen =
    false;

const START_HOLD_THRESHOLD_MS =
    1000;

let startHoldTimer =
    null;

let startHoldStartedAt =
    0;

const StartArbitrationState = Object.freeze({
    IDLE:
        "IDLE",
    PENDING:
        "PENDING",
    SHORT_DISPATCHED:
        "SHORT_DISPATCHED",
    HOLD_CONSUMED:
        "HOLD_CONSUMED"
});

let startArbitrationState =
    StartArbitrationState.IDLE;

let startArbitrationSessionGeneration =
    null;

let lastEscapeSystemCommandAt =
    0;

let onaShutdownInProgress =
    false;

let addingController =
    false;

const ONA_DEMO_DIAGNOSTICS =
    true;

let shellDiagnosticsBadge =
    null;

const ONA_CONTROLLER_BUTTONS = [
    "A",
    "B",
    "X",
    "Y",
    "L1",
    "L2",
    "R1",
    "R2",
    "SELECT",
    "START"
];

function uiBusy() {

    return uiOperationState !== "idle";

}


function updateShellDiagnostics() {

    if (!ONA_DEMO_DIAGNOSTICS) {
        return;
    }

    if (!shellDiagnosticsBadge) {
        shellDiagnosticsBadge =
            document.createElement("div");
        shellDiagnosticsBadge.id =
            "ona-demo-diagnostics";
        shellDiagnosticsBadge.style.position =
            "fixed";
        shellDiagnosticsBadge.style.left =
            "10px";
        shellDiagnosticsBadge.style.bottom =
            "10px";
        shellDiagnosticsBadge.style.zIndex =
            "20000";
        shellDiagnosticsBadge.style.padding =
            "6px 8px";
        shellDiagnosticsBadge.style.border =
            "1px solid rgba(120, 230, 255, 0.26)";
        shellDiagnosticsBadge.style.borderRadius =
            "4px";
        shellDiagnosticsBadge.style.background =
            "rgba(0, 0, 0, 0.56)";
        shellDiagnosticsBadge.style.color =
            "rgba(226, 246, 255, 0.82)";
        shellDiagnosticsBadge.style.font =
            "10px Arial, sans-serif";
        shellDiagnosticsBadge.style.letterSpacing =
            "0.4px";
        shellDiagnosticsBadge.style.pointerEvents =
            "none";
        document.body.appendChild(shellDiagnosticsBadge);
    }

    shellDiagnosticsBadge.textContent =
        "BETA";

}


updateShellDiagnostics();


function setSystemMenuRequestState(nextState, reason = "") {

    if (systemMenuRequestState === nextState) {
        return;
    }

    console.log(
        `[ONA SYSTEM] state ${systemMenuRequestState} -> ${nextState}${reason ? ` reason=${reason}` : ""}`
    );
    systemMenuRequestState =
        nextState;

}


function normalizeSystemMenuState(reason = "normalize") {

    const menuVisible =
        Boolean(systemMenu?.classList.contains("visible"));

    if (
        systemMenuRequestState === SystemMenuRequestState.OPEN &&
        !menuVisible &&
        presentationOwner !== PresentationOwner.ONA_SYSTEM_OVERLAY
    ) {
        setSystemMenuRequestState(
            SystemMenuRequestState.CLOSED,
            `${reason}_open_without_menu`
        );
    }

}


function stableRestoreTargetFromSnapshot(snapshot) {

    if (snapshot?.activeGameSession?.pid) {
        return RestoreTarget.GAME;
    }

    const menuVisiblyOpen =
        Boolean(snapshot?.systemMenuOpen || snapshot?.gameOverlay);
    const runningGameForeground =
        snapshot?.currentState === ONA_STATE.GAME_RUNNING ||
        snapshot?.gameSessionState === GameSessionState.RUNNING ||
        snapshot?.gameSessionState === GameSessionState.RUNNING_FOREGROUND ||
        snapshot?.presentationOwner === PresentationOwner.GAME;

    if (runningGameForeground && !menuVisiblyOpen) {
        return RestoreTarget.GAME;
    }

    if (
        snapshot?.presentationOwner === PresentationOwner.ONA_SYSTEM_OVERLAY ||
        snapshot?.systemMenuOpen
    ) {
        return RestoreTarget.SYSTEM_OVERLAY;
    }

    return RestoreTarget.ONA_SHELL;

}


function forceSystemMenuClosed(reason) {

    systemMenu
        ?.classList
        .remove("visible");
    document.body
        .classList
        .remove("system-menu-open");
    document.body
        .classList
        .remove("mouse-enabled");
    document.body
        .classList
        .remove("game-system-overlay");
    setSystemMenuRequestState(
        SystemMenuRequestState.CLOSED,
        reason
    );
    console.log(
        `[ONA SYSTEM] force CLOSED reason=${reason}`
    );

}


function resetStartArbitration(reason) {

    if (startHoldTimer) {
        clearTimeout(startHoldTimer);
        startHoldTimer =
            null;
    }

    startHoldStartedAt =
        0;
    startArbitrationState =
        StartArbitrationState.IDLE;
    startArbitrationSessionGeneration =
        null;

    console.log(
        `[ONA START] reset reason=${reason}`
    );
    if (String(reason).toUpperCase().includes("MINIMIZE")) {
        console.log(
            "[ONA Minimize] START arbiter reset IDLE"
        );
    }

}


async function logPostRestoreState(context) {

    const escStatus =
        await invoke("escape_system_shortcut_status").catch(() => null);
    const shellStatus =
        await invoke("shell_window_presentation_status").catch(() => null);
    const gameStatus =
        await invoke("running_game_status").catch(() => null);

    console.log(
        `[ONA POST-RESTORE STATE] context=${context} owner=${presentationOwner} systemMenuState=${systemMenuRequestState} routing=${gameInputForwardingEnabled} startArbiterState=${startArbitrationState} startPhysicalDown=${startArbitrationState === StartArbitrationState.PENDING || startArbitrationState === StartArbitrationState.HOLD_CONSUMED} startHoldTimerActive=${Boolean(startHoldTimer)} escRegistered=${Boolean(escStatus?.registered)} escGeneration=${escStatus?.generation ?? "unknown"} quickMenuVisible=${Boolean(systemMenu?.classList.contains("visible"))} shellVisible=${Boolean(shellStatus?.visible)} gamePid=${activeGameSession?.pid || gameStatus?.pid || "none"} gameForeground=${presentationOwner === PresentationOwner.GAME && nativeGameSessionActive()} restoreInProgress=${restoringConsoleExperience} systemUiEpoch=${presentationTransitionId}`
    );

}


function startExitTransaction(kind, session) {

    const generation =
        session?.sessionGeneration || activeGameSession?.sessionGeneration || null;

    if (
        activeExitTransaction &&
        activeExitTransaction.sessionGeneration === generation &&
        activeExitTransaction.state !== "COMPLETED"
    ) {
        return activeExitTransaction;
    }

    activeExitTransaction = {
        sessionGeneration:
            generation,
        gameId:
            session?.gameId || activeGameSession?.gameId || null,
        pid:
            session?.pid || activeGameSession?.pid || null,
        state:
            kind,
        trace:
            activeGameExitTrace || createPresentationTimingTrace(kind)
    };

    console.log(
        `[ONA Exit] ${kind === "COOPERATIVE_EXIT_IN_PROGRESS" ? "cooperative" : "crash"} transaction started session=${generation}`
    );

    return activeExitTransaction;

}


function completeExitTransaction(reason) {

    if (!activeExitTransaction) {
        return;
    }

    activeExitTransaction.state =
        "COMPLETED";
    console.log(
        `[ONA Exit] transaction completed reason=${reason} session=${activeExitTransaction.sessionGeneration}`
    );
    activeExitTransaction =
        null;
    activeGameExitTrace =
        null;

}


function nativeGameSessionActive() {

    return (
        Boolean(activeGameSession?.pid) &&
        [
            GameSessionState.RUNNING,
            GameSessionState.RUNNING_FOREGROUND,
            GameSessionState.SYSTEM_OVERLAY
        ].includes(activeGameSession?.state)
    );

}


function setUiOperation(state) {

    uiOperationState =
        state || "idle";

}


function setConsolePresentationState(state) {

    consolePresentationState =
        state || ConsolePresentationState.IDLE;

    document.body.dataset.consolePresentation =
        consolePresentationState;

    gameLifecycleOverlay?.setAttribute(
        "data-presentation-state",
        consolePresentationState
    );

}


function setGameSessionState(state) {

    if (gameSessionState === state) {
        return;
    }

    console.log(
        `[ONA Session] state ${gameSessionState} -> ${state}`
    );

    gameSessionState =
        state;

}


function setPresentationOwner(owner) {

    if (presentationOwner === owner) {
        return;
    }

    console.log(
        `[ONA Presentation] owner ${presentationOwner} -> ${owner}`
    );

    presentationOwner =
        owner;

    syncEscapeSystemShortcut();
    assertPresentationInvariants();

}


function syncEscapeSystemShortcut() {

    if (presentationOwner === PresentationOwner.GAME) {
        invoke("enable_escape_system_shortcut").catch(
            (error) =>
                console.warn("[ONA Keyboard] ESC native shortcut enable failed:", error)
        );
        return;
    }

    invoke("disable_escape_system_shortcut").catch(
        (error) =>
            console.warn("[ONA Keyboard] ESC native shortcut disable failed:", error)
    );

}


function setActiveGameSession(session) {

    activeGameSession =
        session;

    runningGameId =
        session?.gameId || null;

    console.log(
        "[ONA Session] active",
        activeGameSession || "None"
    );

    renderActiveGameSessionUi();

}


function updateActiveGameSession(fields) {

    if (!activeGameSession) {
        return;
    }

    setActiveGameSession({
        ...activeGameSession,
        ...fields
    });

}


function activeSessionGame() {

    if (!activeGameSession?.gameId) {
        return null;
    }

    return installedGames.find(
        (game) =>
            game.id === activeGameSession.gameId
    ) || {
        id:
            activeGameSession.gameId,
        name:
            activeGameSession.gameName || "GAME"
    };

}


function activeSessionIsBackground() {

    return activeGameSession?.state ===
        GameSessionState.BACKGROUND;

}


function activeSessionIsForeground() {

    return activeGameSession?.state ===
        GameSessionState.RUNNING_FOREGROUND ||
        activeGameSession?.state ===
        GameSessionState.RUNNING;

}


function selectedGameHasBackgroundSession(game) {

    return Boolean(
        game?.id &&
        activeSessionIsBackground() &&
        activeGameSession?.gameId === game.id
    );

}


async function setGameInputRouting(enabled) {

    try {
        if (!enabled && gameInputForwardingEnabled) {
            await invoke("neutralize_game_input").catch(
                (error) =>
                    console.warn("[ONA Input Routing] neutralize failed:", error)
            );
        }
        await invoke(
            "set_game_input_forwarding",
            {
                enabled
            }
        );
        gameInputForwardingEnabled =
            Boolean(enabled);
        assertPresentationInvariants();
    }
    catch (error) {
        console.error(
            "[ONA Input Routing] Could not update game forwarding:",
            error
        );
    }

}


function nextPresentationTransition(reason) {

    return beginPresentationTransition(reason);

}


const PresentationTransitionPriority = {
    USER_NAVIGATION:
        10,
    GAME_EXIT:
        60,
    CRASH_RECOVERY:
        80,
    SHUTDOWN:
        100
};


function transitionPriorityForReason(reason) {

    const normalized =
        String(reason || "").toUpperCase();

    if (normalized.includes("SHUTDOWN")) {
        return PresentationTransitionPriority.SHUTDOWN;
    }

    if (normalized.includes("PROCESS_EXIT") || normalized.includes("CRASH")) {
        return PresentationTransitionPriority.CRASH_RECOVERY;
    }

    if (normalized.includes("GAME_EXIT")) {
        return PresentationTransitionPriority.GAME_EXIT;
    }

    return PresentationTransitionPriority.USER_NAVIGATION;

}


function beginPresentationTransition(reason) {

    const priority =
        transitionPriorityForReason(reason);

    if (
        activePresentationTransition &&
        activePresentationTransition.priority > priority
    ) {
        console.log(
            `[ONA Presentation] transition rejected reason=${reason} active=${activePresentationTransition.reason}`
        );
        return null;
    }

    presentationTransitionId += 1;
    activePresentationTransition = {
        id:
            presentationTransitionId,
        reason,
        priority
    };
    console.log(
        `[ONA Presentation] transition=${presentationTransitionId} reason=${reason}`
    );
    return presentationTransitionId;

}


function presentationTransitionCurrent(id) {

    return Boolean(id) && id === presentationTransitionId;

}


function endPresentationTransition(id) {

    if (presentationTransitionCurrent(id)) {
        activePresentationTransition =
            null;
    }

}


function createPresentationTimingTrace(label) {

    const startedAt =
        performance.now();

    return {
        mark(event, details = "") {
            const elapsed =
                Math.round(performance.now() - startedAt);
            const suffix =
                details
                    ? ` ${details}`
                    : "";

            console.log(
                `[ONA EXIT TRACE] ${label} ${event} +${elapsed}ms${suffix}`
            );
        }
    };

}


async function requirePresentationGuard(mode, reason, trace = null) {

    console.log(
        `[ONA Guard] required mode=${mode} reason=${reason}`
    );
    trace?.mark("T1 GUARD_SHOW_REQUESTED");

    await invoke(
        "show_presentation_guard",
        {
            mode
        }
    );

    const status =
        await invoke("presentation_guard_native_status");

    if (status?.visible) {
        trace?.mark("T2 GUARD_VISIBLE");
    }

    if (status?.fullscreen) {
        trace?.mark("T3 GUARD_FULLSCREEN_BOUNDS_CONFIRMED");
    }

    if (status?.alwaysOnTop) {
        trace?.mark("T4 GUARD_TOPMOST_CONFIRMED");
    }

    if (
        !status?.exists ||
        !status.visible ||
        !status.alwaysOnTop ||
        !status.fullscreen
    ) {
        throw new Error(
            `PRESENTATION_GUARD_REQUIRED_FAILED exists=${status?.exists} visible=${status?.visible} topmost=${status?.alwaysOnTop} fullscreen=${status?.fullscreen}`
        );
    }

    console.log(
        `[ONA Guard] required confirmed mode=${mode}`
    );

}


async function recoverPresentationToShell(reason, game = null) {

    console.error(
        "[ONA Presentation] recovery requested",
        reason
    );
    await setGameInputRouting(false);
    await invoke("restore_shell_window_presentation").catch(
        (error) =>
            console.error("[ONA Presentation] Shell recovery failed:", error)
    );
    await invoke("release_presentation_guard").catch(
        (error) =>
            console.error("[ONA Presentation] Guard recovery failed:", error)
    );
    clearGameLifecycleOverlay();
    setUiOperation("idle");
    setConsolePresentationState(
        ConsolePresentationState.IDLE
    );
    setGameSessionState(
        game ? GameSessionState.BACKGROUND : GameSessionState.IDLE
    );
    setPresentationOwner(
        PresentationOwner.ONA_SHELL
    );

}


async function finalizeSafeHandoffToGame(game, pid, sessionState = GameSessionState.RUNNING_FOREGROUND) {

    const transitionId =
        nextPresentationTransition("SAFE_HANDOFF_FINALIZE");

    console.log(
        "[ONA Presentation] finalizing safe handoff"
    );

    clearGameLifecycleOverlay();
    await invoke(
        "prepare_shell_for_game",
        {
            reason:
                "safe_handoff"
        }
    );

    if (!presentationTransitionCurrent(transitionId)) {
        console.warn(
            "[ONA Presentation] stale handoff finalization ignored"
        );
        return false;
    }

    console.log(
        "[ONA Presentation] shell hidden confirmed"
    );

    try {
        await invoke(
            "release_presentation_guard"
        );
    }
    catch (error) {
        await recoverPresentationToShell(
            `GUARD_RELEASE_AFTER_HANDOFF_FAILED: ${error}`,
            game
        );
        return false;
    }

    if (!presentationTransitionCurrent(transitionId)) {
        console.warn(
            "[ONA Presentation] stale guard release ignored"
        );
        return false;
    }

    console.log(
        "[ONA Presentation] guard hidden confirmed"
    );
    console.log(
        "[ONA Presentation] game visible confirmed"
    );

    setUiOperation("running");
    setGameSessionState(
        sessionState
    );
    updateActiveGameSession({
        state:
            sessionState,
        presentationState:
            ConsolePresentationState.RUNNING,
        pid
    });
    currentState =
        ONA_STATE.GAME_RUNNING;
    setConsolePresentationState(
        ConsolePresentationState.RUNNING
    );
    presentationInvariantWarningsSuppressed =
        true;
    await setGameInputRouting(true);
    setPresentationOwner(
        PresentationOwner.GAME
    );
    await invoke("suppress_running_game_taskbar_identity", { pid }).catch(
        (error) =>
            console.warn("[ONA GameWindow] taskbar suppression after handoff failed:", error)
    );
    presentationInvariantWarningsSuppressed =
        false;
    assertPresentationInvariants();
    console.log(
        "[ONA Presentation] owner=GAME"
    );
    endPresentationTransition(transitionId);
    return true;

}


function assertPresentationInvariants() {

    if (presentationInvariantWarningsSuppressed) {
        return;
    }

    const invalid =
        [];

    if (
        presentationOwner === PresentationOwner.GAME &&
        !gameInputForwardingEnabled &&
        startArbitrationState !== StartArbitrationState.HOLD_CONSUMED
    ) {
        invalid.push("GAME owner with forwarding paused");
    }

    if (
        presentationOwner !== PresentationOwner.GAME &&
        gameInputForwardingEnabled
    ) {
        invalid.push(`${presentationOwner} owner with forwarding enabled`);
    }

    if (
        presentationOwner === PresentationOwner.ONA_SHELL &&
        document.body.classList.contains("game-system-overlay")
    ) {
        invalid.push("ONA_SHELL with system overlay class");
    }

    if (
        presentationOwner === PresentationOwner.ONA_MINIMIZED &&
        systemMenu?.classList.contains("visible")
    ) {
        invalid.push("ONA_MINIMIZED with Quick Menu visible");
    }

    if (invalid.length > 0) {
        console.warn(
            "[ONA State] INVALID PRESENTATION COMBINATION",
            invalid.join("; "),
            {
                presentationOwner,
                gameSessionState,
                consolePresentationState,
                gameInputForwardingEnabled
            }
        );
    }

}


async function runPresentationWatchdog() {

    if (
        ![
            PresentationOwner.GAME,
            PresentationOwner.ONA_SHELL,
            PresentationOwner.ONA_SYSTEM_OVERLAY
        ].includes(presentationOwner)
    ) {
        return;
    }

    const guard =
        await invoke("presentation_guard_native_status").catch(
            () =>
                null
        );

    if (!guard) {
        return;
    }

    const violations =
        [];

    if (
        presentationOwner === PresentationOwner.GAME &&
        (guard.visible || guard.alwaysOnTop || !gameInputForwardingEnabled)
    ) {
        violations.push(
            `GAME guardVisible=${guard.visible} guardTopmost=${guard.alwaysOnTop} forwarding=${gameInputForwardingEnabled}`
        );
    }

    if (
        presentationOwner === PresentationOwner.ONA_SHELL &&
        (guard.visible || guard.alwaysOnTop || gameInputForwardingEnabled || document.body.classList.contains("game-system-overlay"))
    ) {
        violations.push(
            `ONA_SHELL guardVisible=${guard.visible} guardTopmost=${guard.alwaysOnTop} forwarding=${gameInputForwardingEnabled}`
        );
    }

    if (
        presentationOwner === PresentationOwner.ONA_SYSTEM_OVERLAY &&
        (guard.visible || guard.alwaysOnTop || gameInputForwardingEnabled)
    ) {
        violations.push(
            `ONA_SYSTEM_OVERLAY guardVisible=${guard.visible} guardTopmost=${guard.alwaysOnTop} forwarding=${gameInputForwardingEnabled}`
        );
    }

    for (const violation of violations) {
        if (!presentationWatchdogWarnings.has(violation)) {
            presentationWatchdogWarnings.add(violation);
            console.warn(
                "[ONA Presentation Watchdog] invariant violation",
                violation
            );
        }
    }

}


function renderActiveGameSessionUi() {

    const game =
        activeSessionGame();

    document.body.classList.toggle(
        "has-background-game",
        activeSessionIsBackground()
    );

    if (homeContinueCard) {
        homeContinueCard.classList.toggle(
            "home-background-session",
            activeSessionIsBackground()
        );
    }

    if (!activeSessionIsBackground() || !game) {
        return;
    }

    if (homeContinueTitle) {
        homeContinueTitle.textContent =
            game.name || activeGameSession.gameName || "GAME";
    }

    if (homeContinueMeta) {
        homeContinueMeta.textContent =
            `RUNNING / PID ${activeGameSession.pid || "UNKNOWN"}`;
    }

    if (homeContinueAction) {
        homeContinueAction.textContent =
            "CONTINUE GAME";
    }

}


function waitForTransition(milliseconds = 320) {

    return new Promise(
        (resolve) =>
            setTimeout(
                resolve,
                milliseconds
            )
    );

}

let controllerProfile = {
    name:
        "Default",

    stick: {
        centerX:
            0,
        centerY:
            0,
        deadzone:
            0.12,
        sensitivity:
            1
    },

    buttonMapping:
        ONA_CONTROLLER_BUTTONS.map(
            (button) => ({
                physical:
                    button,
                onaAction:
                    button
            })
        )
};

let lastRawStick = {
    x:
        0,
    y:
        0
};

let shellSettings = {
    language:
        "English",
    uiAnimations:
        true,
    reducedMotion:
        false,
    visualIntensity:
        "normal",
    uiMuted:
        false,
    uiVolume:
        70
};

let settingsFocusLevel =
    "category";

let settingsData = {
    display:
        null,
    storage:
        null,
    system:
        null,
    connectivity:
        null
};


// =========================================================
// SCREEN MANAGEMENT
// =========================================================

function hideAllScreens() {

    controllerScreen
        ?.classList
        .remove("active");

    profileScreen
        ?.classList
        .remove("active");

    mainMenu
        ?.classList
        .remove("active");

    controllersScreen
        ?.classList
        .remove("active");

    gameLibraryScreen
        ?.classList
        .remove("active");

    storeScreen
        ?.classList
        .remove("active");

    settingsScreen
        ?.classList
        .remove("active");

    controllerLabScreen
        ?.classList
        .remove("active");

}


// =========================================================
// SHOW CONTROLLER SCREEN
// =========================================================

function showControllerScreen() {

    hideAllScreens();

    controllerScreen
        ?.classList
        .add("active");

    updateControllerScreenMode();

    generateQR();

}


function updateControllerScreenMode() {

    if (controllerStatus) {
        controllerStatus.textContent =
            addingController
                ? "Scan to add another ONA Controller."
                : "Scan the QR code with your phone to begin.";
    }

    if (connectionText) {
        connectionText.textContent =
            addingController
                ? `PLAYER ${playerCountValue + 1} PAIRING`
                : "WAITING FOR CONTROLLER";
    }

    connectionDot
        ?.classList
        .toggle("connected", playerCountValue > 0);

}


// =========================================================
// GENERATE QR
// =========================================================

async function generateQR() {
    const qrContainer = document.getElementById('qr-code');
    if (!qrContainer) {
        console.warn('QR container not found');
        return;
    }

    try {
        const { url, svg } = await invoke('generate_qr_session');
        console.log('QR URL generada:', url);

        qrContainer.innerHTML = '';

        qrContainer.innerHTML = svg;

    } catch (error) {
        console.error('Error generando QR:', error);
        qrContainer.innerHTML = '<p style="color:red;font-size:12px;">Error al generar QR</p>';
    }
}


// =========================================================
// SHOW PROFILE SCREEN
// =========================================================

function showProfileScreen() {

    hideAllScreens();

    profileScreen
        ?.classList
        .add("active");

    updateProfileControllerStatus();

    updateProfileSelection();

}


// =========================================================
// SHOW MAIN MENU
// =========================================================

function showMainMenu() {

    hideAllScreens();

    mainMenu
        ?.classList
        .add("active");

    updateCurrentProfile();
    updateHomePlayer();
    loadHomeCatalog();

}


async function loadHomeCatalog() {

    try {

        const catalog =
            await invoke("list_installed_games");

        installedGames =
            Array.isArray(catalog?.games)
                ? catalog.games
                : [];

        if (
            selectedGameIndex > installedGames.length
        ) {
            selectedGameIndex = 0;
        }

        renderHome();

    }

    catch (error) {

        console.error(
            "[ONA Home] Unable to load games:",
            error
        );

        installedGames = [];

        renderHome();

    }

}


function updateHomePlayer() {

    if (homePlayerAvatar) {
        homePlayerAvatar.textContent =
            `P${currentProfile.player}`;
    }

    if (homePlayerName) {
        homePlayerName.textContent =
            currentProfile.name;
    }

}


function renderHome() {

    const activeGame =
        activeSessionIsBackground()
            ? activeSessionGame()
            :
        installedGames[selectedGameIndex] ||
        installedGames[0];

    mainMenu
        ?.classList
        .toggle(
            "home-has-games",
            Boolean(activeGame)
        );

    if (homeContinueCard) {
        homeContinueCard.classList.toggle(
            "home-empty-library",
            !activeGame
        );
    }

    if (homeContinueTitle) {
        homeContinueTitle.textContent =
            activeGame?.name ||
            "YOUR LIBRARY IS READY";
    }

    if (homeContinueMeta) {
        homeContinueMeta.textContent =
            activeSessionIsBackground()
                ? `RUNNING / PID ${activeGameSession.pid || "UNKNOWN"}`
                : activeGame
                ? "Ready to play"
                : "Add your first game to start playing.";
    }

    if (homeContinueAction) {
        homeContinueAction.textContent =
            activeSessionIsBackground()
                ? "CONTINUE GAME"
                : activeGame
                ? "PLAY NOW"
                : "GAME LIBRARY";
    }

    renderHomeRecentGames();
    renderActiveGameSessionUi();

}


function renderHomeRecentGames() {

    if (!homeRecentRow) {
        return;
    }

    const selectedAction =
        mainMenu
            ?.querySelector(".menu-item.selected")
            ?.dataset
            ?.action;

    homeRecentRow.innerHTML = "";

    installedGames.forEach(
        (game, index) => {

            const card =
                document.createElement("button");

            card.className =
                "menu-item home-recent-card";

            card.dataset.action =
                "recent";

            card.dataset.gameIndex =
                String(index);

            card.classList.toggle(
                "selected",
                index === selectedGameIndex &&
                selectedAction === "recent"
            );

            const iconMarkup =
                game.icon
                    ? `<img class="home-game-art" src="${escapeAttribute(game.icon)}" alt="">`
                    : `<span class="home-game-art home-game-art-placeholder">${escapeHtml(game.name?.slice(0, 2) || "ON")}</span>`;

            card.innerHTML =
                `${iconMarkup}
                <span class="home-recent-name">${escapeHtml(game.name || "Untitled Game")}</span>`;

            card.addEventListener(
                "click",
                () => {
                    selectedGameIndex = index;
                    mainMenu
                        ?.querySelectorAll(".menu-item")
                        .forEach(
                            (item) =>
                                item.classList.remove("selected")
                        );
                    card.classList.add("selected");
                    renderHome();
                }
            );

            homeRecentRow.appendChild(card);

        }
    );

}


// =========================================================
// SHOW GAME LIBRARY
// =========================================================

function showGameLibrary() {

    hideAllScreens();

    gameLibraryScreen
        ?.classList
        .add("active");

    updateLibraryPlayer();
    loadGameLibrary();

}


function updateLibraryPlayer() {

    if (libraryPlayerAvatar) {
        libraryPlayerAvatar.textContent =
            `P${currentProfile.player}`;
    }

    if (libraryPlayerName) {
        libraryPlayerName.textContent =
            currentProfile.name;
    }

}


// =========================================================
// SHOW STORE
// =========================================================

function showStore() {

    hideAllScreens();

    storeScreen
        ?.classList
        .add("active");

    updateStorePlayer();

    const selected =
        storeScreen
            ?.querySelector(".store-item.selected");

    if (!selected) {
        storeScreen
            ?.querySelector(".store-item")
            ?.classList
            .add("selected");
    }

}


function updateStorePlayer() {

    if (storePlayerAvatar) {
        storePlayerAvatar.textContent =
            `P${currentProfile.player}`;
    }

    if (storePlayerName) {
        storePlayerName.textContent =
            currentProfile.name;
    }

}


// =========================================================
// SHOW SETTINGS
// =========================================================

async function showSettings() {

    hideAllScreens();

    settingsScreen
        ?.classList
        .add("active");

    settingsFocusLevel =
        "category";

    await loadSettingsData();
    renderSettingsPanel();
    ensureSettingsFocus();

}


async function loadSettingsData() {

    try {
        shellSettings =
            await invoke("load_shell_settings");
    }
    catch (error) {
        console.error("[ONA Settings] Load failed:", error);
    }

    await Promise.all([
        invoke("display_layout")
            .then((layout) => {
                settingsData.display = layout;
            })
            .catch((error) =>
                console.error("[ONA Settings] Display info failed:", error)
            ),
        invoke("storage_information")
            .then((storage) => {
                settingsData.storage = storage;
            })
            .catch((error) =>
                console.error("[ONA Settings] Storage info failed:", error)
            ),
        invoke("system_information")
            .then((system) => {
                settingsData.system = system;
            })
            .catch((error) =>
                console.error("[ONA Settings] System info failed:", error)
            ),
        invoke("local_connectivity_status")
            .then((connectivity) => {
                settingsData.connectivity = connectivity;
            })
            .catch((error) =>
                console.error("[ONA Settings] Connectivity info failed:", error)
            )
    ]);

    applyShellSettings();

}


async function persistShellSettings() {

    applyShellSettings();

    try {
        shellSettings =
            await invoke(
                "save_shell_settings",
                {
                    settings:
                        shellSettings
                }
            );
    }
    catch (error) {
        console.error("[ONA Settings] Save failed:", error);
    }

    renderSettingsPanel();
    ensureSettingsFocus();

}


function applyShellSettings() {

    document.body.classList.toggle(
        "ona-reduced-motion",
        Boolean(shellSettings.reducedMotion) ||
        !shellSettings.uiAnimations
    );

    document.body.dataset.visualIntensity =
        shellSettings.visualIntensity || "normal";

}


function selectedSettingsCategory() {

    return settingsCategories
        ?.querySelector(".settings-category.selected")
        ?.dataset
        ?.settingsCategory || "general";

}


function formatBytes(bytes) {

    const value =
        Number(bytes || 0);

    if (value < 1024 * 1024) {
        return `${Math.round(value / 1024)} KB`;
    }

    if (value < 1024 * 1024 * 1024) {
        return `${(value / 1024 / 1024).toFixed(1)} MB`;
    }

    return `${(value / 1024 / 1024 / 1024).toFixed(2)} GB`;

}


function settingRow(label, value, action = "", disabled = false) {

    return `<button class="settings-option${disabled ? " disabled" : ""}" data-settings-action="${action}" ${disabled ? "disabled" : ""}>
        <span>${escapeHtml(label)}</span>
        <strong>${escapeHtml(value)}</strong>
    </button>`;

}


function renderSettingsPanel() {

    if (!settingsPanel) {
        return;
    }

    const category =
        selectedSettingsCategory();

    const displays =
        settingsData.display?.displays || [];
    const targetDisplay =
        displays.find(
            (display) =>
                display.index === settingsData.display?.targetIndex
        ) || displays[0];

    const panels = {
        general:
            settingRow("LANGUAGE", shellSettings.language || "English", "", true),
        display:
            settingRow("GAMING DISPLAY", targetDisplay ? `${targetDisplay.name || targetDisplay.identifier}` : "UNKNOWN", "", true) +
            settingRow("RESOLUTION", targetDisplay ? `${targetDisplay.width} x ${targetDisplay.height}` : "UNKNOWN", "", true) +
            settingRow("ROLE", targetDisplay?.isPrimary ? "PRIMARY" : "SECONDARY", "", true) +
            settingRow("DETECTED MONITORS", String(displays.length), "", true) +
            settingRow("FULLSCREEN", "ON", "", true),
        audio:
            settingRow("ONA UI VOLUME", `${shellSettings.uiVolume}%`, "ui-volume") +
            settingRow("MUTE UI", shellSettings.uiMuted ? "ON" : "OFF", "ui-muted"),
        personalization:
            settingRow("UI ANIMATIONS", shellSettings.uiAnimations ? "ON" : "OFF", "ui-animations") +
            settingRow("REDUCED MOTION", shellSettings.reducedMotion ? "ON" : "OFF", "reduced-motion") +
            settingRow("VISUAL INTENSITY", (shellSettings.visualIntensity || "normal").toUpperCase(), "visual-intensity"),
        network:
            settingRow("CORE", "ONLINE", "", true) +
            settingRow("CONTROLLER", playerCountValue > 0 ? "CONNECTED" : "WAITING", "", true) +
            settingRow("PLAYERS / CONTROLLERS", `${playerCountValue} / 10`, "", true) +
            settingRow("NETWORK MODE", settingsData.connectivity?.networkMode || "UNKNOWN", "", true) +
            settingRow("LAN", settingsData.connectivity?.lanAvailable ? "AVAILABLE" : "NOT DETECTED", "", true) +
            settingRow("LOCAL IPV4", settingsData.connectivity?.localIpv4 || "UNKNOWN", "", true) +
            settingRow("CONTROLLER URL", settingsData.connectivity?.controllerUrl || "WAITING FOR LOCAL NETWORK", "", true) +
            settingRow("DIRECT LOCAL", settingsData.connectivity?.directLocalActive ? "ACTIVE" : "NOT ACTIVE", "", true),
        storage:
            settingRow("ONA GAMES", `${settingsData.storage?.installedGames ?? installedGames.length} INSTALLED`, "", true) +
            settingRow("APP DATA", settingsData.storage?.appDataPath || "UNKNOWN", "", true) +
            settingRow("USED BY ONA DATA", formatBytes(settingsData.storage?.appDataBytes), "", true),
        system:
            settingRow("ONA VERSION", settingsData.system?.version || "0.1.0", "", true) +
            settingRow("PLATFORM", settingsData.system?.platform || navigator.platform, "", true) +
            settingRow("ARCHITECTURE", settingsData.system?.architecture || "UNKNOWN", "", true) +
            settingRow("APP DATA PATH", settingsData.system?.appDataPath || "UNKNOWN", "", true),
        power:
            settingRow("MINIMIZE ONA", "APPLY", "power-minimize") +
            settingRow("RESTART ONA", "APPLY", "power-restart") +
            settingRow("EXIT ONA", "APPLY", "power-exit")
    };

    settingsPanel.innerHTML =
        `<div class="settings-panel-title">${escapeHtml(category.toUpperCase())}</div>
        <div class="settings-options">${panels[category] || ""}</div>`;

}


function ensureSettingsFocus() {

    const category =
        settingsCategories?.querySelector(".settings-category.selected");

    if (!category) {
        settingsCategories
            ?.querySelector(".settings-category")
            ?.classList
            .add("selected");
    }

    if (settingsFocusLevel === "option") {
        const selectedOption =
            settingsPanel?.querySelector(".settings-option.selected:not(.disabled)");

        if (!selectedOption) {
            settingsPanel
                ?.querySelector(".settings-option:not(.disabled)")
                ?.classList
                .add("selected");
        }
    }

}


function moveSettingsSelection(direction) {

    const step =
        direction === "left" ||
        direction === "up"
            ? -1
            : 1;

    if (
        settingsFocusLevel === "category" ||
        direction === "left"
    ) {
        settingsFocusLevel =
            "category";
        settingsPanel
            ?.querySelectorAll(".settings-option")
            .forEach((option) => option.classList.remove("selected"));
        moveSelectedElement(
            settingsCategories,
            ".settings-category",
            step
        );
        renderSettingsPanel();
        return;
    }

    if (direction === "right") {
        settingsFocusLevel =
            "option";
        ensureSettingsFocus();
        return;
    }

    moveSelectedElement(
        settingsPanel,
        ".settings-option:not(.disabled)",
        step
    );

}


function activateSettingsSelection() {

    if (settingsFocusLevel === "category") {
        settingsFocusLevel =
            "option";
        ensureSettingsFocus();
        return;
    }

    const action =
        settingsPanel
            ?.querySelector(".settings-option.selected")
            ?.dataset
            ?.settingsAction;

    if (!action) {
        return;
    }

    if (action === "ui-animations") {
        shellSettings.uiAnimations =
            !shellSettings.uiAnimations;
        persistShellSettings();
        return;
    }

    if (action === "reduced-motion") {
        shellSettings.reducedMotion =
            !shellSettings.reducedMotion;
        persistShellSettings();
        return;
    }

    if (action === "visual-intensity") {
        shellSettings.visualIntensity =
            shellSettings.visualIntensity === "normal"
                ? "low"
                : "normal";
        persistShellSettings();
        return;
    }

    if (action === "ui-muted") {
        shellSettings.uiMuted =
            !shellSettings.uiMuted;
        persistShellSettings();
        return;
    }

    if (action === "ui-volume") {
        shellSettings.uiVolume =
            shellSettings.uiVolume >= 100
                ? 0
                : shellSettings.uiVolume + 10;
        persistShellSettings();
        return;
    }

    if (action === "power-minimize") {
        minimizeButton?.click();
        return;
    }

    if (action === "power-restart") {
        restartButton?.click();
        return;
    }

    if (action === "power-exit") {
        exitButton?.click();
    }

}


settingsCategories?.addEventListener(
    "click",
    (event) => {
        const category =
            event.target.closest(".settings-category");

        if (!category) {
            return;
        }

        settingsFocusLevel =
            "category";

        settingsCategories
            ?.querySelectorAll(".settings-category")
            .forEach((item) => item.classList.remove("selected"));

        category.classList.add("selected");
        renderSettingsPanel();
    }
);


settingsPanel?.addEventListener(
    "click",
    (event) => {
        const option =
            event.target.closest(".settings-option:not(.disabled)");

        if (!option) {
            return;
        }

        settingsFocusLevel =
            "option";

        settingsPanel
            ?.querySelectorAll(".settings-option")
            .forEach((item) => item.classList.remove("selected"));

        option.classList.add("selected");
        activateSettingsSelection();
    }
);


// =========================================================
// SHOW CONTROLLERS
// =========================================================

function showControllers() {

    hideAllScreens();

    controllersScreen
        ?.classList
        .add("active");

    updateControllersScreen();
    ensureControllersFocus();

}


function showAdditionalControllerQr() {

    if (playerCountValue <= 0) {
        return;
    }

    addingController =
        true;

    transitionTo(
        ONA_STATE.WAITING_CONTROLLER,
        {
            addingController:
                true,
            keepAddingController:
                true
        }
    );

}


// =========================================================
// SHOW CONTROLLER LAB
// =========================================================

function showControllerLab() {

    hideAllScreens();

    controllerLabScreen
        ?.classList
        .add("active");

    updateControllerLabConnection();

    updateControllersScreen();

    updateControllerLabProfile();

    updateControllerLabBridgeStatus();

}


// =========================================================
// STATE TRANSITIONS
// =========================================================

function transitionTo(state, context = {}) {

    if (
        state === ONA_STATE.WAITING_CONTROLLER &&
        playerCountValue > 0 &&
        !context.addingController
    ) {
        return;
    }

    if (
        state !== ONA_STATE.WAITING_CONTROLLER &&
        !context.keepAddingController
    ) {
        addingController =
            false;
    }

    setState(state);

}


// =========================================================
// STATE MACHINE
// =========================================================

function setState(state) {

    console.log(
        "ONA STATE:",
        currentState,
        "→",
        state
    );

    currentState = state;


    switch (state) {

        case ONA_STATE.WAITING_CONTROLLER:

            showControllerScreen();

            break;


        case ONA_STATE.PROFILE_SELECT:

            showProfileScreen();

            break;


        case ONA_STATE.MAIN_MENU:

            showMainMenu();

            break;

        case ONA_STATE.GAME_LIBRARY:

            showGameLibrary();

            break;

        case ONA_STATE.STORE:

            showStore();

            break;

        case ONA_STATE.SETTINGS:

            showSettings();

            break;

        case ONA_STATE.CONTROLLERS:

            showControllers();

            break;

        case ONA_STATE.CONTROLLER_LAB:

            showControllerLab();

            break;


        case ONA_STATE.QUICK_MENU:

            break;

    }

}


// =========================================================
// GAME LIBRARY DATA
// =========================================================

async function loadGameLibrary() {

    if (libraryStatus) {
        libraryStatus.textContent =
            "Loading installed games.";
    }

    try {

        const catalog =
            await invoke("list_installed_games");

        installedGames =
            Array.isArray(catalog?.games)
                ? catalog.games
                : [];

        if (
            selectedGameIndex >= installedGames.length
        ) {
            selectedGameIndex = 0;
        }

        renderGameLibrary();

    }

    catch (error) {

        console.error(
            "[ONA Library] Unable to load games:",
            error
        );

        if (libraryStatus) {
            libraryStatus.textContent =
                `Library error: ${error}`;
        }

    }

}


function renderGameLibrary() {

    if (!gameGrid) {
        return;
    }

    gameGrid.innerHTML = "";

    if (!installedGames.length) {

        if (libraryStatus) {
            libraryStatus.textContent =
                "";
        }

        gameGrid.appendChild(
            createAddGameCard(true)
        );

        for (
            let index = 0;
            index < 3;
            index++
        ) {
            const slot =
                document.createElement("div");

            slot.className =
                "game-card game-card-slot";

            gameGrid.appendChild(slot);
        }

        return;

    }

    if (libraryStatus) {
        libraryStatus.textContent =
            "";
    }

    installedGames.forEach(
        (game, index) => {

            const card =
                document.createElement("button");

            card.className =
                "game-card";

            card.dataset.gameId =
                game.id;

            card.classList.toggle(
                "selected",
                index === selectedGameIndex
            );

            const iconMarkup =
                game.icon
                    ? `<img class="game-icon" src="${escapeAttribute(game.icon)}" alt="">`
                    : `<div class="game-icon game-icon-placeholder">${escapeHtml(game.name?.slice(0, 2) || "ON")}</div>`;
            const hasBackgroundSession =
                selectedGameHasBackgroundSession(game);
            const hasOtherBackgroundSession =
                activeSessionIsBackground() &&
                activeGameSession?.gameId !== game.id;

            card.innerHTML =
                `${iconMarkup}
                <div class="game-card-body">
                    <div class="game-title">${escapeHtml(game.name || "Untitled Game")}</div>
                    <div class="game-meta">${escapeHtml(game.developer || "Unknown Developer")} / ${escapeHtml(game.version || "0.0.0")}</div>
                    <div class="game-description">${escapeHtml(game.description || "")}</div>
                    <div class="game-state">${hasBackgroundSession ? "RUNNING" : hasOtherBackgroundSession ? "ANOTHER GAME RUNNING" : game.installed ? "INSTALLED" : "NOT INSTALLED"}</div>
                </div>
                <div class="game-play-label">${hasBackgroundSession ? "A CONTINUE / X OPTIONS" : "A OPEN / X OPTIONS"}</div>`;

            card.addEventListener(
                "click",
                () => {
                    selectedGameIndex = index;
                    renderGameLibrary();
                }
            );

            gameGrid.appendChild(card);

        }
    );

    gameGrid.appendChild(
        createAddGameCard(
            selectedGameIndex === installedGames.length
        )
    );

}


function createAddGameCard(selected) {

    const card =
        document.createElement("button");

    card.className =
        "game-card game-card-add";

    card.dataset.gameAction =
        "import";

    card.classList.toggle(
        "selected",
        selected
    );

    const title =
        installedGames.length
            ? "ADD GAME"
            : "ADD YOUR FIRST GAME";

    card.innerHTML =
        `<div class="game-add-icon">+</div>
        <div class="game-card-body">
            <div class="game-title">${title}</div>
            <div class="game-description">Import a compatible game package to your library.</div>
            <div class="game-import-label">IMPORT GAME</div>
        </div>`;

    card.addEventListener(
        "click",
        () => {
            selectedGameIndex =
                installedGames.length;
            renderGameLibrary();
        }
    );

    return card;

}


function navigateGames(direction) {

    if (
        currentState !== ONA_STATE.GAME_LIBRARY
    ) {
        return;
    }

    if (!installedGames.length) {
        selectedGameIndex = 0;
        renderGameLibrary();
        return;
    }

    const libraryItemCount =
        installedGames.length + 1;

    selectedGameIndex += direction;

    if (selectedGameIndex < 0) {
        selectedGameIndex = libraryItemCount - 1;
    }

    if (selectedGameIndex >= libraryItemCount) {
        selectedGameIndex = 0;
    }

    renderGameLibrary();

}


function selectedIndexFromElements(elements) {

    const selectedIndex =
        elements.findIndex(
            (element) =>
                element.classList.contains("selected")
        );

    return selectedIndex >= 0
        ? selectedIndex
        : 0;

}


function selectElementByIndex(elements, index) {

    if (!elements.length) {
        return;
    }

    let nextIndex =
        index;

    if (nextIndex < 0) {
        nextIndex =
            elements.length - 1;
    }

    if (nextIndex >= elements.length) {
        nextIndex =
            0;
    }

    elements.forEach(
        (element) =>
            element.classList.remove("selected")
    );

    elements[nextIndex]
        .classList
        .add("selected");

}


function moveSelectedElement(container, selector, direction) {

    const elements =
        Array.from(
            container?.querySelectorAll(selector) || []
        )
        .filter(
            (element) =>
                !element.disabled &&
                !element.classList.contains("disabled") &&
                element.offsetParent !== null
        );

    if (!elements.length) {
        return;
    }

    selectElementByIndex(
        elements,
        selectedIndexFromElements(elements) + direction
    );

}


function navigateSystemMenu(direction) {

    moveSelectedElement(
        systemMenu,
        ".system-menu-item",
        direction
    );

}


function activateSelectedSystemMenuItem() {

    const selected =
        systemMenu?.querySelector(
            ".system-menu-item.selected"
        );

    selected?.click();

}


function handleStartButtonState(state) {

    const sessionGeneration =
        activeGameSession?.sessionGeneration || null;
    const gameForeground =
        nativeGameSessionActive() &&
        presentationOwner === PresentationOwner.GAME;
    const systemOverlayActive =
        presentationOwner === PresentationOwner.ONA_SYSTEM_OVERLAY &&
        systemMenuRequestState === SystemMenuRequestState.OPEN;
    const systemInputActive =
        Boolean(activeGameSession?.pid) &&
        (gameForeground || systemOverlayActive);
    let startRejectReason =
        "";

    if (!systemInputActive) {
        if (startArbitrationState !== StartArbitrationState.IDLE) {
            resetStartArbitration("NON_GAME_START_BYPASS");
        }
        startRejectReason =
            "not_game_foreground";
        console.log("[ONA START] shell path - game arbitration bypassed");
        console.log(
            `[ONA START TRACE] owner=${presentationOwner} arbiter=${startArbitrationState} timerArmed=${Boolean(startHoldTimer)} holdThresholdReached=false requestSystemMenuCalled=false rejectReason=${startRejectReason}`
        );
        return false;
    }

    if (state === "down" || state === "pressed") {
        logPostRestoreState("start_down");
        if (startArbitrationState !== StartArbitrationState.IDLE) {
            startRejectReason =
                `arbiter_${startArbitrationState}`;
            console.log(
                `[ONA START] physical DOWN ignored state=${startArbitrationState}`
            );
            console.log(
                `[ONA START TRACE] owner=${presentationOwner} arbiter=${startArbitrationState} timerArmed=${Boolean(startHoldTimer)} holdThresholdReached=false requestSystemMenuCalled=false rejectReason=${startRejectReason}`
            );
            return true;
        }

        startHoldStartedAt =
            performance.now();
        console.log(
            `[ONA START] physical DOWN timestamp=${Math.round(startHoldStartedAt)}`
        );
        console.log(
            gameForeground && gameInputForwardingEnabled
                ? "[ONA START] DOWN forwarded to game"
                : "[ONA START] DOWN observed by system overlay"
        );
        startArbitrationState =
            StartArbitrationState.PENDING;
        startArbitrationSessionGeneration =
            sessionGeneration;
        console.log("[ONA START] pending");
        console.log(
            `[ONA START TRACE] owner=${presentationOwner} arbiter=${startArbitrationState} timerArmed=true holdThresholdReached=false requestSystemMenuCalled=false rejectReason=none`
        );

        startHoldTimer =
            setTimeout(
                () => {
                    if (startArbitrationState !== StartArbitrationState.PENDING) {
                        return;
                    }

                    startHoldTimer =
                        null;
                    if (
                        startArbitrationSessionGeneration !==
                        (activeGameSession?.sessionGeneration || null)
                    ) {
                        resetStartArbitration("SESSION_CHANGED_BEFORE_HOLD");
                        return;
                    }
                    startArbitrationState =
                        StartArbitrationState.HOLD_CONSUMED;

                    const gameOwnsPresentation =
                        presentationOwner === PresentationOwner.GAME &&
                        nativeGameSessionActive();
                    const quickMenuOwnsPresentation =
                        presentationOwner === PresentationOwner.ONA_SYSTEM_OVERLAY &&
                        systemMenuRequestState === SystemMenuRequestState.OPEN;

                    if (
                        Boolean(activeGameSession?.pid) &&
                        (gameOwnsPresentation || quickMenuOwnsPresentation)
                    ) {
                        const elapsed =
                            Math.round(performance.now() - startHoldStartedAt);
                        console.log(
                            `[ONA START] hold threshold reached elapsed=${elapsed}`
                        );
                        console.log("[ONA START] classified HOLD");
                        console.log("[ONA START] game START suppressed");
                        console.log("[ONA START] HOLD detected - toggling Quick Menu");
                        console.log(
                            "[ONA SYSTEM] request source=HOLD_START"
                        );
                        console.log(
                            `[ONA START TRACE] owner=${presentationOwner} arbiter=${startArbitrationState} timerArmed=false holdThresholdReached=true requestSystemMenuCalled=true rejectReason=none`
                        );
                        requestSystemMenu("HOLD_START");
                    } else {
                        console.log(
                            `[ONA START TRACE] owner=${presentationOwner} arbiter=${startArbitrationState} timerArmed=false holdThresholdReached=true requestSystemMenuCalled=false rejectReason=not_game_foreground_at_threshold`
                        );
                        resetStartArbitration("HOLD_WITHOUT_GAME_FOREGROUND");
                    }
                },
                START_HOLD_THRESHOLD_MS
            );

        return true;
    }

    if (state === "up" || state === "released") {
        const elapsed =
            Math.round(performance.now() - startHoldStartedAt);
        console.log(
            `[ONA START] physical UP elapsed=${Number.isFinite(elapsed) ? elapsed : 0}`
        );

        if (startHoldTimer) {
            clearTimeout(startHoldTimer);
            startHoldTimer =
                null;
        }

        if (startArbitrationState === StartArbitrationState.HOLD_CONSUMED) {
            console.log("[ONA START] HOLD release consumed");
            const returnToGameWaitingForRelease =
                presentationOwner === PresentationOwner.GAME &&
                systemMenuRequestState === SystemMenuRequestState.CLOSED &&
                !gameInputForwardingEnabled;

            if (returnToGameWaitingForRelease) {
                setGameInputRouting(true).then(
                    () => {
                        resetStartArbitration("HOLD_CLOSE_RELEASED");
                        console.log("[ONA SYSTEM] game routing enabled after HOLD release");
                        assertPresentationInvariants();
                    }
                );
                return true;
            }

            resetStartArbitration("HOLD_RELEASED");
            return true;
        }

        if (startArbitrationState === StartArbitrationState.PENDING) {
            if (systemOverlayActive) {
                console.log("[ONA START] SHORT consumed by system overlay");
                resetStartArbitration("OVERLAY_SHORT_CLOSE");
                requestSystemMenu("START_OVERLAY");
                return true;
            }

            console.log("[ONA START] classified SHORT");
            console.log("[ONA START] SHORT released/forwarded");
            startArbitrationState =
                StartArbitrationState.SHORT_DISPATCHED;

            startArbitrationState =
                StartArbitrationState.IDLE;
            startHoldStartedAt =
                0;
            startArbitrationSessionGeneration =
                null;
            return true;
        }

        startArbitrationState =
            StartArbitrationState.IDLE;
        startHoldStartedAt =
            0;
        startArbitrationSessionGeneration =
            null;

        return true;
    }

    return false;

}


function directionFromJoystick(x, y) {

    const absX =
        Math.abs(x);

    const absY =
        Math.abs(y);

    if (
        absX < UI_NAVIGATION_THRESHOLD &&
        absY < UI_NAVIGATION_THRESHOLD
    ) {
        return null;
    }

    if (absX > absY) {
        return x > 0
            ? "right"
            : "left";
    }

    return y > 0
        ? "down"
        : "up";

}


function handleUiJoystickNavigation(x, y) {

    if (
        Math.abs(x) < UI_NAVIGATION_NEUTRAL_THRESHOLD &&
        Math.abs(y) < UI_NAVIGATION_NEUTRAL_THRESHOLD
    ) {
        uiNavigationLocked =
            false;

        return true;
    }

    if (uiNavigationLocked) {
        return true;
    }

    const direction =
        directionFromJoystick(
            x,
            y
        );

    if (!direction) {
        return true;
    }

    uiNavigationLocked =
        true;

    moveUiSelection(direction);

    return true;

}


function moveUiSelection(direction) {

    const step =
        direction === "left" ||
        direction === "up"
            ? -1
            : 1;

    if (importBrowserOpen) {
        navigateInstallPackages(step);
        return;
    }

    if (gameOptionsOpen) {
        navigateGameOptions(step);
        return;
    }

    if (
        currentState ===
        ONA_STATE.PROFILE_SELECT
    ) {
        navigateProfiles(step);
        return;
    }

    if (
        currentState ===
        ONA_STATE.MAIN_MENU
    ) {
        moveSelectedElement(
            mainMenu,
            ".menu-item",
            step
        );
        return;
    }

    if (
        currentState ===
        ONA_STATE.GAME_LIBRARY
    ) {
        navigateGames(step);
        return;
    }

    if (
        currentState ===
        ONA_STATE.STORE
    ) {
        moveSelectedElement(
            storeScreen,
            ".store-item",
            step
        );
        return;
    }

    if (
        currentState ===
        ONA_STATE.SETTINGS
    ) {
        moveSettingsSelection(direction);
        return;
    }

    if (
        currentState ===
        ONA_STATE.CONTROLLERS
    ) {
        ensureControllersFocus();

        moveSelectedElement(
            controllersScreen,
            ".controller-action:not(.disabled)",
            step
        );
        return;
    }

    if (
        currentState ===
        ONA_STATE.QUICK_MENU
    ) {
        navigateSystemMenu(step);
    }

}


async function importLocalGame() {

    if (uiBusy()) {
        return;
    }

    openImportBrowser();

}


function openImportBrowser() {

    importBrowserOpen =
        true;

    importGameOverlay
        ?.classList
        .add("visible");

    scanInstallSources();

}


function closeImportBrowser() {

    importBrowserOpen =
        false;

    importGameOverlay
        ?.classList
        .remove("visible");

    importBrowserState =
        "idle";

    installedPackageProfile =
        null;

    if (importDetails) {
        importDetails.hidden =
            true;
        importDetails.textContent =
            "";
    }

}


async function scanInstallSources() {

    importBrowserState =
        "scanning";

    scannedInstallPackages =
        [];

    invalidInstallPackages =
        [];

    selectedInstallPackageIndex =
        0;

    installedPackageProfile =
        null;

    pendingInstallPackage =
        null;

    renderImportBrowser();

    try {

        const report =
            await invoke(
                "scan_game_installation_sources"
            );

        scannedInstallPackages =
            Array.isArray(report?.games)
                ? report.games
                : [];

        invalidInstallPackages =
            Array.isArray(report?.invalidPackages)
                ? report.invalidPackages
                : [];

        importBrowserState =
            scannedInstallPackages.length
                ? "found"
                : "empty";

        renderImportBrowser(report);

    }

    catch (error) {

        console.error(
            "[ONA Import] Scan failed:",
            error
        );

        importBrowserState =
            "error";

        invalidInstallPackages =
            [
                {
                    error:
                        String(error)
                }
            ];

        renderImportBrowser();

    }

}


function renderImportBrowser(report = {}) {

    if (!importGameOverlay) {
        return;
    }

    const sourceCount =
        Number(report?.sources?.length || 0);

    if (importSourceCount) {
        importSourceCount.textContent =
            importBrowserState === "scanning"
                ? "SCANNING"
                : `${sourceCount} SOURCE${sourceCount === 1 ? "" : "S"}`;
    }

    if (importResults) {
        importResults.innerHTML =
            "";
    }

    if (importDetails) {
        importDetails.hidden =
            true;
        importDetails.textContent =
            "";
    }

    switch (importBrowserState) {

        case "scanning":

            setImportStatus(
                "SEARCHING FOR ONA GAMES..."
            );

            renderImportMessage(
                "SCANNING EXTERNAL STORAGE",
                "Looking for ONA Library on available drives."
            );

            setImportActions(
                ["B BACK"]
            );

            return;

        case "empty":

            setImportStatus(
                "NO ONA GAMES FOUND"
            );

            renderImportMessage(
                "INSERT A USB DRIVE",
                "ONA is looking for a folder named ONA Library."
            );

            setImportActions(
                ["A SCAN AGAIN", "B BACK"]
            );

            renderInvalidPackageDetails();

            return;

        case "installing":

            setImportStatus(
                `${installActionLabel(pendingInstallPackage || selectedInstallPackage()).toUpperCase()} ${pendingInstallPackage?.name || selectedInstallPackage()?.name || "GAME"}`
            );

            renderImportMessage(
                "INSTALLING...",
                "Copying and validating game files in ONA local storage."
            );

            setImportActions(
                []
            );

            return;

        case "confirm-replace":

            renderInstallConfirmation();

            return;

        case "installed":

            setImportStatus(
                `${installedPackageProfile?.name || "GAME"} READY`
            );

            renderImportMessage(
                "LIBRARY UPDATED",
                "The local ONA Game Library now points to this package."
            );

            setImportActions(
                ["A LIBRARY", "B BACK TO LIBRARY"]
            );

            return;

        case "package-error":

        case "error":

            setImportStatus(
                "THIS GAME PACKAGE COULD NOT BE INSTALLED"
            );

            renderImportMessage(
                "PACKAGE NOT COMPATIBLE",
                "The package is not compatible with this version of ONA."
            );

            setImportActions(
                ["A DETAILS", "B BACK"]
            );

            return;

        default:

            setImportStatus(
                `${scannedInstallPackages.length} ONA GAME${scannedInstallPackages.length === 1 ? "" : "S"} FOUND`
            );

            renderInstallPackageCards();

            setImportActions(
                ["A INSTALL", "B BACK"]
            );

            renderInvalidPackageDetails();

            return;

    }

}


function setImportStatus(text) {

    if (importStatus) {
        importStatus.textContent =
            text;
    }

}


function setImportActions(actions) {

    if (!importActions) {
        return;
    }

    importActions.innerHTML =
        actions
            .map((action) => `<span>${escapeHtml(action)}</span>`)
            .join("");

}


function renderImportMessage(title, copy) {

    if (!importResults) {
        return;
    }

    importResults.innerHTML =
        `<div class="import-message">
            <div class="import-pulse"></div>
            <strong>${escapeHtml(title)}</strong>
            <span>${escapeHtml(copy)}</span>
        </div>`;

}


function renderInstallPackageCards() {

    if (!importResults) {
        return;
    }

    scannedInstallPackages.forEach(
        (game, index) => {

            const card =
                document.createElement("button");

            card.className =
                "import-game-card";

            card.classList.toggle(
                "selected",
                index === selectedInstallPackageIndex
            );

            const iconMarkup =
                game.icon
                    ? `<img class="import-game-icon" src="${escapeAttribute(game.icon)}" alt="">`
                    : `<div class="import-game-icon import-game-icon-placeholder">${escapeHtml(game.name?.slice(0, 2) || "ON")}</div>`;
            const actionLabel =
                installActionLabel(game).toUpperCase();
            const installedVersion =
                game.installedVersion
                    ? `INSTALLED ${escapeHtml(game.installedVersion)}`
                    : "NOT INSTALLED";

            card.innerHTML =
                `${iconMarkup}
                <div class="import-game-info">
                    <span class="import-game-source">${escapeHtml(game.sourceName || "EXTERNAL STORAGE")}</span>
                    <strong>${escapeHtml(game.name || "Untitled Game")}</strong>
                    <span>${escapeHtml(game.gameId || "")}</span>
                    <span>PACKAGE ${escapeHtml(game.version || "0.0.0")} / ${installedVersion}</span>
                    <span>${game.alreadyInstalled ? "GAME ALREADY INSTALLED" : "ONA COMPATIBLE"}</span>
                    <em>${actionLabel}</em>
                </div>`;

            card.addEventListener(
                "click",
                () => {
                    selectedInstallPackageIndex =
                        index;
                    renderImportBrowser({
                        sources:
                            []
                    });
                }
            );

            importResults.appendChild(card);

        }
    );

}


function renderInvalidPackageDetails() {

    if (
        !importDetails ||
        !invalidInstallPackages.length
    ) {
        return;
    }

    importDetails.hidden =
        false;

    importDetails.textContent =
        `Developer details: ${invalidInstallPackages.length} incompatible package${invalidInstallPackages.length === 1 ? "" : "s"} ignored.`;

}


function renderInstallConfirmation() {

    const game =
        pendingInstallPackage || selectedInstallPackage();

    if (!game) {
        importBrowserState =
            "found";
        renderImportBrowser();
        return;
    }

    const actionLabel =
        installActionLabel(game).toUpperCase();
    const installedVersion =
        game.installedVersion || "not installed";
    const packageVersion =
        game.version || "unknown";

    setImportStatus(
        `${actionLabel} ${game.name || "GAME"}`
    );

    if (importResults) {
        importResults.innerHTML =
            `<div class="import-message">
                <div class="import-pulse"></div>
                <strong>${escapeHtml(actionLabel)}</strong>
                <span>${escapeHtml(game.name || "Untitled Game")}</span>
                <span>Installed: ${escapeHtml(installedVersion)}</span>
                <span>Package: ${escapeHtml(packageVersion)}</span>
            </div>`;
    }

    if (importDetails) {
        importDetails.hidden =
            false;
        importDetails.textContent =
            "Developer details: ONA will validate the package, stage the replacement, and keep the current installation if replacement fails.";
    }

    setImportActions(
        [`A ${actionLabel}`, "B CANCEL"]
    );

}


function installActionLabel(game) {

    switch (game?.installAction) {

        case "update":
            return "Update Game";

        case "reinstall":
            return "Reinstall Game";

        case "downgrade":
            return "Install Older Version";

        default:
            return "Install Game";

    }

}


function selectedInstallPackage() {

    return scannedInstallPackages[
        selectedInstallPackageIndex
    ];

}


function navigateInstallPackages(direction) {

    if (
        !importBrowserOpen ||
        importBrowserState !== "found" ||
        !scannedInstallPackages.length
    ) {
        return;
    }

    selectedInstallPackageIndex += direction;

    if (selectedInstallPackageIndex < 0) {
        selectedInstallPackageIndex =
            scannedInstallPackages.length - 1;
    }

    if (
        selectedInstallPackageIndex >=
        scannedInstallPackages.length
    ) {
        selectedInstallPackageIndex =
            0;
    }

    renderImportBrowser();

}


async function activateImportBrowser() {

    if (!importBrowserOpen) {
        return;
    }

    if (uiBusy()) {
        return;
    }

    if (
        importBrowserState === "empty"
    ) {
        scanInstallSources();
        return;
    }

    if (
        importBrowserState === "installed"
    ) {
        if (installedPackageProfile?.id) {
            closeImportBrowser();
            await loadGameLibrary();
            selectedGameIndex =
                installedGames.findIndex(
                    (game) =>
                        game.id === installedPackageProfile.id
                );

            if (selectedGameIndex < 0) {
                selectedGameIndex =
                    0;
            }
        }

        return;
    }

    if (
        importBrowserState === "confirm-replace"
    ) {
        await installSelectedPackage(
            pendingInstallPackage || selectedInstallPackage()
        );
        return;
    }

    if (
        importBrowserState === "error" ||
        importBrowserState === "package-error"
    ) {
        showImportDeveloperDetails();
        return;
    }

    if (
        importBrowserState !== "found"
    ) {
        return;
    }

    const game =
        selectedInstallPackage();

    if (!game) {
        return;
    }

    if (game.alreadyInstalled) {
        importBrowserState =
            "confirm-replace";
        pendingInstallPackage =
            game;
        renderImportBrowser();
        return;
    }

    await installSelectedPackage(game);

}


async function installSelectedPackage(game) {

    if (!game) {
        return;
    }

    if (libraryStatus) {
        libraryStatus.textContent =
            `${installActionLabel(game)}.`;
    }

    setUiOperation("installing");

    pendingInstallPackage =
        game;

    importBrowserState =
        "installing";

    renderImportBrowser();

    try {

        const profile =
            await invoke(
                "import_local_game",
                {
                    sourceDir:
                        String(game.packagePath),
                    action:
                        game.installAction || "install"
                }
            );

        console.log(
            "[ONA Library] Imported:",
            profile
        );

        installedPackageProfile =
            profile;

        pendingInstallPackage =
            null;

        importBrowserState =
            "installed";
        setUiOperation("idle");

        await loadGameLibrary();

        renderImportBrowser();

    }

    catch (error) {

        console.error(
            "[ONA Library] Import failed:",
            error
        );

        importBrowserState =
            "package-error";
        setUiOperation("idle");
        pendingInstallPackage =
            null;

        invalidInstallPackages =
            [
                {
                    error:
                        String(error) === "GAME_IS_CURRENTLY_RUNNING"
                            ? "GAME IS CURRENTLY RUNNING. Close the game before updating or reinstalling."
                            : String(error)
                }
            ];

        renderImportBrowser();

    }

}


function backFromImportBrowser() {

    if (!importBrowserOpen) {
        return;
    }

    if (
        importBrowserState === "installed"
    ) {
        closeImportBrowser();
        loadGameLibrary();
        return;
    }

    if (
        importBrowserState === "confirm-replace"
    ) {
        importBrowserState =
            "found";
        pendingInstallPackage =
            null;
        renderImportBrowser();
        return;
    }

    closeImportBrowser();

}


function showImportDeveloperDetails() {

    if (!importDetails) {
        return;
    }

    importDetails.hidden =
        false;

    importDetails.textContent =
        `Developer details: ${invalidInstallPackages
            .map((packageInfo) => packageInfo.error)
            .join(" / ") || "No details available."}`;

}


async function launchSelectedGame() {

    if (uiBusy()) {
        return;
    }

    const game =
        installedGames[selectedGameIndex];

    if (!game) {
        return;
    }

    if (presentationOwner === PresentationOwner.ONA_SHELL) {
        console.log(
            "[ONA Presentation] normalizing shell before PLAY"
        );
        await invoke("restore_shell_window_presentation").catch(
            (error) =>
                console.warn("[ONA Presentation] Shell normalization before PLAY failed:", error)
        );
        await invoke("release_presentation_guard").catch(
            (error) =>
                console.warn("[ONA Presentation] Guard normalization before PLAY failed:", error)
        );
        document.body
            .classList
            .remove("game-system-overlay");
        await setGameInputRouting(false);
    }

    if (selectedGameHasBackgroundSession(game)) {
        await continueActiveGameSession();
        return;
    }

    if (
        activeSessionIsBackground() &&
        activeGameSession?.gameId !== game.id
    ) {
        openAnotherGameRunningPrompt(game);
        return;
    }

    setUiOperation("launching");
    setGameSessionState(
        GameSessionState.LAUNCHING
    );
    setPresentationOwner(
        PresentationOwner.ONA_TRANSITION_GUARD
    );
    setConsolePresentationState(
        ConsolePresentationState.PREPARING
    );

    showGameLifecycleOverlay(
        "PREPARING",
        game.name,
        "Preparing console presentation."
    );
    console.log(
        "[ONA Presentation] owner ONA_SHELL -> ONA_TRANSITION_GUARD"
    );

    try {
        await requirePresentationGuard(
            "LAUNCHING_GAME",
            "launch"
        );
    }
    catch (error) {
        console.error(
            "[ONA Presentation] Guard required before spawn failed:",
            error
        );
        setUiOperation("idle");
        setGameSessionState(
            GameSessionState.IDLE
        );
        setPresentationOwner(
            PresentationOwner.ONA_SHELL
        );
        setConsolePresentationState(
            ConsolePresentationState.IDLE
        );
        hideGameLifecycleOverlay();
        return;
    }

    console.log(
        "[ONA Presentation] State=PREPARING game=",
        game
    );

    try {
        await invoke(
            "hide_game_cursor"
        );
    }
    catch (error) {
        console.warn(
            "[ONA Cursor] Could not hide cursor:",
            error
        );
    }

    await waitForTransition();

    if (libraryStatus) {
        libraryStatus.textContent =
            `Launching ${game.name}.`;
    }

    try {

        setConsolePresentationState(
            ConsolePresentationState.LAUNCHING
        );

        showGameLifecycleOverlay(
            "LAUNCHING",
            game.name,
            "Starting native game process."
        );
        console.log(
            "[ONA Presentation] spawning game"
        );

        const status =
            await invoke(
                "launch_installed_game",
                {
                    gameId:
                        game.id
                }
            );

        console.log(
            "[ONA Launcher]",
            status
        );
        console.log(
            "[ONA Presentation] LAUNCH PID:",
            status.pid || "none"
        );

        runningGameId =
            game.id;
        activeSessionGeneration +=
            1;
        setActiveGameSession({
            sessionGeneration:
                activeSessionGeneration,
            gameId:
                game.id,
            gameName:
                game.name,
            pid:
                status.pid || null,
            state:
                GameSessionState.LAUNCHING,
            presentationState:
                ConsolePresentationState.LAUNCHING,
            startedAt:
                Date.now(),
            gameExitingConsumed:
                false
        });

        if (libraryStatus) {
            libraryStatus.textContent =
                status.pid
                    ? `${game.name} is running. PID ${status.pid}.`
                    : `${game.name} launch state: ${status.state}.`;
        }

        if (status.pid) {
            setConsolePresentationState(
                ConsolePresentationState.WAITING_FOR_READY
            );

            showGameLifecycleOverlay(
                "LAUNCHING",
                game.name,
                "Waiting for game readiness."
            );
            console.log(
                "[ONA Presentation] State=WAITING_FOR_READY timeoutMs=5000"
            );

            const handoff =
                await invoke(
                    "wait_for_game_handoff_ready",
                    {
                        pid:
                            status.pid,
                        timeoutMs:
                            5000
                    }
                );
            if (handoff.primaryWindow?.hwnd) {
                updateActiveGameSession({
                    primaryHwnd:
                        handoff.primaryWindow.hwnd,
                    primaryWindowClass:
                        handoff.primaryWindow.windowClass,
                    primaryWindowTitle:
                        handoff.primaryWindow.windowTitle,
                    expectedBounds:
                        handoff.primaryWindow.expectedBounds
                });
            }
            if (handoff.gameReady) {
                console.log(
                    "[ONA Presentation] GAME_READY received"
                );
            }

            showGameLifecycleOverlay(
                handoff.presentationValid && handoff.gameReady
                    ? "GAME READY"
                    : handoff.presentationValid && handoff.legacyFallback
                        ? "LEGACY HANDOFF"
                        : "DISPLAY NOT READY",
                game.name,
                handoff.presentationValid && handoff.gameReady
                    ? "Game confirmed ready for console presentation."
                    : handoff.presentationValid && handoff.legacyFallback
                        ? "Game window is on the ONA Gaming Display without GAME_READY."
                        : handoff.rejectionReason
                            ? `Presentation rejected: ${handoff.rejectionReason}. ONA will stay visible.`
                            : handoff.windowReady
                            ? "Game window is not ready for console presentation. ONA will stay visible."
                        : "Game display was not confirmed. ONA will stay visible."
                ,
                formatHandoffDiagnostics(handoff)
            );
            console.log(
                "[ONA Presentation] Handoff result:",
                handoff
            );

            if (handoff.presentationValid && (handoff.gameReady || handoff.legacyFallback)) {
                await waitForTransition(
                    handoff.legacyFallback
                        ? 520
                        : 280
                );

                const finalized =
                    await finalizeSafeHandoffToGame(
                        game,
                        status.pid,
                        GameSessionState.RUNNING_FOREGROUND
                    );

                if (finalized) {
                    startRunningGameMonitor(game);
                }
            }
            else {
                setConsolePresentationState(
                    ConsolePresentationState.FAILED
                );
                setGameSessionState(
                    GameSessionState.FAILED
                );
                await rollbackFailedLaunch(
                    game,
                    "HANDOFF_NOT_SAFE",
                    handoff
                );
            }
        }
        else {
            hideGameLifecycleOverlay();
            setUiOperation("idle");
            resetStartArbitration("GAME_SESSION_END_NO_PID");
            setActiveGameSession(null);
            setGameSessionState(
                GameSessionState.IDLE
            );
            setPresentationOwner(
                PresentationOwner.ONA_SHELL
            );
            setConsolePresentationState(
                ConsolePresentationState.IDLE
            );
        }

    }

    catch (error) {

        console.error(
            "[ONA Launcher] Launch failed:",
            error
        );

        if (libraryStatus) {
            libraryStatus.textContent =
                "GAME COULD NOT START";
        }

        showGameLifecycleOverlay(
            "GAME COULD NOT START",
            game.name,
            "The game could not be launched.",
            String(error)
        );

        setConsolePresentationState(
            ConsolePresentationState.FAILED
        );
        setGameSessionState(
            GameSessionState.FAILED
        );

        await rollbackFailedLaunch(game);

    }

}


async function rollbackFailedLaunch(game, reason = "LAUNCH_FAILED", diagnostics = null) {

    console.warn(
        "[ONA Presentation] Rolling back launch:",
        {
            reason,
            diagnostics
        }
    );

    showPresentationGuard(
        "RETURNING",
        game.name,
        "Cancelling launch and restoring ONA."
    );

    try {
        await requirePresentationGuard(
            "RETURNING_TO_ONA",
            "ROLLBACK"
        );
    }
    catch (error) {
        console.error(
            "[ONA Presentation] Rollback guard failed; preserving current owner:",
            error
        );
        return;
    }

    try {
        await invoke(
            "terminate_running_game"
        );
    }
    catch (error) {
        console.error(
            "[ONA Launcher] Rollback termination failed:",
            error
        );
    }

    runningGameId =
        null;
    resetStartArbitration("GAME_SESSION_END_ROLLBACK");
    setActiveGameSession(null);

    setUiOperation("idle");
    setGameSessionState(
        GameSessionState.RETURNING
    );

    try {
        await invoke(
            "restore_shell_after_game"
        );
    }
    catch (error) {
        console.error(
            "[ONA Launcher] Rollback shell restore failed:",
            error
        );
    }

    if (libraryStatus) {
        libraryStatus.textContent =
            `${game.name} did not enter the ONA Gaming Display. Launch was cancelled.`;
    }

    try {
        await invoke(
            "restore_game_cursor"
        );
    }
    catch (error) {
        console.warn(
            "[ONA Cursor] Could not restore cursor after rollback:",
            error
        );
    }

    console.warn(
        "[ONA Presentation] User message:",
        `${game.name} did not enter the ONA Gaming Display. Launch was cancelled.`
    );

    await waitForTransition(2400);
    hideGameLifecycleOverlay();
    setConsolePresentationState(
        ConsolePresentationState.IDLE
    );
    setGameSessionState(
        GameSessionState.IDLE
    );
    setPresentationOwner(
        PresentationOwner.ONA_SHELL
    );
    await invoke("release_presentation_guard").catch(
        (error) =>
            console.warn("[ONA Presentation] Could not release guard:", error)
    );

}


function formatHandoffDiagnostics(handoff) {

    const diagnostics =
        handoff?.diagnostics;

    if (!diagnostics) {
        return "";
    }

    const expected =
        diagnostics.expectedBounds
            ? `${diagnostics.expectedBounds.x},${diagnostics.expectedBounds.y} ${diagnostics.expectedBounds.width}x${diagnostics.expectedBounds.height}`
            : "unknown";
    const detected =
        diagnostics.detectedBounds
            ? `${diagnostics.detectedBounds.x},${diagnostics.detectedBounds.y} ${diagnostics.detectedBounds.width}x${diagnostics.detectedBounds.height}`
            : "none";
    const client =
        diagnostics.detectedClientBounds
            ? `${diagnostics.detectedClientBounds.x},${diagnostics.detectedClientBounds.y} ${diagnostics.detectedClientBounds.width}x${diagnostics.detectedClientBounds.height}`
            : "none";
    const style =
        diagnostics.windowStyle
            ? `${diagnostics.windowStyle.styleHex}/${diagnostics.windowStyle.exStyleHex} caption=${diagnostics.windowStyle.hasCaption} frame=${diagnostics.windowStyle.hasThickFrame || diagnostics.windowStyle.hasDialogFrame}`
            : "unknown";

    return [
        `expected=${expected}`,
        `detected=${detected}`,
        `client=${client}`,
        `monitor=${diagnostics.detectedMonitor || "none"}->${diagnostics.expectedMonitor || "unknown"}`,
        `style=${style}`,
        `gameReady=${diagnostics.gameReadyReceived}`,
        `presentationValid=${diagnostics.presentationValid}`,
        `reason=${diagnostics.rejectionReason || "none"}`
    ].join(" | ");

}


function showGameLifecycleOverlay(title, gameName, status, details = "") {

    gameLifecycleOverlay
        ?.classList
        .add("visible");

    gameLifecycleOverlay?.setAttribute(
        "data-presentation-state",
        consolePresentationState
    );

    if (gameLifecycleTitle) {
        gameLifecycleTitle.textContent =
            title;
    }

    if (gameLifecycleName) {
        gameLifecycleName.textContent =
            gameName || "GAME";
    }

    if (gameLifecycleStatus) {
        gameLifecycleStatus.textContent =
            status || "";
    }

    if (gameLifecycleDetails) {
        gameLifecycleDetails.hidden =
            !details;
        gameLifecycleDetails.textContent =
            details
                ? `Developer details: ${details}`
                : "";
    }

}


function hideGameLifecycleOverlay() {

    gameLifecycleOverlay
        ?.classList
        .remove("visible");

}


function clearGameLifecycleOverlay() {

    hideGameLifecycleOverlay();

    if (gameLifecycleTitle) {
        gameLifecycleTitle.textContent =
            "";
    }

    if (gameLifecycleName) {
        gameLifecycleName.textContent =
            "";
    }

    if (gameLifecycleStatus) {
        gameLifecycleStatus.textContent =
            "";
    }

    if (gameLifecycleDetails) {
        gameLifecycleDetails.hidden =
            true;
        gameLifecycleDetails.textContent =
            "";
    }

}


function showPresentationGuard(title = "RETURNING", gameName = "GAME", status = "Restoring ONA presentation.") {

    setPresentationOwner(
        PresentationOwner.ONA_TRANSITION_GUARD
    );
    setConsolePresentationState(
        ConsolePresentationState.RETURNING
    );
    showGameLifecycleOverlay(
        title,
        gameName,
        status
    );
    console.log(
        "[ONA Presentation] Windows exposure prevented"
    );

}


function waitForShellFrames(count = 2) {

    return new Promise(
        (resolve) => {
            let remaining =
                Math.max(1, count);

            const step =
                () => {
                    remaining -= 1;

                    if (remaining <= 0) {
                        resolve();
                        return;
                    }

                    window.requestAnimationFrame(step);
                };

            window.requestAnimationFrame(step);
        }
    );

}


async function requestOnaExit() {

    if (
        onaShutdownInProgress ||
        uiBusy()
    ) {
        return;
    }

    try {
        const status =
            await invoke(
                "running_game_status"
            );

        if (status.state === "running" && status.pid) {
            onaExitConfirmOpen =
                true;

            showGameLifecycleOverlay(
                "EXIT ONA?",
                "GAME RUNNING",
                "A CLOSE GAME AND EXIT ONA / B CANCEL"
            );

            return;
        }
    }
    catch (error) {
        console.error(
            "[ONA Shutdown] Runtime status failed:",
            error
        );
    }

    await closeOnaOwnedRuntimeAndExit();

}


function cancelOnaExit() {

    onaExitConfirmOpen =
        false;

    hideGameLifecycleOverlay();

}


async function closeOnaOwnedRuntimeAndExit() {

    if (onaShutdownInProgress) {
        return;
    }

    onaShutdownInProgress =
        true;

    console.log(
        "Exiting ONA..."
    );

    try {

        beginPresentationTransition("SHUTDOWN");
        setUiOperation("shutdown");

        if (activeGameSession?.pid || runningGameId) {
            showPresentationGuard(
                "RETURNING",
                activeSessionGame()?.name || "GAME",
                "Closing active game session before exiting ONA."
            );
            await requirePresentationGuard(
                "RETURNING_TO_ONA",
                "SHUTDOWN"
            );
        }

        await setGameInputRouting(false);

        await invoke(
            "terminate_running_game"
        );
        resetStartArbitration("GAME_SESSION_END_SHUTDOWN");
        setActiveGameSession(null);

        await invoke("disable_escape_system_shortcut")
            .catch(
                (error) =>
                    console.warn("[ONA Shutdown] ESC shortcut cleanup failed:", error)
            );

        await invoke("exit_ona_process");

    }

    catch (error) {

        onaShutdownInProgress =
            false;
        setUiOperation("idle");

        console.error(
            "Unable to close ONA:",
            error
        );

    }

}


function startRunningGameMonitor(game) {

    if (runningGamePollTimer) {
        clearInterval(runningGamePollTimer);
    }

    const monitorSessionGeneration =
        activeGameSession?.sessionGeneration;
    const monitorPid =
        activeGameSession?.pid;
    const monitorGameId =
        game.id;

    runningGamePollTimer =
        setInterval(
            async () => {

                if (
                    !activeGameSession ||
                    activeGameSession.sessionGeneration !== monitorSessionGeneration ||
                    activeGameSession.pid !== monitorPid ||
                    activeGameSession.gameId !== monitorGameId
                ) {
                    return;
                }

                try {

                    const status =
                        await invoke(
                            "running_game_status"
                        );

                    const lifecycleStatus =
                        await invoke(
                            "game_lifecycle_bridge_status"
                        ).catch(
                            () =>
                                null
                        );

                    if (
                        lifecycleStatus?.gameExiting &&
                        !activeGameSession?.gameExitingConsumed &&
                        presentationOwner === PresentationOwner.GAME
                    ) {
                        updateActiveGameSession({
                            gameExitingConsumed:
                                true
                        });
                        console.log(
                            `[ONA Lifecycle] GAME_EXITING consumed session=${monitorSessionGeneration}`
                        );
                        activeGameExitTrace =
                            createPresentationTimingTrace("GAME_EXITING");
                        activeGameExitTrace.mark("T0 GAME_EXITING_RECEIVED");
                        console.log(
                            "[ONA Presentation] GAME_EXITING received"
                        );
                        console.log(
                            "[ONA Session] GAME_EXITING received - activating transition guard"
                        );
                        const exitTransaction =
                            startExitTransaction(
                                "COOPERATIVE_EXIT_IN_PROGRESS",
                                activeGameSession
                            );
                        showPresentationGuard(
                            "RETURNING",
                            game.name,
                            "Game session is ending."
                        );

                        try {
                            await requirePresentationGuard(
                                "RETURNING_TO_ONA",
                                "GAME_EXITING",
                                activeGameExitTrace
                            );
                            setUiOperation("restoring");
                            setGameSessionState(
                                GameSessionState.RETURNING
                            );
                            exitTransaction.guardAcquired =
                                true;
                        }
                        catch (error) {
                            console.error(
                                "[ONA Session] Could not activate guard during game exit:",
                                error
                            );
                        }
                    }
                    else if (
                        lifecycleStatus?.gameExiting &&
                        activeGameSession?.gameExitingConsumed
                    ) {
                        console.log(
                            "[ONA Lifecycle] duplicate GAME_EXITING ignored"
                        );
                    }

                    if (
                        status.state !== "running" ||
                        status.gameId !== game.id
                    ) {
                        console.log(
                            "[ONA Runtime] Game process ended or changed:",
                            status
                        );
                        const cooperativeExit =
                            activeExitTransaction?.sessionGeneration === monitorSessionGeneration &&
                            activeExitTransaction.state === "COOPERATIVE_EXIT_IN_PROGRESS";
                        const exitTrace =
                            activeExitTransaction?.trace ||
                            activeGameExitTrace ||
                            createPresentationTimingTrace(
                                cooperativeExit
                                    ? "GAME_EXITING"
                                    : "PROCESS_EXIT"
                            );
                        exitTrace.mark("GAME_PROCESS_EXIT");

                        clearInterval(runningGamePollTimer);
                        runningGamePollTimer =
                            null;

                        const wasBackground =
                            activeSessionIsBackground();
                        const endingSession =
                            activeGameSession
                                ? { ...activeGameSession }
                                : null;

                        resetStartArbitration("GAME_SESSION_END");
                        setActiveGameSession(null);
                        runningGameId =
                            null;
                        await setGameInputRouting(false);

                        if (wasBackground) {
                            setGameSessionState(
                                GameSessionState.EXITED
                            );
                            setGameSessionState(
                                GameSessionState.IDLE
                            );
                            renderHome();
                            renderGameLibrary();
                            return;
                        }

                        const transitionId =
                            cooperativeExit
                                ? null
                                : beginPresentationTransition("PROCESS_EXIT");

                        if (
                            transitionId &&
                            !presentationTransitionCurrent(transitionId)
                        ) {
                            return;
                        }

                        if (cooperativeExit) {
                            console.log(
                                "[ONA Exit] process death completes existing cooperative transaction"
                            );
                        } else {
                            startExitTransaction(
                                "CRASH_RECOVERY_IN_PROGRESS",
                                endingSession
                            );
                        }

                        setUiOperation("restoring");
                        setGameSessionState(
                            GameSessionState.RETURNING
                        );
                        if (!cooperativeExit) {
                            showPresentationGuard(
                                "RETURNING",
                                game.name,
                                "Game session ended."
                            );
                        }

                        await restoreShellAfterGame(
                            game,
                            exitTrace,
                            {
                                guardAlreadyAcquired:
                                    cooperativeExit
                            }
                        );
                        completeExitTransaction(
                            cooperativeExit
                                ? "COOPERATIVE_PROCESS_EXIT"
                                : "CRASH_PROCESS_EXIT"
                        );
                        if (transitionId) {
                            endPresentationTransition(transitionId);
                        }

                    }

                }

                catch (error) {
                    console.error(
                        "[ONA Runtime] Game monitor failed:",
                        error
                    );
                }

            },
            250
        );

}


async function restoreShellAfterGame(game, trace = null, options = {}) {

    if (!options.guardAlreadyAcquired) {
        showPresentationGuard(
            "RETURNING",
            game.name,
            "Restoring ONA Gaming Display."
        );
    }

    if (!options.guardAlreadyAcquired) {
        try {
            await requirePresentationGuard(
                "RETURNING_TO_ONA",
                "RESTORE_SHELL_AFTER_GAME",
                trace
            );
        }
        catch (error) {
            console.error("[ONA Presentation] Guard failed before shell restore:", error);
            return;
        }
    }

    setConsolePresentationState(
        ConsolePresentationState.RETURNING
    );
    setPresentationOwner(
        PresentationOwner.ONA_TRANSITION_GUARD
    );

    try {
        trace?.mark("T5 ALLOW_GAME_PRESENTATION_RESTORE");
        trace?.mark("T6 PREPARE_SHELL");
        await invoke(
            "restore_shell_after_game"
        );
        const shellStatus =
            await invoke("shell_window_presentation_status");

        if (shellStatus?.visible && shellStatus?.fullscreen) {
            trace?.mark("T7 SHELL_VISIBLE_FULLSCREEN_CONFIRMED");
        }
    }
    catch (error) {
        console.error(
            "[ONA Runtime] Shell restore failed:",
            error
        );
    }

    try {
        await invoke(
            "restore_game_cursor"
        );
    }
    catch (error) {
        console.warn(
            "[ONA Cursor] Could not restore cursor:",
            error
        );
    }

    await loadGameLibrary();

    const gameIndex =
        installedGames.findIndex(
            (installedGame) =>
                installedGame.id === game.id
        );

    selectedGameIndex =
        gameIndex >= 0
            ? gameIndex
            : 0;

    transitionTo(
        ONA_STATE.GAME_LIBRARY
    );

    await waitForShellFrames(2);

    setUiOperation("idle");
    setConsolePresentationState(
        ConsolePresentationState.IDLE
    );
    setGameSessionState(
        GameSessionState.IDLE
    );
    await invoke("release_presentation_guard")
        .then(
            () =>
                trace?.mark("T8 GUARD_RELEASE")
        )
        .catch(
            (error) =>
                console.warn("[ONA Presentation] Could not release guard:", error)
        );
    setPresentationOwner(
        PresentationOwner.ONA_SHELL
    );
    trace?.mark("T9 ONA_SHELL");
    hideGameLifecycleOverlay();

    if (libraryStatus) {
        libraryStatus.textContent =
            `${game.name} closed.`;
    }

}


async function showRuntimeStatus() {

    try {

        const status =
            await invoke(
                "running_game_status"
            );

        const bridge =
            await invoke(
                "game_input_bridge_status"
            );

        if (libraryStatus) {
            libraryStatus.textContent =
                `Runtime ${status.state}. PID ${status.pid || "none"}. Input bridge ${bridge.address}.`;
        }

    }

    catch (error) {
        console.error(
            "[ONA Runtime] Status failed:",
            error
        );
    }

}


function openGameOptions() {

    const game =
        installedGames[selectedGameIndex];

    if (!game) {
        return;
    }

    selectedOptionsGame =
        game;

    gameOptionsOpen =
        true;

    gameOptionsMode =
        selectedGameHasBackgroundSession(game)
            ? "background-session"
            : "menu";

    selectedGameOptionIndex =
        0;

    renderGameOptions();

    gameOptionsOverlay
        ?.classList
        .add("visible");

}


function openAnotherGameRunningPrompt(game) {

    pendingPlayAfterCloseGame =
        game;

    selectedOptionsGame =
        game;

    gameOptionsOpen =
        true;

    gameOptionsMode =
        "another-game-running";

    selectedGameOptionIndex =
        0;

    if (gameOptionsTitle) {
        gameOptionsTitle.textContent =
            "ANOTHER GAME IS RUNNING";
    }

    renderGameOptions();

    gameOptionsOverlay
        ?.classList
        .add("visible");

}


function closeGameOptions() {

    gameOptionsOpen =
        false;

    gameOptionsMode =
        "menu";

    selectedGameOptionIndex =
        0;

    selectedOptionsGame =
        null;

    gameOptionsOverlay
        ?.classList
        .remove("visible");

}


function renderGameOptions() {

    const game =
        selectedOptionsGame;

    if (gameOptionsTitle) {
        gameOptionsTitle.textContent =
            gameOptionsMode === "another-game-running"
                ? "ANOTHER GAME IS RUNNING"
                : game?.name || "GAME";
    }

    if (gameOptionsDetails) {
        gameOptionsDetails.hidden =
            true;
        gameOptionsDetails.textContent =
            "";
    }

    renderGameOptionsActions();

    if (!gameOptionsStatus) {
        return;
    }

    if (gameOptionsMode === "confirm-uninstall") {
        gameOptionsStatus.textContent =
            "The game will be removed from this ONA system.";
        gameOptionsOverlay
            ?.classList
            .add("confirming");
        return;
    }

    if (gameOptionsMode === "background-session") {
        gameOptionsStatus.textContent =
            "RUNNING";
        return;
    }

    if (gameOptionsMode === "another-game-running") {
        const currentGame =
            activeSessionGame();
        gameOptionsStatus.textContent =
            `${currentGame?.name || "A game"} is currently running. Starting ${game?.name || "this game"} will close the current game.`;
        gameOptionsOverlay
            ?.classList
            .add("confirming");
        return;
    }

    if (gameOptionsMode === "uninstalling") {
        gameOptionsStatus.textContent =
            "UNINSTALLING...";
        return;
    }

    if (gameOptionsMode === "uninstalled") {
        gameOptionsStatus.textContent =
            "UNINSTALLED";
        return;
    }

    if (gameOptionsMode === "error") {
        gameOptionsStatus.textContent =
            "GAME IS CURRENTLY RUNNING";
        return;
    }

    gameOptionsOverlay
        ?.classList
        .remove("confirming");

    gameOptionsStatus.textContent =
        "A SELECT / X UNINSTALL / B BACK";

}


function gameOptionsMenuItems() {

    if (gameOptionsMode === "confirm-uninstall") {
        return [
            {
                id:
                    "confirm-uninstall",
                label:
                    "UNINSTALL GAME"
            }
        ];
    }

    if (gameOptionsMode === "background-session") {
        return [
            {
                id:
                    "continue-game",
                label:
                    "CONTINUE GAME"
            },
            {
                id:
                    "close-game",
                label:
                    "CLOSE GAME"
            },
            {
                id:
                    "back",
                label:
                    "BACK"
            }
        ];
    }

    if (gameOptionsMode === "another-game-running") {
        return [
            {
                id:
                    "close-and-play",
                label:
                    "CLOSE & PLAY"
            },
            {
                id:
                    "cancel",
                label:
                    "CANCEL"
            }
        ];
    }

    if (gameOptionsMode !== "menu") {
        return [];
    }

    return [
        {
            id:
                "play",
            label:
                "PLAY GAME"
        },
        {
            id:
                "uninstall",
            label:
                "UNINSTALL GAME"
        }
    ];

}


function renderGameOptionsActions() {

    if (!gameOptionsActions) {
        return;
    }

    const items =
        gameOptionsMenuItems();

    if (!items.length) {
        gameOptionsActions.innerHTML =
            "";
        return;
    }

    if (selectedGameOptionIndex >= items.length) {
        selectedGameOptionIndex =
            items.length - 1;
    }

    gameOptionsActions.innerHTML =
        "";

    items.forEach(
        (item, index) => {

            const button =
                document.createElement("button");

            button.type =
                "button";

            button.className =
                "game-option-item";

            button.dataset.gameOption =
                item.id;

            button.classList.toggle(
                "selected",
                index === selectedGameOptionIndex
            );

            button.textContent =
                item.label;

            button.addEventListener(
                "click",
                () => {
                    selectedGameOptionIndex =
                        index;
                    renderGameOptions();
                }
            );

            gameOptionsActions.appendChild(button);

        }
    );

}


function navigateGameOptions(direction) {

    if (
        !gameOptionsOpen ||
        (
            gameOptionsMode !== "menu" &&
            gameOptionsMode !== "background-session" &&
            gameOptionsMode !== "another-game-running"
        )
    ) {
        return;
    }

    const items =
        gameOptionsMenuItems();

    if (!items.length) {
        return;
    }

    selectedGameOptionIndex += direction;

    if (selectedGameOptionIndex < 0) {
        selectedGameOptionIndex =
            items.length - 1;
    }

    if (selectedGameOptionIndex >= items.length) {
        selectedGameOptionIndex =
            0;
    }

    renderGameOptions();

}


async function activateGameOptions() {

    if (
        !gameOptionsOpen ||
        !selectedOptionsGame
    ) {
        return;
    }

    if (gameOptionsMode === "menu") {
        const selectedAction =
            gameOptionsMenuItems()[selectedGameOptionIndex]?.id ||
            "play";

        if (selectedAction === "uninstall") {
            requestGameUninstallConfirmation();
            return;
        }

        const gameIndex =
            installedGames.findIndex(
                (game) =>
                    game.id === selectedOptionsGame.id
            );

        if (gameIndex >= 0) {
            selectedGameIndex =
                gameIndex;
        }

        closeGameOptions();
        await launchSelectedGame();
        return;
    }

    if (gameOptionsMode === "background-session") {
        const selectedAction =
            gameOptionsMenuItems()[selectedGameOptionIndex]?.id ||
            "continue-game";

        if (selectedAction === "continue-game") {
            closeGameOptions();
            await continueActiveGameSession();
            return;
        }

        if (selectedAction === "close-game") {
            await closeActiveGameSessionAndReturnTo(ONA_STATE.GAME_LIBRARY);
            closeGameOptions();
            return;
        }

        closeGameOptions();
        return;
    }

    if (gameOptionsMode === "another-game-running") {
        const selectedAction =
            gameOptionsMenuItems()[selectedGameOptionIndex]?.id ||
            "cancel";

        if (selectedAction === "close-and-play") {
            const gameToLaunch =
                pendingPlayAfterCloseGame;
            closeGameOptions();
            await closeActiveGameSessionAndReturnTo(ONA_STATE.GAME_LIBRARY);
            pendingPlayAfterCloseGame =
                null;

            if (gameToLaunch?.id) {
                selectedGameIndex =
                    installedGames.findIndex(
                        (game) =>
                            game.id === gameToLaunch.id
                    );
                await launchSelectedGame();
            }
            return;
        }

        pendingPlayAfterCloseGame =
            null;
        closeGameOptions();
        return;
    }

    if (gameOptionsMode === "confirm-uninstall") {
        await uninstallSelectedOptionsGame();
        return;
    }

    if (gameOptionsMode === "error") {
        showGameOptionsDetails();
        return;
    }

}


function requestGameUninstallConfirmation() {

    if (
        !gameOptionsOpen ||
        !selectedOptionsGame ||
        gameOptionsMode !== "menu" ||
        uiBusy()
    ) {
        return;
    }

    gameOptionsMode =
        "confirm-uninstall";

    selectedGameOptionIndex =
        0;

    renderGameOptions();

}


function backFromGameOptions() {

    if (!gameOptionsOpen) {
        return;
    }

    if (
        gameOptionsMode === "confirm-uninstall" ||
        gameOptionsMode === "another-game-running" ||
        gameOptionsMode === "error"
    ) {
        gameOptionsMode =
            "menu";
        selectedGameOptionIndex =
            0;
        renderGameOptions();
        return;
    }

    if (gameOptionsMode === "background-session") {
        closeGameOptions();
        return;
    }

    closeGameOptions();

}


async function uninstallSelectedOptionsGame() {

    const game =
        selectedOptionsGame;

    if (!game) {
        return;
    }

    if (uiBusy()) {
        return;
    }

    setUiOperation("uninstalling");

    gameOptionsMode =
        "uninstalling";

    renderGameOptions();

    try {

        await invoke(
            "uninstall_installed_game",
            {
                gameId:
                    game.id
            }
        );

        gameOptionsMode =
            "uninstalled";

        renderGameOptions();

        if (libraryStatus) {
            libraryStatus.textContent =
                `${game.name} uninstalled.`;
        }

        await loadGameLibrary();

        setTimeout(
            () => {
                closeGameOptions();
                renderGameLibrary();
                setUiOperation("idle");
            },
            700
        );

    }

    catch (error) {

        console.error(
            "[ONA Library] Uninstall failed:",
            error
        );

        gameOptionsMode =
            "error";
        setUiOperation("idle");

        renderGameOptions();

        if (gameOptionsDetails) {
            gameOptionsDetails.hidden =
                false;
            gameOptionsDetails.textContent =
                String(error) === "GAME_IS_CURRENTLY_RUNNING"
                    ? "Close the game before uninstalling."
                    : `Developer details: ${error}`;
        }

    }

}


function showGameOptionsDetails() {

    if (!gameOptionsDetails) {
        return;
    }

    gameOptionsDetails.hidden =
        false;

    if (!gameOptionsDetails.textContent) {
        gameOptionsDetails.textContent =
            "Developer details: no additional details.";
    }

}


// =========================================================
// CONTROLLER LAB DATA
// =========================================================

function renderControllerLabButtons() {

    if (!labButtons) {
        return;
    }

    labButtons.innerHTML =
        "";

    ONA_CONTROLLER_BUTTONS.forEach(
        (button) => {

            const buttonElement =
                document.createElement("div");

            buttonElement.className =
                "lab-button";

            buttonElement.dataset.button =
                button;

            buttonElement.innerHTML =
                `<span>${button}</span><strong>RELEASED</strong>`;

            labButtons.appendChild(buttonElement);

        }
    );

}


async function loadControllerProfile() {

    try {

        controllerProfile =
            await invoke(
                "load_controller_profile",
                {
                    playerId:
                        selectedPlayer
                }
            );

    }

    catch (error) {

        console.error(
            "[Controller Lab] Profile load failed:",
            error
        );

    }

    updateControllerLabProfile();

}


async function saveControllerProfile() {

    try {

        controllerProfile =
            await invoke(
                "save_controller_profile",
                {
                    playerId:
                        selectedPlayer,
                    profile:
                        controllerProfile
                }
            );

        if (labSaveStatus) {
            labSaveStatus.textContent =
                "Profile saved.";
        }

    }

    catch (error) {

        console.error(
            "[Controller Lab] Profile save failed:",
            error
        );

        if (labSaveStatus) {
            labSaveStatus.textContent =
                `Save failed: ${error}`;
        }

    }

    updateControllerLabProfile();

}


function updateControllersScreen() {

    if (controllersStatus) {
        controllersStatus.textContent =
            playerCountValue > 0
                ? "CONTROLLER 1 / CONNECTED"
                : "WAITING FOR CONTROLLER";
    }

    if (controllersPlayerName) {
        controllersPlayerName.textContent =
            currentProfile.name;
    }

}


function ensureControllersFocus() {

    const actions =
        Array.from(
            controllersScreen?.querySelectorAll(
                ".controller-action:not(.disabled)"
            ) || []
        );

    if (!actions.length) {
        return;
    }

    if (
        !actions.some(
            (action) =>
                action.classList.contains("selected")
        )
    ) {
        actions[0]
            .classList
            .add("selected");
    }

}


function selectedControllerAction() {

    return controllersScreen
        ?.querySelector(".controller-action.selected")
        ?.dataset
        ?.controllerAction;

}


function activateSelectedControllerAction() {

    const action =
        selectedControllerAction();

    if (action === "profile") {
        transitionTo(
            ONA_STATE.PROFILE_SELECT
        );
        return;
    }

    if (action === "add") {
        showAdditionalControllerQr();
        return;
    }

    if (action === "calibrate") {
        openControllerLab();
    }

}


controllersScreen
    ?.querySelectorAll(".controller-action")
    .forEach(
        (action) => {
            action.addEventListener(
                "click",
                () => {
                    controllersScreen
                        ?.querySelectorAll(".controller-action")
                        .forEach(
                            (item) =>
                                item.classList.remove("selected")
                        );

                    action.classList.add("selected");
                }
            );
        }
    );


function openControllerLab() {

    loadControllerProfile();

    transitionTo(
        ONA_STATE.CONTROLLER_LAB
    );

}


function updateControllerLabProfile() {

    if (labProfileName) {
        labProfileName.textContent =
            controllerProfile.name || "Default";
    }

    if (labDeadzone) {
        labDeadzone.value =
            String(controllerProfile.stick?.deadzone ?? 0.12);
    }

    if (labDeadzoneValue) {
        labDeadzoneValue.textContent =
            Number(controllerProfile.stick?.deadzone ?? 0.12).toFixed(2);
    }

    if (labSensitivity) {
        labSensitivity.value =
            String(controllerProfile.stick?.sensitivity ?? 1);
    }

    if (labSensitivityValue) {
        labSensitivityValue.textContent =
            Number(controllerProfile.stick?.sensitivity ?? 1).toFixed(2);
    }

    updateControllerLabBridgeStatus();

}


function updateControllerLabConnection() {

    const connected =
        playerCountValue > 0;

    labConnectionDot
        ?.classList
        .toggle("connected", connected);

    if (labConnectionText) {
        labConnectionText.textContent =
            connected
                ? "CONNECTED"
                : "WAITING";
    }

}


async function updateControllerLabBridgeStatus() {

    if (!labBridgeStatus) {
        return;
    }

    const stick =
        controllerProfile.stick || {};

    labBridgeStatus.textContent =
        `${Number(stick.centerX || 0).toFixed(2)} / ${Number(stick.centerY || 0).toFixed(2)}`;

}


function updateControllerLabStick(x, y) {

    lastRawStick = {
        x,
        y
    };

    const stick =
        controllerProfile.stick || {};

    const centerX =
        Number(stick.centerX || 0);

    const centerY =
        Number(stick.centerY || 0);

    const deadzone =
        Number(stick.deadzone ?? 0.12);

    const sensitivity =
        Number(stick.sensitivity ?? 1);

    let calibratedX =
        x - centerX;

    let calibratedY =
        y - centerY;

    const distance =
        Math.hypot(
            calibratedX,
            calibratedY
        );

    if (distance <= deadzone) {
        calibratedX = 0;
        calibratedY = 0;
    }

    else {

        const range =
            1 - deadzone;

        const scaledDistance =
            Math.min(
                1,
                Math.max(
                    0,
                    (distance - deadzone) / range
                )
            );

        const scale =
            distance > 0
                ? scaledDistance / distance
                : 0;

        calibratedX =
            Math.max(
                -1,
                Math.min(
                    1,
                    calibratedX * scale * sensitivity
                )
            );

        calibratedY =
            Math.max(
                -1,
                Math.min(
                    1,
                    calibratedY * scale * sensitivity
                )
            );

    }

    if (labStickX) {
        labStickX.textContent =
            calibratedX.toFixed(2);
    }

    if (labStickY) {
        labStickY.textContent =
            calibratedY.toFixed(2);
    }

    if (labRawStickX) {
        labRawStickX.textContent =
            x.toFixed(2);
    }

    if (labRawStickY) {
        labRawStickY.textContent =
            y.toFixed(2);
    }

    if (labStickDot) {
        labStickDot.style.transform =
            `translate(calc(-50% + ${calibratedX * 72}px), calc(-50% + ${calibratedY * 72}px))`;
    }

    if (labLastInput) {
        labLastInput.textContent =
            `JOYSTICK ${calibratedX.toFixed(2)} / ${calibratedY.toFixed(2)}`;
    }

}


function updateControllerLabButton(button, state) {

    const labButton =
        labButtons?.querySelector(
            `[data-button="${button}"]`
        );

    if (!labButton) {
        return;
    }

    const pressed =
        state === "down" ||
        state === "pressed";

    labButton
        .classList
        .toggle("pressed", pressed);

    const stateElement =
        labButton.querySelector("strong");

    if (stateElement) {
        stateElement.textContent =
            pressed
                ? "PRESSED"
                : "RELEASED";
    }

    if (labLastInput) {
        labLastInput.textContent =
            `${button} ${pressed ? "DOWN" : "UP"}`;
    }

}


function setControllerLabCenter() {

    controllerProfile.stick = {
        ...(controllerProfile.stick || {}),
        centerX:
            lastRawStick.x,
        centerY:
            lastRawStick.y
    };

    if (labSaveStatus) {
        labSaveStatus.textContent =
            "Stick center updated.";
    }

    updateControllerLabStick(
        lastRawStick.x,
        lastRawStick.y
    );

    updateControllerLabBridgeStatus();

}


function setControllerLabDeadzone(value) {

    controllerProfile.stick = {
        ...(controllerProfile.stick || {}),
        deadzone:
            Number(value)
    };

    updateControllerLabProfile();

    updateControllerLabStick(
        lastRawStick.x,
        lastRawStick.y
    );

}


function setControllerLabSensitivity(value) {

    controllerProfile.stick = {
        ...(controllerProfile.stick || {}),
        sensitivity:
            Number(value)
    };

    updateControllerLabProfile();

    updateControllerLabStick(
        lastRawStick.x,
        lastRawStick.y
    );

}


function escapeHtml(value) {

    return String(value)
        .replaceAll("&", "&amp;")
        .replaceAll("<", "&lt;")
        .replaceAll(">", "&gt;")
        .replaceAll('"', "&quot;")
        .replaceAll("'", "&#039;");

}


function escapeAttribute(value) {

    return escapeHtml(value);

}


// =========================================================
// SHOW SHELL
// =========================================================

function showShell() {

    console.log(
        "Showing ONA Shell..."
    );


    shell
        ?.classList
        .add("shell-visible");


    bootScreen
        ?.classList
        .add("boot-hidden");


    transitionTo(
        ONA_STATE.WAITING_CONTROLLER
    );

}


// =========================================================
// CONTROLLER CONNECTED
// =========================================================

function controllerConnected(
    controllerInfo = {}
) {

    const playerId =
        Number(controllerInfo.playerId || 1);

    if (addingController) {
        playerCountValue =
            Math.max(
                playerCountValue,
                playerId
            );
    }

    else {
        playerCountValue =
            Math.max(
                playerCountValue,
                playerId
            );
    }


    if (playerCount) {

        playerCount.textContent =
            `${playerCountValue} / 10`;

    }


    connectionDot
        ?.classList
        .add("connected");


    if (connectionText) {

        connectionText.textContent =
            `PLAYER ${playerId} CONNECTED`;

    }


    if (controllerStatus) {

        controllerStatus.textContent =
            controllerInfo.name
                ? `${controllerInfo.name} connected.`
                : "Controller connected.";

    }


    console.log(
        "ONA Controller connected:",
        controllerInfo
    );

    updateControllerLabConnection();
    updateControllersScreen();

    if (addingController) {

        addingController =
            false;

        transitionTo(
            ONA_STATE.CONTROLLERS
        );

        return;

    }

    setTimeout(
        () => {

            transitionTo(
                ONA_STATE.PROFILE_SELECT
            );

        },
        700
    );

}


// =========================================================
// PROFILE CONTROLLER STATUS
// =========================================================

function updateProfileControllerStatus() {

    if (!profileControllerText) {
        return;
    }


    if (playerCountValue > 0) {

        profileControllerText.textContent =
            "CONTROLLER CONNECTED";

    }

    else {

        profileControllerText.textContent =
            "WAITING FOR CONTROLLER";

    }

}


// =========================================================
// PROFILE SELECTION VISUAL
// =========================================================

function updateProfileSelection() {

    if (!profileGrid) {
        return;
    }


    const cards =
        Array.from(
            profileGrid.querySelectorAll(
            ".profile-card"
            )
        );

    if (
        !cards.some(
            (card) =>
                card.classList.contains("selected")
        )
    ) {

        const selectedCard =
            cards.find(
                (card) =>
                    Number(card.dataset.player) === selectedPlayer
            );

        selectedCard
            ?.classList
            .add("selected");

    }

    const selectedIndex =
        cards.findIndex(
            (card) =>
                card.classList.contains("selected")
        );

    cards.forEach(
        (card, index) => {
            card.classList.remove(
                "profile-card-previous",
                "profile-card-next",
                "profile-card-hidden"
            );

            if (
                selectedIndex < 0 ||
                cards.length < 2 ||
                index === selectedIndex
            ) {
                return;
            }

            const previousIndex =
                (
                    selectedIndex -
                    1 +
                    cards.length
                ) %
                cards.length;

            const nextIndex =
                (
                    selectedIndex +
                    1
                ) %
                cards.length;

            if (index === previousIndex) {
                card.classList.add("profile-card-previous");
                return;
            }

            if (index === nextIndex) {
                card.classList.add("profile-card-next");
                return;
            }

            card.classList.add("profile-card-hidden");
        }
    );


    if (profilePlayerNumber) {

        profilePlayerNumber.textContent =
            `PLAYER ${selectedPlayer}`;

    }

}


// =========================================================
// SELECT PROFILE
// =========================================================

function selectProfile(card) {

    if (!card) {
        return;
    }


    const profile =
        card.dataset.profile;


    if (profile === "add") {

        profileGrid
            ?.querySelectorAll(".profile-card")
            .forEach(
                (card) =>
                    card.classList.remove("selected")
            );

        card.classList.add("selected");

        console.log(
            "Add profile selected"
        );

        return;

    }


    const player =
        Number(
            card.dataset.player
        );


    if (!player) {
        return;
    }


    selectedPlayer =
        player;

    profileGrid
        ?.querySelectorAll(".profile-card")
        .forEach(
            (profileCard) =>
                profileCard.classList.remove("selected")
        );

    card.classList.add("selected");


    currentProfile = {

        player:
            player,

        id:
            profile || "guest",

        name:
            `PLAYER ${player}`,

        type:
            profile === "guest"
                ? "GUEST"
                : "PROFILE"

    };


    updateProfileSelection();


    console.log(
        "ONA profile selected:",
        currentProfile
    );


    setTimeout(
        () => {

            transitionTo(
                ONA_STATE.MAIN_MENU
            );

        },
        300
    );

}


// =========================================================
// PROFILE CLICK
// =========================================================

if (profileGrid) {

    profileGrid.addEventListener(
        "click",
        (event) => {

            const card =
                event.target.closest(
                    ".profile-card"
                );


            if (!card) {
                return;
            }


            selectProfile(card);

        }
    );

}

importGameButton?.addEventListener(
    "click",
    importLocalGame
);

labDeadzone?.addEventListener(
    "input",
    (event) => setControllerLabDeadzone(event.target.value)
);

labSensitivity?.addEventListener(
    "input",
    (event) => setControllerLabSensitivity(event.target.value)
);

labCenterButton?.addEventListener(
    "click",
    setControllerLabCenter
);

labSaveProfileButton?.addEventListener(
    "click",
    saveControllerProfile
);

calibrateControllerButton?.addEventListener(
    "click",
    openControllerLab
);

changeProfileButton?.addEventListener(
    "click",
    () => transitionTo(ONA_STATE.PROFILE_SELECT)
);

addControllerButton?.addEventListener(
    "click",
    showAdditionalControllerQr
);


// =========================================================
// CURRENT PROFILE
// =========================================================

function updateCurrentProfile() {

    if (currentProfileAvatar) {

        currentProfileAvatar.textContent =
            `P${currentProfile.player}`;

    }


    if (currentProfileName) {

        currentProfileName.textContent =
            currentProfile.name;

    }

}


// =========================================================
// KEYBOARD PROFILE NAVIGATION
// =========================================================

function navigateProfiles(direction) {

    if (
        currentState !==
        ONA_STATE.PROFILE_SELECT
    ) {

        return;

    }


    const cards =
        Array.from(
            profileGrid?.querySelectorAll(
                ".profile-card"
            ) || []
        );


    if (!cards.length) {
        return;
    }


    let currentIndex =
        cards.findIndex(
            (card) =>
                card.classList.contains("selected")
        );


    if (currentIndex < 0) {

        currentIndex = 0;

    }


    currentIndex += direction;


    if (
        currentIndex < 0
    ) {

            currentIndex =
            cards.length - 1;

    }


    if (
        currentIndex >=
        cards.length
    ) {

        currentIndex = 0;

    }


    const card =
        cards[currentIndex];


    const player =
        Number(
            card.dataset.player
        );

    if (player) {
        selectedPlayer =
            player;
    }

    cards.forEach(
        (profileCard) =>
            profileCard.classList.remove("selected")
    );

    card.classList.add("selected");


    updateProfileSelection();

}


// =========================================================
// KEYBOARD INPUT
// =========================================================

window.addEventListener(
    "keydown",
    (event) => {

        if (onaExitConfirmOpen) {

            if (
                event.key === "Enter" ||
                event.key === " "
            ) {
                event.preventDefault();
                onaExitConfirmOpen =
                    false;
                closeOnaOwnedRuntimeAndExit();
                return;
            }

            if (
                event.key === "Backspace" ||
                event.key === "Escape"
            ) {
                event.preventDefault();
                cancelOnaExit();
                return;
            }

            return;

        }

        if (importBrowserOpen) {

            switch (event.key) {

                case "ArrowLeft":

                case "ArrowUp":

                    event.preventDefault();
                    navigateInstallPackages(-1);
                    break;

                case "ArrowRight":

                case "ArrowDown":

                    event.preventDefault();
                    navigateInstallPackages(1);
                    break;

                case "Enter":

                case " ":

                    event.preventDefault();
                    activateImportBrowser();
                    break;

                case "Backspace":

                case "Escape":

                    event.preventDefault();
                    backFromImportBrowser();
                    break;

            }

            return;

        }

        if (gameOptionsOpen) {

            switch (event.key) {

                case "ArrowLeft":

                case "ArrowUp":

                    event.preventDefault();
                    navigateGameOptions(-1);
                    break;

                case "ArrowRight":

                case "ArrowDown":

                    event.preventDefault();
                    navigateGameOptions(1);
                    break;

                case "Enter":

                case " ":

                    event.preventDefault();
                    activateGameOptions();
                    break;

                case "x":

                case "X":

                    event.preventDefault();
                    requestGameUninstallConfirmation();
                    break;

                case "Backspace":

                case "Escape":

                    event.preventDefault();
                    backFromGameOptions();
                    break;

            }

            return;

        }

        if (
            currentState ===
            ONA_STATE.PROFILE_SELECT
        ) {

            switch (event.key) {

                case "ArrowLeft":

                case "ArrowUp":

                    event.preventDefault();

                    navigateProfiles(-1);

                    break;


                case "ArrowRight":

                case "ArrowDown":

                    event.preventDefault();

                    navigateProfiles(1);

                    break;


                case "Enter":

                case " ":

                    event.preventDefault();


                    const card =
                        profileGrid?.querySelector(
                            ".profile-card.selected"
                        );


                    if (card) {

                        selectProfile(card);

                    }

                    break;

            }

        }

        if (
            currentState ===
            ONA_STATE.GAME_LIBRARY
        ) {

            switch (event.key) {

                case "ArrowLeft":

                case "ArrowUp":

                    event.preventDefault();

                    navigateGames(-1);

                    break;


                case "ArrowRight":

                case "ArrowDown":

                    event.preventDefault();

                    navigateGames(1);

                    break;


                case "Enter":

                case " ":

                    event.preventDefault();

                    if (
                        installedGames.length &&
                        selectedGameIndex < installedGames.length
                    ) {
                        openGameOptions();
                    }
                    else {
                        importGameButton?.click();
                    }

                    break;

                case "x":

                case "X":

                    event.preventDefault();

                    if (
                        installedGames.length &&
                        selectedGameIndex < installedGames.length
                    ) {
                        openGameOptions();
                    }

                    break;


                case "Backspace":

                    event.preventDefault();

                    transitionTo(
                        ONA_STATE.MAIN_MENU
                    );

                    break;

            }

        }

        if (
            currentState ===
            ONA_STATE.STORE
        ) {

            switch (event.key) {

                case "ArrowLeft":

                case "ArrowUp":

                    event.preventDefault();

                    moveSelectedElement(
                        storeScreen,
                        ".store-item",
                        -1
                    );

                    break;


                case "ArrowRight":

                case "ArrowDown":

                    event.preventDefault();

                    moveSelectedElement(
                        storeScreen,
                        ".store-item",
                        1
                    );

                    break;


                case "Backspace":

                    event.preventDefault();

                    transitionTo(
                        ONA_STATE.MAIN_MENU
                    );

                    break;

            }

        }

        if (
            currentState ===
            ONA_STATE.SETTINGS
        ) {

            switch (event.key) {

                case "ArrowLeft":

                    event.preventDefault();
                    moveSettingsSelection("left");
                    break;

                case "ArrowRight":

                    event.preventDefault();
                    moveSettingsSelection("right");
                    break;

                case "ArrowUp":

                    event.preventDefault();
                    moveSettingsSelection("up");
                    break;

                case "ArrowDown":

                    event.preventDefault();
                    moveSettingsSelection("down");
                    break;

                case "Enter":

                case " ":

                    event.preventDefault();
                    activateSettingsSelection();
                    break;

                case "Backspace":

                    event.preventDefault();

                    if (settingsFocusLevel === "option") {
                        settingsFocusLevel = "category";
                        settingsPanel
                            ?.querySelectorAll(".settings-option")
                            .forEach((option) => option.classList.remove("selected"));
                    }
                    else {
                        transitionTo(ONA_STATE.MAIN_MENU);
                    }

                    break;

            }

        }

        if (
            currentState ===
            ONA_STATE.CONTROLLERS
        ) {

            switch (event.key) {

                case "ArrowLeft":

                case "ArrowUp":

                    event.preventDefault();

                    ensureControllersFocus();
                    moveSelectedElement(
                        controllersScreen,
                        ".controller-action:not(.disabled)",
                        -1
                    );

                    break;


                case "ArrowRight":

                case "ArrowDown":

                    event.preventDefault();

                    ensureControllersFocus();
                    moveSelectedElement(
                        controllersScreen,
                        ".controller-action:not(.disabled)",
                        1
                    );

                    break;


                case "Enter":

                case " ":

                    event.preventDefault();

                    ensureControllersFocus();
                    activateSelectedControllerAction();

                    break;

                case "Backspace":

                    event.preventDefault();

                    transitionTo(
                        ONA_STATE.MAIN_MENU
                    );

                    break;

            }

        }

        if (
            currentState ===
            ONA_STATE.CONTROLLER_LAB
        ) {

            switch (event.key) {

                case "Backspace":

                    event.preventDefault();

                    transitionTo(
                        ONA_STATE.CONTROLLERS
                    );

                    break;

                case "Enter":

                case " ":

                    event.preventDefault();

                    saveControllerProfile();

                    break;

            }

        }

        if (
            currentState ===
            ONA_STATE.QUICK_MENU
        ) {

            switch (event.key) {

                case "ArrowUp":

                    event.preventDefault();

                    navigateSystemMenu(-1);

                    break;

                case "ArrowDown":

                    event.preventDefault();

                    navigateSystemMenu(1);

                    break;

                case "Enter":

                case " ":

                    event.preventDefault();

                    activateSelectedSystemMenuItem();

                    break;

            }

        }

    }
);


// =========================================================
// VIDEO — METADATA
// =========================================================

if (intro) {

    intro.addEventListener(
        "loadedmetadata",
        () => {

            console.log(
                "ONA intro metadata loaded"
            );


            console.log(
                "Duration:",
                intro.duration
            );


            console.log(
                "Resolution:",
                intro.videoWidth,
                "x",
                intro.videoHeight
            );

        }
    );


    intro.addEventListener(
        "playing",
        () => {

            console.log(
                "ONA intro playing"
            );

        }
    );


    intro.addEventListener(
        "ended",
        () => {

            console.log(
                "ONA intro finished"
            );


            showShell();

        }
    );


    intro.addEventListener(
        "error",
        () => {

            console.error(
                "ONA intro ERROR"
            );


            if (intro.error) {

                console.error(
                    "Video error code:",
                    intro.error.code
                );


                console.error(
                    "Video error message:",
                    intro.error.message
                );

            }


            showShell();

        }
    );

}


// =========================================================
// START INTRO
// =========================================================

function startIntro() {

    if (!intro) {

        showShell();

        return;

    }


    console.log(
        "Attempting to start ONA intro..."
    );


    intro.currentTime = 0;

    intro.muted = false;


    const promise =
        intro.play();


    if (!promise) {

        return;

    }


    promise

        .then(
            () => {

                console.log(
                    "ONA intro started with audio."
                );

            }
        )

        .catch(
            (error) => {

                console.warn(
                    "Audio autoplay blocked."
                );


                console.warn(
                    error
                );


                intro.muted = true;


                intro.play()

                    .then(
                        () => {

                            console.log(
                                "ONA intro started muted."
                            );

                        }
                    )

                    .catch(
                        (playError) => {

                            console.error(
                                "Video playback failed:",
                                playError
                            );


                            showShell();

                        }
                    );

            }
        );

}


// =========================================================
// AUDIO UNLOCK
// =========================================================

function unlockAudio() {

    if (!intro) {
        return;
    }


    if (!intro.muted) {
        return;
    }


    intro.muted = false;


    intro.play()

        .then(
            () => {

                console.log(
                    "ONA audio unlocked."
                );

            }
        )

        .catch(
            () => {

                console.warn(
                    "ONA audio unlock failed."
                );

            }
        );

}


window.addEventListener(
    "pointerdown",
    unlockAudio,
    {
        once: true
    }
);


// =========================================================
// FALLBACK DE SEGURIDAD
// =========================================================

setTimeout(
    () => {

        if (
            currentState ===
            ONA_STATE.BOOT
        ) {

            console.warn(
                "ONA intro fallback activated."
            );


            showShell();

        }

    },
    10000
);


// =========================================================
// ESC — SYSTEM MENU
// =========================================================

window.addEventListener(
    "keydown",
    async (event) => {

        if (
            event.key !== "Escape"
        ) {

            return;

        }


        event.preventDefault();
        console.log("[ONA Keyboard] ESC dom down");
        await handleEscapeSystemCommand("dom");

    }
);


async function handleEscapeSystemCommand(source = "dom") {

    const escStatus =
        await invoke("escape_system_shortcut_status").catch(() => null);
    await logPostRestoreState("esc");

    const now =
        performance.now();
    if (now - lastEscapeSystemCommandAt < 120) {
        console.log(
            `[ONA Keyboard] ESC duplicate ignored source=${source}`
        );
        console.log(
            `[ONA ESC TRACE] eventReceived=true registered=${Boolean(escStatus?.registered)} generation=${escStatus?.generation ?? "unknown"} owner=${presentationOwner} systemMenuState=${systemMenuRequestState} requestSystemMenuCalled=false rejectReason=duplicate_event`
        );
        return;
    }
    lastEscapeSystemCommandAt =
        now;

    console.log(
        `[ONA Keyboard] owner=${presentationOwner} source=${source}`
    );

    if (
        source === "dom" &&
        presentationOwner === PresentationOwner.GAME
    ) {
        console.log("[ONA ESC] DOM ignored while GAME owns presentation");
        return;
    }

    if (
        presentationOwner === PresentationOwner.ONA_TRANSITION_GUARD ||
        presentationOwner === PresentationOwner.ONA_MINIMIZED
    ) {
        console.log("[ONA Keyboard] ESC ignored");
        console.log(
            `[ONA ESC TRACE] eventReceived=true registered=${Boolean(escStatus?.registered)} generation=${escStatus?.generation ?? "unknown"} owner=${presentationOwner} systemMenuState=${systemMenuRequestState} requestSystemMenuCalled=false rejectReason=owner_${presentationOwner}`
        );
        return;
    }

    if (
        currentState === ONA_STATE.QUICK_MENU ||
        presentationOwner === PresentationOwner.ONA_SYSTEM_OVERLAY
    ) {
        console.log(
            `[ONA ESC TRACE] eventReceived=true registered=${Boolean(escStatus?.registered)} generation=${escStatus?.generation ?? "unknown"} owner=${presentationOwner} systemMenuState=${systemMenuRequestState} requestSystemMenuCalled=true rejectReason=none`
        );
        await requestSystemMenu(`ESC_${source.toUpperCase()}`);
        return;
    }

    console.log(
        `[ONA ESC TRACE] eventReceived=true registered=${Boolean(escStatus?.registered)} generation=${escStatus?.generation ?? "unknown"} owner=${presentationOwner} systemMenuState=${systemMenuRequestState} requestSystemMenuCalled=true rejectReason=none`
    );
    await requestSystemMenu(`ESC_${source.toUpperCase()}`);

}


async function requestSystemMenu(source = "SYSTEM") {

    normalizeSystemMenuState(`request_${source}`);

    if (
        restoringConsoleExperience ||
        presentationOwner === PresentationOwner.ONA_MINIMIZED
    ) {
        console.log(
            `[ONA SYSTEM] request source=${source} rejected reason=restore_in_progress`
        );
        return;
    }

    if (!VALID_SYSTEM_MENU_SOURCES.has(source)) {
        console.warn(
            `[ONA SYSTEM] request source=${source} rejected reason=invalid_source`
        );
        return;
    }

    console.log(
        `[ONA SYSTEM] request source=${source} state=${systemMenuRequestState}`
    );

    if (
        systemMenuRequestState === SystemMenuRequestState.OPEN &&
        (
            source === "ESC_NATIVE" ||
            source === "ESC_DOM" ||
            source === "HOLD_START" ||
            source === "START_OVERLAY"
        )
    ) {
        console.log(
            `[ONA SYSTEM] request source=${source} accepted action=close`
        );
        await closeSystemMenu(
            stateBeforeSystemMenu,
            {
                deferGameInputUntilStartRelease:
                    source === "HOLD_START"
            }
        );
        return;
    }

    if (systemMenuRequestState !== SystemMenuRequestState.CLOSED) {
        console.log(
            `[ONA SYSTEM] request source=${source} rejected reason=state_${systemMenuRequestState}`
        );
        return;
    }

    console.log(
        `[ONA SYSTEM] request source=${source} accepted`
    );
    setSystemMenuRequestState(
        SystemMenuRequestState.OPENING,
        source
    );

    try {
        console.log("[ONA Keyboard] Quick Menu requested");

        if (presentationOwner === PresentationOwner.GAME || nativeGameSessionActive()) {
            await openSystemMenuDuringGame();
            console.log("[ONA Keyboard] Quick Menu opened");
            return;
        }

        openSystemMenu();
        console.log("[ONA Keyboard] Quick Menu opened");
    }
    finally {
        setSystemMenuRequestState(
            systemMenu?.classList.contains("visible")
                ? SystemMenuRequestState.OPEN
                : SystemMenuRequestState.CLOSED,
            `request_${source}_finally`
        );

        if (systemMenuRequestState === SystemMenuRequestState.OPEN) {
            console.log("[ONA SYSTEM] menu OPEN");
        }
    }

}


// =========================================================
// SYSTEM MENU
// =========================================================

function openSystemMenu(context = {}) {

    console.log(
        "Opening ONA system menu",
        context
    );


    stateBeforeSystemMenu =
        context.returnState || currentState;

    currentState =
        ONA_STATE.QUICK_MENU;


    systemMenu
        ?.classList
        .add("visible");


    document.body
        .classList
        .add("system-menu-open");


    document.body
        .classList
        .add("mouse-enabled");

    document.body.classList.toggle(
        "game-system-overlay",
        Boolean(context.gameOverlay)
    );


    systemMenu
        ?.querySelectorAll(".system-menu-item")
        .forEach(
            (item) =>
                item.classList.remove("selected")
        );

    resumeButton
        ?.classList
        .add("selected");

    resumeButton?.focus();
    setSystemMenuRequestState(
        SystemMenuRequestState.OPEN,
        "openSystemMenu"
    );

}


async function openSystemMenuDuringGame() {

    if (!nativeGameSessionActive()) {
        openSystemMenu();
        return;
    }

    const transitionId =
        beginPresentationTransition("QUICK_MENU_OPEN");

    console.log(
        "[ONA QuickMenu] Entered from running game"
    );

    try {
        await requirePresentationGuard(
            "RETURNING_TO_ONA",
            "QUICK_MENU_OPEN"
        );

        if (!presentationTransitionCurrent(transitionId)) {
            return;
        }

        clearGameLifecycleOverlay();
        openSystemMenu({
            returnState:
                ONA_STATE.GAME_RUNNING,
            gameOverlay:
                true
        });
        await setGameInputRouting(false);

        if (!presentationTransitionCurrent(transitionId)) {
            return;
        }

        await invoke(
            "show_system_overlay_over_game"
        );
    }
    catch (error) {
        console.error(
            "[ONA Quick Menu] Could not show ONA overlay over game:",
            error
        );
        endPresentationTransition(transitionId);
        throw error;
    }

    if (!presentationTransitionCurrent(transitionId)) {
        return;
    }

    setUiOperation("running");
    setConsolePresentationState(
        ConsolePresentationState.RUNNING
    );
    setGameSessionState(
        GameSessionState.SYSTEM_OVERLAY
    );
    updateActiveGameSession({
        state:
            GameSessionState.SYSTEM_OVERLAY,
        presentationState:
            ConsolePresentationState.RUNNING
    });
    setPresentationOwner(
        PresentationOwner.ONA_SYSTEM_OVERLAY
    );
    clearGameLifecycleOverlay();
    await invoke("release_presentation_guard");
    console.log("[ONA Presentation] guard -> ONA_SYSTEM_OVERLAY");
    endPresentationTransition(transitionId);

}


// =========================================================
// CLOSE SYSTEM MENU
// =========================================================

async function closeSystemMenu(targetState = stateBeforeSystemMenu, options = {}) {

    console.log(
        "Closing ONA system menu"
    );

    console.log("[ONA SYSTEM] menu CLOSE requested");
    setSystemMenuRequestState(
        SystemMenuRequestState.CLOSING,
        "closeSystemMenu"
    );

    if (targetState === ONA_STATE.GAME_RUNNING && runningGameId) {
        console.log(
            "[ONA QuickMenu] Returning directly to running game"
        );

        const transitionId =
            beginPresentationTransition("QUICK_MENU_CLOSE_TO_GAME");

        try {
            await requirePresentationGuard(
                "RESUMING_GAME",
                "QUICK_MENU_CLOSE_TO_GAME"
            );

            if (!presentationTransitionCurrent(transitionId)) {
                return;
            }
            await invoke(
                "hide_game_cursor"
            );
            if (activeGameSession?.pid) {
                const foregroundGranted =
                    await invoke(
                        "focus_running_game",
                        {
                            pid:
                                activeGameSession.pid
                        }
                    );
                console.log(
                    `[ONA GameWindow] Quick Menu return using PRIMARY hwnd=${activeGameSession.primaryHwnd || "captured-native"} foreground=${foregroundGranted}`
                );
                await invoke(
                    "suppress_running_game_taskbar_identity",
                    {
                        pid:
                            activeGameSession.pid
                    }
                ).catch(
                    (error) =>
                        console.warn("[ONA GameWindow] taskbar suppression after Quick Menu failed:", error)
                );
            }
            await invoke(
                "prepare_shell_for_game",
                {
                    reason:
                        "quick_menu_return"
                }
            );
            forceSystemMenuClosed("returned_to_game");
            await invoke("release_presentation_guard");
            console.log("[ONA Presentation] guard -> GAME");

            if (!presentationTransitionCurrent(transitionId)) {
                return;
            }

            currentState =
                ONA_STATE.GAME_RUNNING;
            setUiOperation("running");
            setConsolePresentationState(
                ConsolePresentationState.RUNNING
            );
            setGameSessionState(
                GameSessionState.RUNNING
            );
            clearGameLifecycleOverlay();

            presentationInvariantWarningsSuppressed =
                true;
            setPresentationOwner(
                PresentationOwner.GAME
            );
            updateActiveGameSession({
                state:
                    GameSessionState.RUNNING,
                presentationState:
                    ConsolePresentationState.RUNNING
            });
            console.log("[ONA SYSTEM] owner=GAME");

            if (
                options.deferGameInputUntilStartRelease &&
                startArbitrationState === StartArbitrationState.HOLD_CONSUMED
            ) {
                console.log("[ONA SYSTEM] game routing remains paused until HOLD release");
            }
            else {
                await setGameInputRouting(true);
                console.log("[ONA SYSTEM] game routing enabled");
            }
            presentationInvariantWarningsSuppressed =
                false;
            assertPresentationInvariants();
            endPresentationTransition(transitionId);
        }
        catch (error) {
            presentationInvariantWarningsSuppressed =
                false;
            console.error(
                "[ONA Quick Menu] Could not return presentation to game:",
                error
            );
            setSystemMenuRequestState(
                SystemMenuRequestState.OPEN,
                "close_failed"
            );
            systemMenu
                ?.classList
                .add("visible");
            document.body
                .classList
                .add("system-menu-open");
            document.body
                .classList
                .add("mouse-enabled");
            document.body
                .classList
                .add("game-system-overlay");
            setPresentationOwner(
                PresentationOwner.ONA_SYSTEM_OVERLAY
            );
            setGameSessionState(
                GameSessionState.SYSTEM_OVERLAY
            );
            updateActiveGameSession({
                state:
                    GameSessionState.SYSTEM_OVERLAY,
                presentationState:
                    ConsolePresentationState.RUNNING
            });
            endPresentationTransition(transitionId);
        }

        return;
    }

    forceSystemMenuClosed("closed_to_shell");
    transitionTo(targetState);

}


// =========================================================
// RESUME
// =========================================================

resumeButton?.addEventListener(
    "click",
    () => {

        closeSystemMenu();

    }
);

homeButton?.addEventListener(
    "click",
    () => {

        if (nativeGameSessionActive()) {
            backgroundActiveGameSessionAndReturnHome();
            return;
        }

        closeSystemMenu(
            ONA_STATE.HOME
        );

    }
);


async function backgroundActiveGameSessionAndReturnHome() {

    const game =
        activeSessionGame();

    console.log(
        "[ONA Session] HOME requested"
    );

    systemMenu
        ?.classList
        .remove("visible");

    document.body
        .classList
        .remove("system-menu-open");

    document.body
        .classList
        .remove("mouse-enabled");

    setGameSessionState(
        GameSessionState.BACKGROUND
    );
    showPresentationGuard(
        "RETURNING",
        game?.name || "GAME",
        "Returning to ONA Home."
    );
    console.log(
        "[ONA Presentation] GAME -> guard"
    );

    try {
        await requirePresentationGuard(
            "RETURNING_TO_ONA",
            "HOME_FROM_GAME"
        );
        await setGameInputRouting(false);
        await invoke(
            "restore_shell_after_game"
        );
        console.log(
            "[ONA Presentation] shell restored behind guard"
        );
    }
    catch (error) {
        console.error(
            "[ONA Session] Could not prepare ONA Home over game:",
            error
        );
        return;
    }

    updateActiveGameSession({
        state:
            GameSessionState.BACKGROUND,
        presentationState:
            ConsolePresentationState.IDLE
    });

    setUiOperation(
        "idle"
    );

    await loadGameLibrary();

    transitionTo(
        ONA_STATE.HOME
    );

    clearGameLifecycleOverlay();
    setConsolePresentationState(
        ConsolePresentationState.IDLE
    );
    setGameSessionState(
        GameSessionState.BACKGROUND
    );
    setPresentationOwner(
        PresentationOwner.ONA_SHELL
    );
    await invoke("release_presentation_guard").catch(
        (error) =>
            console.warn("[ONA Presentation] Could not release guard:", error)
    );
    console.log(
        "[ONA Presentation] guard -> ONA_SHELL"
    );

}


async function closeActiveGameSessionAndReturnTo(targetState = ONA_STATE.HOME) {

    const game =
        activeSessionGame();

    if (!activeGameSession?.pid && !runningGameId) {
        resetStartArbitration("GAME_SESSION_END_NO_PID");
        setActiveGameSession(null);
        transitionTo(targetState);
        return;
    }

    console.log(
        "[ONA Session] graceful shutdown requested"
    );

    showPresentationGuard(
        "RETURNING",
        game?.name || "GAME",
        "Closing game session."
    );
    try {
        await requirePresentationGuard(
            "RETURNING_TO_ONA",
            "CLOSE_GAME"
        );
    }
    catch (error) {
        console.error("[ONA Presentation] Guard failed before close:", error);
        return;
    }
    await setGameInputRouting(false);
    setGameSessionState(
        GameSessionState.STOPPING
    );

    try {
        const status =
            await invoke(
                "terminate_running_game"
            );

        console.log(
            "[ONA Session] process exited code=",
            status?.exitCode ?? "unknown"
        );
    }
    catch (error) {
        console.error(
            "[ONA Session] Game shutdown failed:",
            error
        );
    }

    if (runningGamePollTimer) {
        clearInterval(runningGamePollTimer);
        runningGamePollTimer =
            null;
    }

    resetStartArbitration("GAME_SESSION_END_CLOSE");
    setActiveGameSession(null);
    runningGameId =
        null;
    setUiOperation("idle");
    setGameSessionState(
        GameSessionState.IDLE
    );

    await loadGameLibrary();

    transitionTo(
        targetState
    );

    clearGameLifecycleOverlay();
    setConsolePresentationState(
        ConsolePresentationState.IDLE
    );
    setPresentationOwner(
        PresentationOwner.ONA_SHELL
    );
    await invoke("release_presentation_guard").catch(
        (error) =>
            console.warn("[ONA Presentation] Could not release guard:", error)
    );

}


async function continueActiveGameSession() {

    if (!activeGameSession?.pid) {
        resetStartArbitration("GAME_SESSION_END_NO_ACTIVE_CONTINUE");
        setActiveGameSession(null);
        await loadGameLibrary();
        return;
    }

    const game =
        activeSessionGame();
    const pidBefore =
        activeGameSession.pid;

    console.log(
        `[ONA Session] CONTINUE requested pid=${pidBefore}`
    );

    showPresentationGuard(
        "RETURNING",
        game?.name || "GAME",
        "Continuing game session."
    );

    try {
        await requirePresentationGuard(
            "RESUMING_GAME",
            "CONTINUE_GAME"
        );
    }
    catch (error) {
        console.error(
            "[ONA Presentation] Guard could not be raised before continue:",
            error
        );
        return;
    }

    const status =
        await invoke(
            "running_game_status"
        );

    if (
        status.state !== "running" ||
        status.pid !== pidBefore ||
        status.gameId !== activeGameSession.gameId
    ) {
        console.warn(
            "[ONA Session] Background session is stale:",
            status
        );
        resetStartArbitration("GAME_SESSION_END_STALE_CONTINUE");
        setActiveGameSession(null);
        runningGameId =
            null;
        await setGameInputRouting(false);
        await loadGameLibrary();
        clearGameLifecycleOverlay();
        transitionTo(
            ONA_STATE.GAME_LIBRARY
        );
        await invoke("release_presentation_guard").catch(
            (error) =>
                console.warn("[ONA Presentation] Could not release guard after stale continue:", error)
        );
        return;
    }

    await invoke(
        "focus_running_game",
        {
            pid:
                pidBefore
        }
    ).catch(
        (error) =>
            console.warn("[ONA Presentation] Existing game foreground best-effort failed:", error)
    );

    await waitForTransition(220);
    const finalized =
        await finalizeSafeHandoffToGame(
            game,
            pidBefore,
            GameSessionState.RUNNING_FOREGROUND
        );

    if (!finalized) {
        return;
    }

    console.log(
        `[ONA Session] CONTINUE pid before=${pidBefore} after=${activeGameSession.pid}`
    );

}

quickControllersButton?.addEventListener(
    "click",
    () => {

        closeSystemMenu(
            ONA_STATE.CONTROLLERS
        );

    }
);


// =========================================================
// MINIMIZE ONA
// =========================================================

function captureConsolePresentationSnapshot() {

    const snapshot = {
        currentState,
        consolePresentationState,
        gameSessionState,
        presentationOwner,
        activeGameSession:
            activeGameSession
                ? { ...activeGameSession }
                : null,
        systemMenuOpen:
            Boolean(systemMenu?.classList.contains("visible")),
        gameOverlay:
            document.body.classList.contains("game-system-overlay"),
        nativeShellMinimize:
            false
    };

    snapshot.restoreTarget =
        stableRestoreTargetFromSnapshot(snapshot);

    return snapshot;

}


async function withPresentationTransaction(work) {

    presentationInvariantWarningsSuppressed =
        true;

    try {
        return await work();
    }
    finally {
        presentationInvariantWarningsSuppressed =
            false;
        assertPresentationInvariants();
    }

}


async function minimizeOnaConsoleExperience() {

    if (presentationOwner === PresentationOwner.ONA_MINIMIZED) {
        await invoke("minimize_main_window");
        return;
    }

    const gameAlive =
        Boolean(activeGameSession?.pid || runningGameId);

    if (gameAlive) {
        forceSystemMenuClosed("minimize_game");
        presentationOwner =
            PresentationOwner.GAME;
        currentState =
            ONA_STATE.GAME_RUNNING;
    }

    preMinimizePresentationState =
        captureConsolePresentationSnapshot();

    if (gameAlive) {
        preMinimizePresentationState.restoreTarget =
            RestoreTarget.GAME;
        preMinimizePresentationState.presentationOwner =
            PresentationOwner.GAME;
        preMinimizePresentationState.currentState =
            ONA_STATE.GAME_RUNNING;
        preMinimizePresentationState.systemMenuOpen =
            false;
        preMinimizePresentationState.gameOverlay =
            false;
    }

    console.log(
        `[ONA Minimize] preOwner=${preMinimizePresentationState.presentationOwner}`
    );
    console.log(
        `[ONA Minimize] ownerBefore=${preMinimizePresentationState.presentationOwner} restoreTarget=${preMinimizePresentationState.restoreTarget}`
    );
    console.log(
        "[ONA Minimize] saved window presentation state",
        preMinimizePresentationState
    );
    console.log(
        `[ONA MINIMIZE] gameAlive=${gameAlive} restoreTarget=${preMinimizePresentationState.restoreTarget} menu=${systemMenuRequestState}`
    );
    resetStartArbitration("MINIMIZE_BUTTON");

    await withPresentationTransaction(async () => {

        systemMenu
            ?.classList
            .remove("visible");

        document.body
            .classList
            .remove("system-menu-open");

        document.body
            .classList
            .remove("mouse-enabled");

        document.body
            .classList
            .remove("game-system-overlay");

        await setGameInputRouting(false);

        const pid =
            preMinimizePresentationState?.activeGameSession?.pid;

        if (pid) {
            try {
                const minimized =
                    await invoke(
                        "minimize_running_game",
                        {
                            pid
                        }
                    );
                if (!minimized) {
                    throw new Error("PRIMARY_GAME_WINDOW_MINIMIZE_FAILED");
                }
            }
            catch (error) {
                console.error("[ONA Presentation] Could not minimize PRIMARY game window:", error);
                await recoverPresentationToShell(
                    `PRIMARY_GAME_WINDOW_MINIMIZE_FAILED: ${error}`,
                    activeSessionGame()
                );
                return;
            }
        }

        await invoke("hide_system_overlay_over_game").catch(
            (error) =>
                console.warn("[ONA Presentation] Could not hide system overlay before minimize:", error)
        );

        clearGameLifecycleOverlay();
        setPresentationOwner(
            PresentationOwner.ONA_MINIMIZED
        );
        setGameSessionState(
            GameSessionState.MINIMIZED
        );

        if (activeGameSession) {
            updateActiveGameSession({
                state:
                    GameSessionState.MINIMIZED
            });
        }

        await invoke("minimize_main_window");

    });

}


async function handleNativeMainWindowMinimize() {

    if (
        presentationOwner === PresentationOwner.ONA_MINIMIZED ||
        uiOperationState === "launching" ||
        uiOperationState === "restoring"
    ) {
        return;
    }

    console.log(
        "[ONA Minimize] native event detected"
    );

    const gameAlive =
        Boolean(activeGameSession?.pid || runningGameId);

    if (gameAlive) {
        forceSystemMenuClosed("minimize_game");
        presentationOwner =
            PresentationOwner.GAME;
        currentState =
            ONA_STATE.GAME_RUNNING;
    }

    preMinimizePresentationState =
        captureConsolePresentationSnapshot();

    if (gameAlive) {
        preMinimizePresentationState.restoreTarget =
            RestoreTarget.GAME;
        preMinimizePresentationState.presentationOwner =
            PresentationOwner.GAME;
        preMinimizePresentationState.currentState =
            ONA_STATE.GAME_RUNNING;
        preMinimizePresentationState.systemMenuOpen =
            false;
        preMinimizePresentationState.gameOverlay =
            false;
    }
    nativeShellMinimizeRestore =
        preMinimizePresentationState.presentationOwner === PresentationOwner.ONA_SHELL;

    if (nativeShellMinimizeRestore) {
        preMinimizePresentationState.systemMenuOpen =
            false;
        preMinimizePresentationState.gameOverlay =
            false;
        preMinimizePresentationState.nativeShellMinimize =
            true;
        preMinimizePresentationState.restoreTarget =
            RestoreTarget.ONA_SHELL;
        systemMenu
            ?.classList
            .remove("visible");
        document.body
            .classList
            .remove("system-menu-open");
        document.body
            .classList
            .remove("mouse-enabled");
        document.body
            .classList
            .remove("game-system-overlay");
    }

    console.log(
        `[ONA Minimize] previousOwner=${preMinimizePresentationState.presentationOwner}`
    );
    console.log(
        `[ONA Minimize] ownerBefore=${preMinimizePresentationState.presentationOwner} restoreTarget=${preMinimizePresentationState.restoreTarget}`
    );
    console.log(
        `[ONA MINIMIZE] gameAlive=${gameAlive} restoreTarget=${preMinimizePresentationState.restoreTarget} menu=${systemMenuRequestState}`
    );
    resetStartArbitration("NATIVE_MINIMIZE");

    await withPresentationTransaction(async () => {
        await invoke("release_presentation_guard").catch(
            (error) =>
                console.warn("[ONA Minimize] Guard release during native minimize failed:", error)
        );
        await setGameInputRouting(false);
        setPresentationOwner(
            PresentationOwner.ONA_MINIMIZED
        );
        setGameSessionState(
            GameSessionState.MINIMIZED
        );
        console.log(
            "[ONA Minimize] owner=ONA_MINIMIZED"
        );
    });

}


async function restoreOnaConsoleExperience() {

    if (
        presentationOwner !== PresentationOwner.ONA_MINIMIZED ||
        restoringConsoleExperience
    ) {
        return;
    }

    restoringConsoleExperience =
        true;

    const snapshot =
        preMinimizePresentationState;
    const restoreTarget =
        snapshot?.restoreTarget || stableRestoreTargetFromSnapshot(snapshot);

    console.log(
        `[ONA Restore] restoring owner=${snapshot?.presentationOwner || PresentationOwner.ONA_SHELL}`
    );
    console.log(
        `[ONA Restore] target=${restoreTarget}`
    );

    try {
        await withPresentationTransaction(async () => {

            const pid =
                activeGameSession?.pid || snapshot?.activeGameSession?.pid;

            if (Boolean(activeGameSession?.pid || runningGameId) && pid) {
                forceSystemMenuClosed("restore_game");
                let restoreStatus;
                try {
                    restoreStatus =
                        await invoke(
                            "restore_running_game",
                            {
                                pid
                            }
                        );
                }
                catch (error) {
                    console.error("[ONA Presentation] Could not restore PRIMARY game window:", error);
                    await recoverPresentationToShell(
                        `PRIMARY_GAME_WINDOW_RESTORE_FAILED: ${error}`,
                        activeSessionGame()
                    );
                    return;
                }

                if (
                    !restoreStatus?.restored ||
                    !restoreStatus.exists ||
                    !restoreStatus.visible ||
                    restoreStatus.minimized ||
                    !restoreStatus.rectCompatible ||
                    !restoreStatus.clientRectCompatible ||
                    !restoreStatus.onTargetDisplay
                ) {
                    console.error(
                        "[ONA Presentation] PRIMARY game window failed restore validation; Shell remains visible.",
                        restoreStatus
                    );
                    await recoverPresentationToShell(
                        "PRIMARY_GAME_WINDOW_RESTORE_VALIDATION_FAILED",
                        activeSessionGame()
                    );
                    return;
                }

                console.log(
                    `[ONA Restore] game PRIMARY hwnd restored ${restoreStatus.primaryHwnd}`
                );
                currentState =
                    ONA_STATE.GAME_RUNNING;
                setUiOperation("running");
                setConsolePresentationState(
                    ConsolePresentationState.RUNNING
                );
                const restoredState =
                    snapshot.gameSessionState === GameSessionState.RUNNING
                        ? GameSessionState.RUNNING
                        : GameSessionState.RUNNING_FOREGROUND;
                setGameSessionState(
                    restoredState
                );
                updateActiveGameSession({
                    state:
                        restoredState,
                    presentationState:
                        ConsolePresentationState.RUNNING,
                    primaryHwnd:
                        restoreStatus.primaryHwnd
                });
                clearGameLifecycleOverlay();
                hideGameLifecycleOverlay();
                await invoke("suppress_running_game_taskbar_identity", { pid }).catch(
                    (error) =>
                        console.warn("[ONA GameWindow] taskbar suppression on restore failed:", error)
                );
                await invoke("focus_running_game", { pid }).catch(
                    (error) =>
                        console.warn("[ONA Restore] Could not focus PRIMARY game window:", error)
                );
                await invoke(
                    "prepare_shell_for_game",
                    {
                        reason:
                        "restore_game"
                    }
                );
                await invoke("release_presentation_guard").catch(
                    (error) =>
                        console.warn("[ONA Presentation] Guard release after GAME restore failed:", error)
                );
                resetStartArbitration("RESTORE_GAME");
                presentationInvariantWarningsSuppressed =
                    true;
                await setGameInputRouting(true);
                console.log("[ONA Restore] routing=true");
                setPresentationOwner(
                    PresentationOwner.GAME
                );
                await invoke("enable_escape_system_shortcut").catch(
                    (error) =>
                        console.warn("[ONA Keyboard] ESC native shortcut restore enable failed:", error)
                );
                presentationInvariantWarningsSuppressed =
                    false;
                assertPresentationInvariants();
                await logPostRestoreState("restore_game_complete");
                console.log("[ONA Restore] systemMenuState=CLOSED");
                console.log("[ONA Restore] startArbiter=IDLE");
                console.log("[ONA Restore] gameRouting=true");
                console.log("[ONA Restore] escShortcut=true");
                console.log("[ONA Restore] owner=GAME");
                console.log("[ONA Restore] completed");
                console.log(
                    `[ONA RESTORE] gameAlive=true target=GAME menu=${systemMenuRequestState} routing=${gameInputForwardingEnabled} esc=true owner=${presentationOwner} RETURN`
                );
                return;
            }

            if (
                restoreTarget === RestoreTarget.SYSTEM_OVERLAY
            ) {
                if (
                    snapshot?.nativeShellMinimize ||
                    !snapshot?.activeGameSession?.pid
                ) {
                    console.warn(
                        "[ONA State] INVALID SYSTEM OVERLAY REQUEST source=restore"
                    );
                } else {
                currentState =
                    ONA_STATE.QUICK_MENU;
                setUiOperation("running");
                setConsolePresentationState(
                    ConsolePresentationState.RUNNING
                );
                setGameSessionState(
                    GameSessionState.SYSTEM_OVERLAY
                );
                clearGameLifecycleOverlay();
                await invoke("show_system_overlay_over_game");
                await setGameInputRouting(false);
                setPresentationOwner(
                    PresentationOwner.ONA_SYSTEM_OVERLAY
                );
                systemMenu
                    ?.classList
                    .add("visible");
                document.body
                    .classList
                    .add("system-menu-open");
                document.body
                    .classList
                    .add("mouse-enabled");
                document.body
                    .classList
                    .add("game-system-overlay");
                setSystemMenuRequestState(
                    SystemMenuRequestState.OPEN,
                    "restore_overlay"
                );
                console.log(
                    "[ONA QuickMenu] Entered from running game"
                );
                console.log("[ONA Restore] systemMenuState=OPEN");
                console.log("[ONA Restore] gameRouting=false");
                console.log("[ONA Restore] escShortcut=true");
                console.log("[ONA Restore] owner=ONA_SYSTEM_OVERLAY");
                console.log("[ONA Restore] completed");
                return;
                }
            }

            if (snapshot?.activeGameSession?.state === GameSessionState.BACKGROUND) {
                console.log(
                    "[ONA Restore] applying shell mode"
                );
                await invoke("restore_shell_window_presentation").catch(
                    (error) =>
                        console.warn("[ONA Presentation] Could not restore shell after minimize:", error)
                );
                await invoke("release_presentation_guard").catch(() => {});
                await setGameInputRouting(false);
                systemMenu
                    ?.classList
                    .remove("visible");
                document.body
                    .classList
                    .remove("system-menu-open");
                document.body
                    .classList
                    .remove("mouse-enabled");
                document.body
                    .classList
                    .remove("game-system-overlay");
                setUiOperation("idle");
                setConsolePresentationState(
                    ConsolePresentationState.IDLE
                );
                setGameSessionState(
                    GameSessionState.BACKGROUND
                );
                setPresentationOwner(
                    PresentationOwner.ONA_SHELL
                );
                updateActiveGameSession({
                    state:
                        GameSessionState.BACKGROUND,
                    presentationState:
                        ConsolePresentationState.IDLE
                });
                transitionTo(
                    snapshot.currentState || ONA_STATE.HOME
                );
                console.log("[ONA Restore] QuickMenu=false");
                console.log("[ONA Restore] opacity=255");
                console.log("[ONA Restore] alwaysOnTop=false");
                console.log("[ONA Restore] owner=ONA_SHELL");
                console.log("[ONA Restore] completed");
                return;
            }

            console.log(
                "[ONA Restore] applying shell mode"
            );
            await invoke("restore_shell_window_presentation").catch(
                (error) =>
                    console.warn("[ONA Presentation] Could not restore shell presentation:", error)
            );
            await invoke("release_presentation_guard").catch(() => {});
            await setGameInputRouting(false);
            systemMenu
                ?.classList
                .remove("visible");
            document.body
                .classList
                .remove("system-menu-open");
            document.body
                .classList
                .remove("mouse-enabled");
            document.body
                .classList
                .remove("game-system-overlay");
            setPresentationOwner(
                PresentationOwner.ONA_SHELL
            );
            setGameSessionState(
                snapshot?.gameSessionState || GameSessionState.IDLE
            );
            setConsolePresentationState(
                snapshot?.consolePresentationState || ConsolePresentationState.IDLE
            );
            transitionTo(
                snapshot?.currentState || ONA_STATE.HOME
            );
            console.log("[ONA Restore] QuickMenu=false");
            console.log("[ONA Restore] opacity=255");
            console.log("[ONA Restore] alwaysOnTop=false");
            console.log("[ONA Restore] owner=ONA_SHELL");
            console.log("[ONA Restore] completed");

        });
    }
    finally {
        preMinimizePresentationState =
            null;
        nativeShellMinimizeRestore =
            false;
        restoringConsoleExperience =
            false;
    }

}


minimizeButton?.addEventListener(
    "click",
    async () => {

        try {

            await minimizeOnaConsoleExperience();

        }

        catch (error) {

            console.error(
                "Unable to minimize ONA:",
                error
            );

        }

    }
);

window.addEventListener(
    "focus",
    () => {
        console.log("[ONA NativeWindow] focus telemetry");
    }
);

document.addEventListener(
    "visibilitychange",
    () => {
        console.log(
            `[ONA NativeWindow] visibility telemetry hidden=${document.hidden}`
        );
    }
);

tauri.event.listen(
    "ona-native-minimized",
    () => {
        console.log("[ONA Minimize] native event");
        handleNativeMainWindowMinimize();
    }
);

tauri.event.listen(
    "ona-native-restored",
    () => {
        console.log("[ONA Restore] native event");
        console.log(
            `[ONA Restore] previousOwner=${preMinimizePresentationState?.presentationOwner || PresentationOwner.ONA_SHELL}`
        );
        restoreOnaConsoleExperience();
    }
);

tauri.event.listen(
    "ona-native-close-requested",
    () => {
        console.log("[ONA NativeWindow] close_requested");
        requestOnaExit();
    }
);

tauri.event.listen(
    "ona-escape-system-command",
    () => {
        console.log("[ONA Keyboard] ESC native down");
        handleEscapeSystemCommand("native");
    }
);

window.setInterval(
    () => {
        runPresentationWatchdog();
    },
    1000
);


// =========================================================
// RESTART
// =========================================================

restartButton?.addEventListener(
    "click",
    () => {

        console.log(
            "Restarting ONA..."
        );


        window.location.reload();

    }
);


// =========================================================
// EXIT TO WINDOWS
// =========================================================

exitButton?.addEventListener(
    "click",
    requestOnaExit
);


// =========================================================
// TEMPORARY DEBUG
// =========================================================

window.onaDebugConnect =
    () => {

        controllerConnected({

            name:
                "ONA Controller"

        });

    };


// =========================================================
// CONTROLLER CONNECTION FROM ONA CORE
// =========================================================

tauri.event.listen(
    "controller-connected",
    (event) => controllerConnected({
        name:
            "ONA Controller",
        playerId:
            Number(event.payload || 1)
    })
);

tauri.event.listen(
    "controller-input",
    (event) => {

        const input =
            typeof event.payload === "string"
                ? JSON.parse(event.payload)
                : event.payload;

        console.log("[CONTROLLER INPUT]", input);


        // =====================================================
        // JOYSTICK
        // =====================================================

        if (
            String(input.control || "").toUpperCase()
            === "JOYSTICK"
        ) {

            const x = Number(input.x || 0);
            const y = Number(input.y || 0);

            console.log(
                `[INPUT] JOYSTICK X: ${x.toFixed(2)} Y: ${y.toFixed(2)}`
            );

            updateControllerLabStick(
                x,
                y
            );

            if (
                currentState ===
                ONA_STATE.CONTROLLER_LAB
            ) {
                return;
            }


            if (
            currentState === ONA_STATE.PROFILE_SELECT ||
            currentState === ONA_STATE.MAIN_MENU ||
            currentState === ONA_STATE.CONTROLLERS ||
            currentState === ONA_STATE.GAME_LIBRARY ||
            currentState === ONA_STATE.STORE ||
            currentState === ONA_STATE.SETTINGS ||
            currentState === ONA_STATE.QUICK_MENU
            ) {

                handleUiJoystickNavigation(
                    x,
                    y
                );

                return;
            }


            return;
        }


        // =====================================================
        // BOTONES
        // =====================================================

        const button =
            String(
                input.button ||
                input.control ||
                ""
            ).toUpperCase();


        const state =
            String(
                input.state || ""
            ).toLowerCase();


        console.log(
            `[INPUT] ${button} ${state}`
        );

        updateControllerLabButton(
            button,
            state
        );

        if (button === "START") {
            if (handleStartButtonState(state)) {
                return;
            }
        }

        if (
            currentState ===
            ONA_STATE.CONTROLLER_LAB
        ) {

            if (
                state !== "down" &&
                state !== "pressed"
            ) {
                return;
            }

            if (
                button === "A"
            ) {

                saveControllerProfile();
                return;

            }

            if (
                button === "B"
            ) {

                transitionTo(
                    ONA_STATE.CONTROLLERS
                );

                return;

            }

            if (
                button === "START"
            ) {

                return;

            }

            return;

        }

        if (
            currentState ===
            ONA_STATE.QUICK_MENU
        ) {

            if (
                state !== "down" &&
                state !== "pressed"
            ) {
                return;
            }

            if (
                button === "A"
            ) {

                activateSelectedSystemMenuItem();
                return;

            }

            if (
                button === "B"
            ) {

                closeSystemMenu();
                return;

            }

            return;

        }


        // Solo reaccionar al PRESIONAR
        if (
            state !== "down" &&
            state !== "pressed"
        ) {

            return;

        }

        if (onaExitConfirmOpen) {

            if (
                button === "A"
            ) {
                onaExitConfirmOpen =
                    false;
                closeOnaOwnedRuntimeAndExit();
                return;
            }

            if (
                button === "B"
            ) {
                cancelOnaExit();
                return;
            }

            return;

        }

        if (importBrowserOpen) {

            if (uiBusy()) {
                return;
            }

            if (
                button === "A"
            ) {
                activateImportBrowser();
                return;
            }

            if (
                button === "B"
            ) {
                backFromImportBrowser();
                return;
            }

            return;

        }

        if (gameOptionsOpen) {

            if (
                button === "A"
            ) {
                activateGameOptions();
                return;
            }

        if (
            button === "X"
        ) {
            if (gameOptionsMode !== "menu") {
                return;
            }
            requestGameUninstallConfirmation();
            return;
        }

            if (
                button === "B"
            ) {
                backFromGameOptions();
                return;
            }

            return;

        }

        if (
            currentState === ONA_STATE.WAITING_CONTROLLER &&
            addingController &&
            button === "B"
        ) {

            addingController =
                false;

            transitionTo(
                ONA_STATE.CONTROLLERS
            );

            return;

        }

        if (
            button === "START" &&
            currentState !== ONA_STATE.WAITING_CONTROLLER
        ) {

            return;

        }


        // =====================================================
        // PROFILE SELECT
        // =====================================================

        if (
            currentState ===
            ONA_STATE.PROFILE_SELECT
        ) {

            // A = select profile
            if (
                button === "A"
            ) {

                const card =
                    profileGrid?.querySelector(
                        ".profile-card.selected"
                    );


                if (card) {

                    selectProfile(card);

                }

                return;
            }


            // B = back
            if (
                button === "B"
            ) {

                if (playerCountValue > 0) {
                    return;
                }

                transitionTo(
                    ONA_STATE.WAITING_CONTROLLER
                );

                return;
            }


            // Direction events
            if (
                button === "LEFT" ||
                button === "UP"
            ) {

                navigateProfiles(-1);
                return;

            }


            if (
                button === "RIGHT" ||
                button === "DOWN"
            ) {

                navigateProfiles(1);
                return;

            }

        }


        // =====================================================
        // MAIN MENU
        // =====================================================

        if (
            currentState ===
            ONA_STATE.MAIN_MENU
        ) {

            const menuItems =
                Array.from(
                    mainMenu?.querySelectorAll(
                        ".menu-item"
                    ) || []
                );


            if (!menuItems.length) {
                return;
            }


            // A = activate option
            if (
                button === "A"
            ) {

                if (uiBusy()) {
                    return;
                }

                const selected =
                    mainMenu.querySelector(
                        ".menu-item.selected"
                    );


                if (selected) {

                    console.log(
                        "[MENU] Activating:",
                        selected.dataset.action
                    );


                    // PLAY
                    if (
                        selected.dataset.action
                        === "play"
                    ) {

                        if (activeSessionIsBackground()) {
                            continueActiveGameSession();
                            return;
                        }

                        transitionTo(
                            ONA_STATE.GAME_LIBRARY
                        );

                    }


                    // RECENT GAME
                    if (
                        selected.dataset.action
                        === "recent"
                    ) {

                        const gameIndex =
                            Number(
                                selected.dataset.gameIndex
                            );

                        if (
                            Number.isInteger(gameIndex)
                        ) {
                            selectedGameIndex =
                                gameIndex;
                        }

                        transitionTo(
                            ONA_STATE.GAME_LIBRARY
                        );

                    }


                    // STORE
                    if (
                        selected.dataset.action
                        === "store"
                    ) {

                        transitionTo(
                            ONA_STATE.STORE
                        );

                    }


                    // CONTROLLERS
                    if (
                        selected.dataset.action
                        === "controllers"
                    ) {

                        transitionTo(
                            ONA_STATE.CONTROLLERS
                        );

                    }


                    // SETTINGS
                    if (
                        selected.dataset.action
                        === "settings"
                    ) {

                        transitionTo(
                            ONA_STATE.SETTINGS
                        );

                    }

                }

                return;
            }


            // B = back to profiles
            if (
                button === "B"
            ) {

                return;

                return;

            }


            // Direcciones
            let selectedIndex =
                menuItems.findIndex(
                    item =>
                        item.classList.contains(
                            "selected"
                        )
                );


            if (selectedIndex < 0) {
                selectedIndex = 0;
            }


            if (
                button === "LEFT" ||
                button === "UP"
            ) {

                selectedIndex--;

            }


            if (
                button === "RIGHT" ||
                button === "DOWN"
            ) {

                selectedIndex++;

            }


            if (
                selectedIndex < 0
            ) {

                selectedIndex =
                    menuItems.length - 1;

            }


            if (
                selectedIndex >=
                menuItems.length
            ) {

                selectedIndex = 0;

            }


            menuItems.forEach(
                item =>
                    item.classList.remove(
                        "selected"
                    )
            );


            menuItems[
                selectedIndex
            ].classList.add(
                "selected"
            );


            console.log(
                "[MENU] Selected:",
                menuItems[
                    selectedIndex
                ].dataset.action
            );

        }


        // =====================================================
        // CONTROLLERS
        // =====================================================

        if (
            currentState ===
            ONA_STATE.CONTROLLERS
        ) {

            if (
                button === "A"
            ) {

                activateSelectedControllerAction();
                return;

            }

            if (
                button === "B"
            ) {

                transitionTo(
                    ONA_STATE.MAIN_MENU
                );

                return;

            }

            return;

        }


        // =====================================================
        // STORE
        // =====================================================

        if (
            currentState ===
            ONA_STATE.STORE
        ) {

            if (
                button === "A"
            ) {
                return;
            }

            if (
                button === "B"
            ) {

                transitionTo(
                    ONA_STATE.MAIN_MENU
                );

                return;

            }

            if (
                button === "START"
            ) {

                return;

            }

            return;

        }


        // =====================================================
        // SETTINGS
        // =====================================================

        if (
            currentState ===
            ONA_STATE.SETTINGS
        ) {

            if (
                button === "A"
            ) {
                activateSettingsSelection();
                return;
            }

            if (
                button === "B"
            ) {
                if (settingsFocusLevel === "option") {
                    settingsFocusLevel =
                        "category";
                    settingsPanel
                        ?.querySelectorAll(".settings-option")
                        .forEach(
                            (option) =>
                                option.classList.remove("selected")
                        );
                }
                else {
                    transitionTo(
                        ONA_STATE.MAIN_MENU
                    );
                }

                return;
            }

            if (
                button === "START"
            ) {
                return;
            }

            return;

        }


        // =====================================================
        // GAME LIBRARY
        // =====================================================

        if (
            currentState ===
            ONA_STATE.GAME_LIBRARY
        ) {

            if (
                button === "A"
            ) {

                if (uiBusy()) {
                    return;
                }

                if (
                    installedGames.length &&
                    selectedGameIndex < installedGames.length
                ) {
                    openGameOptions();
                }
                else {
                    importGameButton?.click();
                }

                return;

            }

            if (
                button === "B"
            ) {

                transitionTo(
                    ONA_STATE.MAIN_MENU
                );

                return;

            }

            if (
                button === "X"
            ) {

                if (
                    !uiBusy() &&
                    installedGames.length &&
                    selectedGameIndex < installedGames.length
                ) {
                    openGameOptions();
                }

                return;

            }

            if (
                button === "START"
            ) {

                return;

            }

            return;

        }

    }
);


// =========================================================
// TEMPORARY DEBUG — RESET
// =========================================================

window.onaDebugReset =
    () => {

        playerCountValue = 0;

        selectedPlayer = 1;


        currentProfile = {

            player:
                1,

            id:
                "guest",

            name:
                "PLAYER 1",

            type:
                "GUEST"

        };


        connectionDot
            ?.classList
            .remove("connected");


        if (connectionText) {

            connectionText.textContent =
                "WAITING FOR CONTROLLER";

        }


        if (controllerStatus) {

            controllerStatus.textContent =
                "Scan the QR code with your phone to begin.";

        }


        if (playerCount) {

            playerCount.textContent =
                "0 / 10";

        }

        updateControllerLabConnection();

        updateControllersScreen();


        setState(
            ONA_STATE.WAITING_CONTROLLER
        );

    };


// =========================================================
// INIT
// =========================================================

console.log(
    "ONA Gaming Studio Shell initialized."
);

renderControllerLabButtons();

loadControllerProfile();

invoke("load_shell_settings")
    .then((settings) => {
        shellSettings =
            settings;
        applyShellSettings();
    })
    .catch((error) =>
        console.error("[ONA Settings] Initial load failed:", error)
    );


startIntro();
