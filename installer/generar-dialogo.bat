@echo off
setlocal EnableExtensions EnableDelayedExpansion
rem Script para generar fondos solidos del instalador (sin logo)
rem Crea installer\dialog_with_logo.bmp (493x312) y opcionalmente installer\banner.bmp (493x58)

title Generar fondos solidos (dialog/banner)
pushd "%~dp0\.." >nul

if not exist installer (
    echo [ERROR] No existe carpeta 'installer'. Ejecuta este script desde el repo.
    pause
    exit /b 1
)

rem Color configurable en formato #RRGGBB (por defecto: gris claro #F3F4F6)
set "BG_COLOR=%~1"
if "%BG_COLOR%"=="" set "BG_COLOR=#F3F4F6"
echo Usando color de fondo: %BG_COLOR%

where magick >nul 2>&1
if %ERRORLEVEL% EQU 0 (
    echo [ImageMagick] Generando dialogo 493x312 y banner 493x58...
    magick -size 493x312 canvas:"%BG_COLOR%" installer\dialog_with_logo.bmp
    magick -size 493x58  canvas:"%BG_COLOR%" installer\banner.bmp
) else (
    echo [AVISO] ImageMagick no esta en PATH. Intentando fallback con PowerShell...
    powershell -NoProfile -ExecutionPolicy Bypass -Command "
      param([string]$hex)
      if (-not $hex) { $hex = '#F3F4F6' }
      function New-SolidBmp {
        param($w,$h,$out)
        $r=[Convert]::ToInt32($hex.Substring(1,2),16)
        $g=[Convert]::ToInt32($hex.Substring(3,2),16)
        $b=[Convert]::ToInt32($hex.Substring(5,2),16)
        Add-Type -AssemblyName System.Drawing
        $bmp = New-Object System.Drawing.Bitmap($w,$h)
        $gdi = [System.Drawing.Graphics]::FromImage($bmp)
        $brush = New-Object System.Drawing.SolidBrush([System.Drawing.Color]::FromArgb(255,$r,$g,$b))
        $gdi.FillRectangle($brush,0,0,$w,$h)
        $bmp.Save($out,[System.Drawing.Imaging.ImageFormat]::Bmp)
        $gdi.Dispose(); $brush.Dispose(); $bmp.Dispose()
      }
      New-SolidBmp 493 312 'installer\\dialog_with_logo.bmp'
      New-SolidBmp 493 58  'installer\\banner.bmp'
    " "%BG_COLOR%"
)

if exist installer\dialog_with_logo.bmp (
    for %%F in (installer\dialog_with_logo.bmp installer\banner.bmp) do (
        if exist %%F echo OK: %%~fF (%%~zF bytes)
    )
    echo.
    echo Listado de installer/:
    dir installer /b
    echo.
    echo Listo. WiX usara: installer\dialog_with_logo.bmp y installer\banner.bmp
    echo Puedes cambiar el color: generar-dialogo.bat #E5E7EB
    pause
    exit /b 0
) else (
    echo [ERROR] No se pudieron generar las imagenes.
    pause
    exit /b 1
)
