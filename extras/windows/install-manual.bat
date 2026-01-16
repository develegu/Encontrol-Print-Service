@echo off
echo =======================================================
echo   REGISTRO MANUAL DE SERVICIO EN-CONTROL (Windows)
echo =======================================================
echo.
echo NOTA: El instalador MSI ya registra este servicio automaticamente.
echo Este script es solo para desarrollo o registro manual.
echo.
echo 1. Deteniendo servicio existente (si existe)...
sc stop EnControlPrintService >nul 2>&1
sc delete EnControlPrintService >nul 2>&1

echo.
echo 2. Obteniendo ruta del ejecutable...
set "EXE_PATH=%~dp0..\..\target\release\en-control-print-service.exe"

if not exist "%EXE_PATH%" (
    echo [ERROR] No se encuentra el ejecutable en: %EXE_PATH%
    echo Asegurate de compilar primero con 'cargo build --release'
    pause
    exit /b 1
)

echo Ruta detectada: %EXE_PATH%
echo.

echo 3. Registrando servicio...
sc create EnControlPrintService binPath= "\"%EXE_PATH%\"" start= auto DisplayName= "En-Control Print Service"
if %ERRORLEVEL% NEQ 0 (
    echo [ERROR] Fallo al crear el servicio.
    echo ASEGURATE DE EJECUTAR ESTE SCRIPT COMO ADMINISTRADOR.
    pause
    exit /b 1
)

sc description EnControlPrintService "Servicio de impresion local para En-Control PWA"

echo.
echo 4. Iniciando servicio...
sc start EnControlPrintService

echo.
echo =======================================================
echo   PROCESO COMPLETADO
echo =======================================================
echo Verifica en services.msc si "En-Control Print Service" esta corriendo.
echo.
pause
