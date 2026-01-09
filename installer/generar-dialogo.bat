@echo off
setlocal EnableExtensions EnableDelayedExpansion
rem Script para generar fondos sólidos del instalador (sin logo)
rem Por defecto SOLO crea installer\dialog_with_logo.bmp (493x312)
rem Opcional: pasar --banner o -b para regenerar tambien installer\banner.bmp (493x58)
rem Opcional: pasar --banner-logo para generar banner con logo a la izquierda

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
set "DO_BANNER=%~2"
set "GEN_BANNER=no"
if /I "%DO_BANNER%"=="--banner" set "GEN_BANNER=yes"
if /I "%DO_BANNER%"=="-b" set "GEN_BANNER=yes"
if /I "%DO_BANNER%"=="--banner-logo" set "GEN_BANNER=yes" & set "BANNER_LOGO=yes"
echo Usando color de fondo: %BG_COLOR%  | Generar banner: %GEN_BANNER%
rem Detectar ImageMagick: ruta absoluta preferida, luego PATH
set "MAGICK_EXE="
for %%I in (
    "C:\Program Files\ImageMagick-7.1.2-Q16-HDRI\magick.exe"
    "C:\Program Files\ImageMagick-7.1.1-Q16-HDRI\magick.exe"
    "C:\Program Files\ImageMagick-7.1.0-Q16-HDRI\magick.exe"
) do (
    if exist "%%~I" set "MAGICK_EXE=%%~I"
)
if not defined MAGICK_EXE (
    for /f "usebackq delims=" %%P in (`where magick 2^>nul`) do if not defined MAGICK_EXE set "MAGICK_EXE=%%~fP"
)

if defined MAGICK_EXE (
    echo [ImageMagick] Usando: "%MAGICK_EXE%"
    echo [ImageMagick] Generando dialogo 493x312...
    "%MAGICK_EXE%" -size 493x312 canvas:"%BG_COLOR%" installer\dialog_with_logo.bmp
    if /I "%GEN_BANNER%"=="yes" (
        echo [ImageMagick] Generando banner 493x58...
        "%MAGICK_EXE%" -size 493x58  canvas:"%BG_COLOR%" installer\banner.bmp
        if /I "%BANNER_LOGO%"=="yes" (
            echo [ImageMagick] Superponiendo logo en banner...
            if not exist installer\logo-64.png (
                echo Creando logo-64...
                "%MAGICK_EXE%" assets\logo-source.png -resize 64x64 installer\logo-64.png
            )
            "%MAGICK_EXE%" composite -gravity west -geometry +10+0 installer\logo-64.png installer\banner.bmp installer\banner_with_logo.bmp
            if exist installer\banner_with_logo.bmp (
                move /Y installer\banner_with_logo.bmp installer\banner.bmp >nul
            )
        )
    ) else (
        echo [ImageMagick] Banner existente se mantiene sin cambios.
    )
) else (
    echo [AVISO] ImageMagick no esta en PATH ni en la ruta conocida. Intentando fallback con PowerShell...
        powershell -NoProfile -ExecutionPolicy Bypass -Command "
            param([string]$hex,[string]$genBanner)
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
            if ($genBanner -and ($genBanner.ToLower() -eq 'yes')) {
                New-SolidBmp 493 58  'installer\\banner.bmp'
                Write-Host '[PS] Banner regenerado'
            } else {
                Write-Host '[PS] Banner existente se mantiene sin cambios.'
            }
        " "%BG_COLOR%" "%GEN_BANNER%"
)

if exist installer\dialog_with_logo.bmp (
    for %%F in (installer\dialog_with_logo.bmp installer\banner.bmp) do (
        if exist %%F echo OK: %%~fF (%%~zF bytes)
    )
    echo.
    echo Listado de installer/:
    dir installer /b
    echo.
    echo Listo. WiX usara installer\dialog_with_logo.bmp (y banner solo si lo regeneraste).
    echo Cambiar color: generar-dialogo.bat #E5E7EB
    echo Incluir banner: generar-dialogo.bat #E5E7EB --banner
    pause
    exit /b 0
) else (
    echo [ERROR] No se pudieron generar las imagenes.
    pause
    exit /b 1
)
