# Generar iconos para el instalador

Este documento explica cómo generar `installer/logo.ico` y `installer/banner.bmp` a partir de una imagen fuente.

Requisitos:

- ImageMagick (`convert`) instalado y en PATH.

Pasos:

1. Coloca la imagen fuente en `assets/logo-source.png` (preferible PNG con fondo transparente).

2. Desde la carpeta `scripts` ejecuta (Windows PowerShell):

```powershell
.\generate-icons.ps1 -SourcePath ..\assets\logo-source.png
```

o en macOS / Linux (bash):

```bash
./generate-icons.sh ../assets/logo-source.png
```

3. Esto generará:

- `installer/logo.ico` — icono multi-resolución para Add/Remove Programs
- `installer/banner.bmp` — banner para WiX UI (493x58)

4. Vuelve a compilar el instalador (`crear-instalador.bat`) para incluir los recursos.

Notas:

- Si quieres que yo genere los iconos directamente, sube la imagen fuente `assets/logo-source.png` al repo o pega su base64 aquí y lo genero y añado por ti.
- Para distribución pública en macOS/Windows considera `codesign` y notarización/firma del instalador.
