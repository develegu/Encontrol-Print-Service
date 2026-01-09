param(
    [string]$Color = "#F3F4F6",
    [ValidateSet("SolidOverlay", "FromLogo")]
    [string]$Mode = "FromLogo"
)

$ErrorActionPreference = 'Stop'

function Find-Magick {
    $candidates = @(
        "$env:ProgramFiles\ImageMagick-7.1.2-Q16-HDRI\magick.exe",
        "$env:ProgramFiles\ImageMagick-7.1.1-Q16-HDRI\magick.exe",
        "$env:ProgramFiles\ImageMagick-7.1.0-Q16-HDRI\magick.exe",
        "$env:ProgramFiles(x86)\ImageMagick-7.1.2-Q16-HDRI\magick.exe",
        "$env:ProgramFiles(x86)\ImageMagick-7.1.1-Q16-HDRI\magick.exe",
        "$env:ProgramFiles(x86)\ImageMagick-7.1.0-Q16-HDRI\magick.exe"
    )
    foreach ($p in $candidates) { if (Test-Path $p) { return $p } }
    $cmd = Get-Command magick.exe -ErrorAction SilentlyContinue
    if ($cmd) { return $cmd.Source }
    return $null
}

$magick = Find-Magick
if (-not $magick) {
    Write-Error "No se encontró ImageMagick (magick.exe). Instala 7.1.x Q16 HDRI o agrega al PATH."
    exit 1
}

# Ir a la raíz del repo (este script está en installer/)
$scriptDir = Split-Path -Parent $PSCommandPath
$root = Split-Path -Parent $scriptDir
Set-Location $root

if (-not (Test-Path 'installer')) {
    Write-Error "No existe carpeta 'installer' en $root"
    exit 1
}

Write-Host "Usando ImageMagick: $magick"
Write-Host "Color de fondo: $Color" "| Modo banner: $Mode"

# Generar icono y logos
if (Test-Path 'assets/logo-source.png') {
    & $magick 'assets/logo-source.png' -background transparent -define icon:auto-resize=256, 128, 64, 48, 32, 16 'installer/ico.ico'
    & $magick 'assets/logo-source.png' -resize 128x128 'installer/logo-128.png'
    & $magick 'assets/logo-source.png' -resize 64x64 'installer/logo-64.png'
    if (Test-Path 'installer/ico.ico') { Copy-Item 'installer/ico.ico' 'installer/logo.ico' -Force }
}
else {
    Write-Warning 'No existe assets/logo-source.png; se omitirá la generación del .ico y logos PNG.'
}

# Dialogo base 493x312 y composición de logo a la izquierda
& $magick -size 493x312 "canvas:$Color" 'installer/dialog_base.bmp'
if (Test-Path 'installer/logo-128.png') {
    & $magick composite -gravity west -geometry +10+0 'installer/logo-128.png' 'installer/dialog_base.bmp' 'installer/dialog_with_logo.bmp'
}
else {
    Copy-Item 'installer/dialog_base.bmp' 'installer/dialog_with_logo.bmp' -Force
    Write-Warning 'installer/logo-128.png no existe; diálogo sin logo.'
}

# Banner 493x58
if ($Mode -eq 'FromLogo') {
    if (Test-Path 'assets/logo-source.png') {
        & $magick 'assets/logo-source.png' -resize 493x58 -gravity center -background $Color -extent 493x58 'installer/banner.bmp'
    }
    else {
        Write-Warning 'No existe assets/logo-source.png; se generará banner base sin logo.'
        & $magick -size 493x58 "canvas:$Color" 'installer/banner.bmp'
    }
}
else {
    & $magick -size 493x58 "canvas:$Color" 'installer/banner.bmp'
    if (Test-Path 'installer/logo-64.png') {
        & $magick composite -gravity west -geometry +10+0 'installer/logo-64.png' 'installer/banner.bmp' 'installer/banner_with_logo.bmp'
        if (Test-Path 'installer/banner_with_logo.bmp') { Move-Item 'installer/banner_with_logo.bmp' 'installer/banner.bmp' -Force }
    }
    else {
        Write-Warning 'installer/logo-64.png no existe; banner sin logo.'
    }
}

Write-Host "\nResultado:"
Get-ChildItem installer/ico.ico, installer/logo.ico, installer/dialog_with_logo.bmp, installer/banner.bmp -ErrorAction SilentlyContinue | ForEach-Object {
    Write-Host ("OK: {0} ({1} bytes)" -f $_.FullName, $_.Length)
}

Write-Host "\nImágenes generadas. Continúa con: crear-instalador.bat"
