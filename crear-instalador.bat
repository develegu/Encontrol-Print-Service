@echo off
echo =======================================================
echo   CONSTRUCTOR DE INSTALADOR - EN-CONTROL PRINT SERVICE
echo =======================================================
echo.
echo 0. Deteniendo procesos anteriores (para evitar error de acceso)...
taskkill /F /IM en-control-print-service.exe >nul 2>&1
timeout /t 2 /nobreak >nul

echo.
echo 1. Limpiando compilaciones previas...
cargo clean

echo.
echo 2. Compilando codigo Rust (Release)...
cargo build --release --target x86_64-pc-windows-msvc
if %ERRORLEVEL% NEQ 0 (
    echo [ERROR] Fallo la compilacion de Rust.
    pause
    exit /b %ERRORLEVEL%
)

rem Copiar ejecutable al destino esperado por WiX
if not exist target\release mkdir target\release
if exist target\x86_64-pc-windows-msvc\release\en-control-print-service.exe (
    copy /Y target\x86_64-pc-windows-msvc\release\en-control-print-service.exe target\release\en-control-print-service.exe >nul
) else if exist target\release\en-control-print-service.exe (
    echo EXE ya presente en target\release\
) else (
    echo [ERROR] No se encontró el ejecutable en target\\x86_64-pc-windows-msvc\\release ni en target\\release
    dir /s /b target\*en-control-print-service*.exe
    pause
    exit /b 1
)

rem Asegurar README.txt (copiado desde README.md)
if exist README.md copy /Y README.md README.txt >nul

echo.
echo 3. Generando Instalador MSI (WiX Toolset)...
set "WIX_PATH=C:\Program Files (x86)\WiX Toolset v3.14\bin"
"%WIX_PATH%\candle" installer.wxs
if %ERRORLEVEL% NEQ 0 (
    echo [ERROR] Fallo al ejecutar 'candle'. Verifica WiX Toolset.
    pause
    exit /b %ERRORLEVEL%
)

"%WIX_PATH%\light" -ext WixUIExtension -out EnControlPrintService.msi installer.wixobj
if %ERRORLEVEL% NEQ 0 (
    echo [ERROR] Fallo al ejecutar 'light'.
    pause
    exit /b %ERRORLEVEL%
)

echo.
echo 4. Construyendo instalador EXE (Bootstrapper Burn) con icono personalizado...
"%WIX_PATH%\candle" bundle.wxs
if %ERRORLEVEL% NEQ 0 (
    echo [ERROR] Fallo al ejecutar 'candle' para el bundle.
    pause
    exit /b %ERRORLEVEL%
)
"%WIX_PATH%\light" -ext WixBalExtension -out EnControlPrintServiceSetup.exe bundle.wixobj
if %ERRORLEVEL% NEQ 0 (
    echo [ERROR] Fallo al ejecutar 'light' para el bundle.
    pause
    exit /b %ERRORLEVEL%
)

echo.
echo =======================================================
echo   PROCESO COMPLETADO EXITOSAMENTE
echo =======================================================
echo.
echo   Instaladores generados:
echo     - MSI: EnControlPrintService.msi
echo     - EXE (con tu icono): EnControlPrintServiceSetup.exe
echo.
pause
