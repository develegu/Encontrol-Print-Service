# Generar imágenes del instalador

Este documento explica cómo generar los recursos del instalador a partir de una imagen fuente: [assets/logo-source.png](assets/logo-source.png).

Recursos generados:

- [installer/ico.ico](installer/ico.ico): icono multi-resolución.
- [installer/banner.bmp](installer/banner.bmp): banner WiX UI (493x58).
- [installer/logo-128.png](installer/logo-128.png) y [installer/logo-64.png](installer/logo-64.png): variantes de logo.
- [installer/dialog_with_logo.bmp](installer/dialog_with_logo.bmp): fondo de diálogo (493x312).
- [installer/dialog_base.bmp](installer/dialog_base.bmp) y [installer/dialog_with_logo.bmp](installer/dialog_with_logo.bmp): fondo de diálogo (493x312); el logo se compone a la izquierda.

## Windows (CMD) con ruta absoluta

Verifica tu instalación de ImageMagick:

```bat
magick.exe -version
```

Si está instalado en `C:\Program Files\ImageMagick-7.1.2-Q16-HDRI\`, ejecuta desde la raíz del repo:

```bat
"C:\Program Files\ImageMagick-7.1.2-Q16-HDRI\magick.exe" assets\logo-source.png -background transparent -define icon:auto-resize=256,128,64,48,32,16 installer\ico.ico
"C:\Program Files\ImageMagick-7.1.2-Q16-HDRI\magick.exe" assets\logo-source.png -resize 493x58 -gravity center -background white -extent 493x58 installer\banner.bmp
"C:\Program Files\ImageMagick-7.1.2-Q16-HDRI\magick.exe" assets\logo-source.png -resize 128x128 installer\logo-128.png
"C:\Program Files\ImageMagick-7.1.2-Q16-HDRI\magick.exe" assets\logo-source.png -resize 64x64 installer\logo-64.png
"C:\Program Files\ImageMagick-7.1.2-Q16-HDRI\magick.exe" -size 493x312 canvas:white installer\dialog_with_logo.bmp
```

## PowerShell (equivalente)

```powershell
& 'C:\Program Files\ImageMagick-7.1.2-Q16-HDRI\magick.exe' 'assets/logo-source.png' -background transparent -define icon:auto-resize=256,128,64,48,32,16 'installer/ico.ico'
& 'C:\Program Files\ImageMagick-7.1.2-Q16-HDRI\magick.exe' 'assets/logo-source.png' -resize 493x58 -gravity center -background white -extent 493x58 'installer/banner.bmp'
& 'C:\Program Files\ImageMagick-7.1.2-Q16-HDRI\magick.exe' 'assets/logo-source.png' -resize 128x128 'installer/logo-128.png'
& 'C:\Program Files\ImageMagick-7.1.2-Q16-HDRI\magick.exe' 'assets/logo-source.png' -resize 64x64 'installer/logo-64.png'
& 'C:\Program Files\ImageMagick-7.1.2-Q16-HDRI\magick.exe' -size 493x312 'canvas:white' 'installer/dialog_with_logo.bmp'
```

## Git Bash (MSYS) con ruta `/c/...`

```bash
"/c/Program Files/ImageMagick-7.1.2-Q16-HDRI/magick.exe" assets/logo-source.png -background transparent -define icon:auto-resize=256,128,64,48,32,16 installer/logo.ico
"/c/Program Files/ImageMagick-7.1.2-Q16-HDRI/magick.exe" assets/logo-source.png -resize 493x58 -gravity center -background white -extent 493x58 installer/banner.bmp
"/c/Program Files/ImageMagick-7.1.2-Q16-HDRI/magick.exe" assets/logo-source.png -resize 128x128 installer/logo-128.png
"/c/Program Files/ImageMagick-7.1.2-Q16-HDRI/magick.exe" assets/logo-source.png -resize 64x64 installer/logo-64.png
"/c/Program Files/ImageMagick-7.1.2-Q16-HDRI/magick.exe" -size 493x312 canvas:white installer/dialog_with_logo.bmp
```

## Scripts disponibles (opcional)

- [installer/generar-imagenes-directo.bat](installer/generar-imagenes-directo.bat): autodetecta ImageMagick (7.1.x) y genera todos los recursos. Ejecutar en CMD.
- [installer/generar-imagenes.ps1](installer/generar-imagenes.ps1): equivalente en PowerShell.

Tras generar los recursos, compila el MSI:

```bat
crear-instalador.bat
```

Notas:

- Asegúrate de tener [assets/logo-source.png](assets/logo-source.png) con fondo transparente para mejores resultados.
- El banner usa fondo blanco y centra el logo; si prefieres fondo gris claro, cambia `background white` por `background "#F3F4F6"`.
