@echo off
echo ========================================
echo  En-Control Print Service - Setup
echo ========================================
echo.

REM Verificar si Rust está instalado
where rustc >nul 2>&1
if %errorlevel% neq 0 (
    echo [ERROR] Rust no esta instalado
    echo.
    echo Por favor instala Rust desde: https://rustup.rs/
    echo Luego reinicia esta ventana y ejecuta este script nuevamente.
    echo.
    pause
    exit /b 1
)

echo [OK] Rust detectado: 
rustc --version
cargo --version
echo.

echo ========================================
echo  Compilando proyecto...
echo ========================================
echo.
echo Esto puede tardar 3-5 minutos la primera vez.
echo.

cargo build --release

if %errorlevel% neq 0 (
    echo.
    echo [ERROR] Fallo la compilacion
    pause
    exit /b 1
)

echo.
echo ========================================
echo  Compilacion exitosa!
echo ========================================
echo.
echo El binario esta en:
echo   target\release\en-control-print-service.exe
echo.
echo Tamaño del archivo:
dir target\release\en-control-print-service.exe | findstr "en-control"
echo.

choice /C YN /M "Quieres ejecutar el servicio ahora"
if errorlevel 2 goto end
if errorlevel 1 goto run

:run
echo.
echo Iniciando servicio en http://localhost:9100
echo Presiona Ctrl+C para detener
echo.
target\release\en-control-print-service.exe

:end
echo.
echo Gracias por usar En-Control Print Service!
pause
