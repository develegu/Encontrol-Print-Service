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
cargo build --release
if %ERRORLEVEL% NEQ 0 (
    echo [ERROR] Fallo la compilacion de Rust.
    pause
    exit /b %ERRORLEVEL%
)

echo.
echo 3. Extrayendo version de Cargo.toml...
for /f "tokens=2 delims==" %%a in ('findstr /r "^version[ ]*=" Cargo.toml') do set VERSION=%%a
set VERSION=%VERSION: =%
set VERSION=%VERSION:"=%
echo    Version detectada: %VERSION%

echo.
echo 4. Generando Instalador MSI (WiX Toolset)...
set "WIX_PATH=C:\Program Files (x86)\WiX Toolset v3.14\bin"
"%WIX_PATH%\candle" -ext WixUtilExtension -dVersion=%VERSION% installer.wxs
if %ERRORLEVEL% NEQ 0 (
    echo [ERROR] Fallo al ejecutar 'candle'. Verifica WiX Toolset.
    pause
    exit /b %ERRORLEVEL%
)

"%WIX_PATH%\light" -ext WixUIExtension -ext WixUtilExtension -out EnControlPrintService.msi installer.wixobj
if %ERRORLEVEL% NEQ 0 (
    echo [ERROR] Fallo al ejecutar 'light'.
    pause
    exit /b %ERRORLEVEL%
)

echo.
echo =======================================================
echo   PROCESO COMPLETADO EXITOSAMENTE
echo =======================================================
echo.
echo   Nuevo instalador generado: EnControlPrintService.msi
echo.
pause
