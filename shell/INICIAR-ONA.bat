@echo off
title ONA Gaming Studio - Development

echo ==========================================
echo        ONA GAMING STUDIO
echo        Development Launcher
echo ==========================================
echo.

cd /d "%~dp0"

echo Proyecto encontrado:
echo %CD%
echo.

if not exist "package.json" (
    echo ERROR: No se encontro package.json.
    echo.
    pause
    exit /b 1
)

echo Iniciando ONA...
echo.

npm run tauri dev

echo.
echo ==========================================
echo ONA se ha cerrado.
echo ==========================================
pause