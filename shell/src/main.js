// =========================================================
// ONA GAMING STUDIO
// ONA SHELL
// =========================================================

console.log("ONA Shell initializing...");

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

const resumeButton =
    document.getElementById("resume-button");

const restartButton =
    document.getElementById("restart-button");

const exitButton =
    document.getElementById("exit-button");


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

    SYSTEM_MENU:
        "system-menu"

};


let currentState =
    ONA_STATE.BOOT;


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

}


// =========================================================
// SHOW CONTROLLER SCREEN
// =========================================================

function showControllerScreen() {

    hideAllScreens();

    controllerScreen
        ?.classList
        .add("active");

    generateQR();

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


        case ONA_STATE.SYSTEM_MENU:

            break;

    }

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


    setState(
        ONA_STATE.WAITING_CONTROLLER
    );

}


// =========================================================
// CONTROLLER CONNECTED
// =========================================================

function controllerConnected(
    controllerInfo = {}
) {

    playerCountValue = 1;


    if (playerCount) {

        playerCount.textContent =
            `${playerCountValue} / 10`;

    }


    connectionDot
        ?.classList
        .add("connected");


    if (connectionText) {

        connectionText.textContent =
            "PLAYER 1 CONNECTED";

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


    setTimeout(
        () => {

            setState(
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
        profileGrid.querySelectorAll(
            ".profile-card"
        );


    cards.forEach(
        (card) => {

            const player =
                Number(
                    card.dataset.player
                );


            card.classList.toggle(
                "selected",
                player === selectedPlayer
            );

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

            setState(
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


    const selectableCards =
        cards.filter(
            (card) =>
                card.dataset.profile !== "add"
        );


    let currentIndex =
        selectableCards.findIndex(
            (card) =>
                Number(
                    card.dataset.player
                ) === selectedPlayer
        );


    if (currentIndex < 0) {

        currentIndex = 0;

    }


    currentIndex += direction;


    if (
        currentIndex < 0
    ) {

        currentIndex =
            selectableCards.length - 1;

    }


    if (
        currentIndex >=
        selectableCards.length
    ) {

        currentIndex = 0;

    }


    const card =
        selectableCards[currentIndex];


    selectedPlayer =
        Number(
            card.dataset.player
        );


    updateProfileSelection();

}


// =========================================================
// KEYBOARD INPUT
// =========================================================

window.addEventListener(
    "keydown",
    (event) => {

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
                            `[data-player="${selectedPlayer}"]`
                        );


                    if (card) {

                        selectProfile(card);

                    }

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


        if (
            currentState ===
            ONA_STATE.SYSTEM_MENU
        ) {

            closeSystemMenu();

            return;

        }


        openSystemMenu();

    }
);


// =========================================================
// SYSTEM MENU
// =========================================================

function openSystemMenu() {

    console.log(
        "Opening ONA system menu"
    );


    currentState =
        ONA_STATE.SYSTEM_MENU;


    systemMenu
        ?.classList
        .add("visible");


    document.body
        .classList
        .add("system-menu-open");


    document.body
        .classList
        .add("mouse-enabled");


    resumeButton?.focus();

}


// =========================================================
// CLOSE SYSTEM MENU
// =========================================================

function closeSystemMenu() {

    console.log(
        "Closing ONA system menu"
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


    if (
        playerCountValue > 0
    ) {

        currentState =
            ONA_STATE.MAIN_MENU;

    }

    else {

        currentState =
            ONA_STATE.WAITING_CONTROLLER;

    }

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
    async () => {

        console.log(
            "Exiting ONA..."
        );


        try {

            const appWindow =
                tauri
                    ?.window
                    ?.getCurrentWindow();


            if (appWindow) {

                await appWindow.close();

            }

        }

        catch (error) {

            console.error(
                "Unable to close ONA:",
                error
            );

        }

    }
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
    () => controllerConnected({ name: "ONA Controller" })
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


            // Zona muerta
            if (
                Math.abs(x) < 0.35 &&
                Math.abs(y) < 0.35
            ) {
                return;
            }


            // ================================================
            // PROFILE SELECT
            // ================================================

            if (
                currentState ===
                ONA_STATE.PROFILE_SELECT
            ) {

                if (
                    Math.abs(x) > Math.abs(y)
                ) {

                    navigateProfiles(
                        x > 0 ? 1 : -1
                    );

                }

                else {

                    navigateProfiles(
                        y > 0 ? 1 : -1
                    );

                }

                return;
            }


            // ================================================
            // MAIN MENU
            // ================================================

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
                    Math.abs(x) >
                    Math.abs(y)
                ) {

                    selectedIndex +=
                        x > 0 ? 1 : -1;

                }

                else {

                    selectedIndex +=
                        y > 0 ? 1 : -1;

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


        // Solo reaccionar al PRESIONAR
        if (
            state !== "down" &&
            state !== "pressed"
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

            // A = seleccionar perfil
            if (
                button === "A"
            ) {

                const card =
                    profileGrid?.querySelector(
                        `[data-player="${selectedPlayer}"]`
                    );


                if (card) {

                    selectProfile(card);

                }

                return;
            }


            // B = regresar
            if (
                button === "B"
            ) {

                setState(
                    ONA_STATE.WAITING_CONTROLLER
                );

                return;
            }


            // D-PAD / botones direccionales
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


            // A = activar opción
            if (
                button === "A"
            ) {

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

                        console.log(
                            "[ONA] PLAY selected"
                        );

                    }


                    // CONTROLLERS
                    if (
                        selected.dataset.action
                        === "controllers"
                    ) {

                        setState(
                            ONA_STATE.WAITING_CONTROLLER
                        );

                    }

                }

                return;
            }


            // B = volver a perfiles
            if (
                button === "B"
            ) {

                setState(
                    ONA_STATE.PROFILE_SELECT
                );

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
                "WAITING FOR PLAYER 1";

        }


        if (controllerStatus) {

            controllerStatus.textContent =
                "Connect your ONA Controller to begin.";

        }


        if (playerCount) {

            playerCount.textContent =
                "0 / 10";

        }


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


startIntro();
