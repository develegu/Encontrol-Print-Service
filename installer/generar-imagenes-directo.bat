@echo off
setlocal EnableExtensions EnableDelayedExpansion
rem Generar imagenes del instalador encontrando ImageMagick automaticamente

rem Bloquear ejecucion desde Git Bash/MSYS que rompe sintaxis de CMD
if not "%MSYSTEM%"=="" (
  echo [ERROR] Este script .bat debe ejecutarse en CMD o PowerShell, no en Git Bash.
  echo Usa: installer\generar-imagenes.ps1 en PowerShell o ejecuta este .bat desde CMD.
  pause
  exit /b 1
)

set "MAGICK_EXE="
rem 1) Buscar instalaciones tipicas en %ProgramFiles% (64-bit) y %ProgramFiles(x86)% (32-bit)
if exist "%ProgramFiles%\ImageMagick-7.1.2-Q16-HDRI\magick.exe" set "MAGICK_EXE=%ProgramFiles%\ImageMagick-7.1.2-Q16-HDRI\magick.exe"
if not defined MAGICK_EXE if exist "%ProgramFiles%\ImageMagick-7.1.1-Q16-HDRI\magick.exe" set "MAGICK_EXE=%ProgramFiles%\ImageMagick-7.1.1-Q16-HDRI\magick.exe"
if not defined MAGICK_EXE if exist "%ProgramFiles%\ImageMagick-7.1.0-Q16-HDRI\magick.exe" set "MAGICK_EXE=%ProgramFiles%\ImageMagick-7.1.0-Q16-HDRI\magick.exe"
if not defined MAGICK_EXE if exist "%ProgramFiles(x86)%\ImageMagick-7.1.2-Q16-HDRI\magick.exe" set "MAGICK_EXE=%ProgramFiles(x86)%\ImageMagick-7.1.2-Q16-HDRI\magick.exe"
if not defined MAGICK_EXE if exist "%ProgramFiles(x86)%\ImageMagick-7.1.1-Q16-HDRI\magick.exe" set "MAGICK_EXE=%ProgramFiles(x86)%\ImageMagick-7.1.1-Q16-HDRI\magick.exe"
if not defined MAGICK_EXE if exist "%ProgramFiles(x86)%\ImageMagick-7.1.0-Q16-HDRI\magick.exe" set "MAGICK_EXE=%ProgramFiles(x86)%\ImageMagick-7.1.0-Q16-HDRI\magick.exe"

rem 2) Si no, intentar via PATH con 'where'
if not defined MAGICK_EXE for /f "delims=" %%I in ('where magick.exe 2^>nul') do if not defined MAGICK_EXE set "MAGICK_EXE=%%~fI"

if not defined MAGICK_EXE (
  echo [ERROR] No se encontro ImageMagick (magick.exe).
  echo Instala ImageMagick 7.1.x Q16 HDRI o agrega 'magick.exe' al PATH.
  echo Ejemplo de ruta: "C:\Program Files\ImageMagick-7.1.2-Q16-HDRI\magick.exe"
  pause
  exit /b 1
)

rem Ir a la raiz del repo (este script esta en installer/)
pushd "%~dp0.." >nul

if not exist installer (
  echo [ERROR] No existe carpeta 'installer'.
  pause
  exit /b 1
)

rem Color de fondo para dialogo y banner (por defecto #F3F4F6)
set "BG_COLOR=#F3F4F6"
set "BANNER_MODE=overlay"

rem Permitir color en 1er o 2do argumento y flag --banner-from-logo en cualquier posicion simple
if not "%~1"=="" (
  if /i not "%~1"=="--banner-from-logo" set "BG_COLOR=%~1"
)
if /i "%~1"=="--banner-from-logo" set "BANNER_MODE=from_logo"
if /i "%~2"=="--banner-from-logo" set "BANNER_MODE=from_logo"

echo Usando ImageMagick: "%MAGICK_EXE%"
"%MAGICK_EXE%" -version >nul 2>&1
if errorlevel 1 (
  echo [ERROR] magick.exe fue localizado pero no pudo ejecutarse.
  echo Prueba manualmente: "%MAGICK_EXE%" -version
  pause
  exit /b 1
)
echo Color de fondo: %BG_COLOR%

rem Generar icono .ico (ico.ico)
if exist assets\logo-source.png (
  "%MAGICK_EXE%" assets\logo-source.png -background transparent -define icon:auto-resize=256,128,64,48,32,16 installer\ico.ico
  "%MAGICK_EXE%" assets\logo-source.png -resize 128x128 installer\logo-128.png
  "%MAGICK_EXE%" assets\logo-source.png -resize 64x64 installer\logo-64.png
  if exist installer\ico.ico copy /Y installer\ico.ico installer\logo.ico >nul
) else (
  echo [AVISO] No existe assets\logo-source.png; se omitira la generacion del .ico y logos PNG.
)

rem Generar dialogo base (fondo solido 493x312) y componer logo a la izquierda si existe
"%MAGICK_EXE%" -size 493x312 canvas:"%BG_COLOR%" installer\dialog_base.bmp
if exist installer\logo-128.png (
  "%MAGICK_EXE%" composite -gravity west -geometry +10+0 installer\logo-128.png installer\dialog_base.bmp installer\dialog_with_logo.bmp
) else (
  copy /Y installer\dialog_base.bmp installer\dialog_with_logo.bmp >nul
  echo [AVISO] installer\logo-128.png no existe; dialogo sin logo.
)

rem Generar banner (493x58): modo from_logo o overlay por defecto
if /i "%BANNER_MODE%"=="from_logo" (
  if exist assets\logo-source.png (
    "%MAGICK_EXE%" assets\logo-source.png -resize 493x58 -gravity center -background "%BG_COLOR%" -extent 493x58 installer\banner.bmp
  ) else (
    echo [AVISO] No existe assets\logo-source.png; se generara banner base sin logo.
    "%MAGICK_EXE%" -size 493x58 canvas:"%BG_COLOR%" installer\banner.bmp
  )
) else (
  "%MAGICK_EXE%" -size 493x58 canvas:"%BG_COLOR%" installer\banner.bmp
  if exist installer\logo-64.png (
    "%MAGICK_EXE%" composite -gravity west -geometry +10+0 installer\logo-64.png installer\banner.bmp installer\banner_with_logo.bmp
    if exist installer\banner_with_logo.bmp move /Y installer\banner_with_logo.bmp installer\banner.bmp >nul
  ) else (
    echo [AVISO] installer\logo-64.png no existe; banner sin logo.
  )
)

echo(
echo Resultado:
for %%F in (installer\ico.ico installer\logo.ico installer\dialog_with_logo.bmp installer\banner.bmp) do if exist %%F echo OK: %%~fF ^(%%~zF bytes^)

dir installer /b

echo(
echo Imagenes generadas. Continua con:
echo   crear-instalador.bat
pause
