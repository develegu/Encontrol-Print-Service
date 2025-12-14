# 🚀 GUÍA RÁPIDA DE INICIO - En-Control Print Service

## 📋 ÍNDICE
1. [Instalar Rust](#1-instalar-rust)
2. [Configurar el Proyecto](#2-configurar-el-proyecto)
3. [Compilar y Probar](#3-compilar-y-probar)
4. [Probar con tu PWA](#4-probar-con-tu-pwa)
5. [GitHub Actions (Compilación Automática)](#5-github-actions)

---

## 1️⃣ INSTALAR RUST

### Windows (Tu PC):

1. **Descargar instalador:**
   ```
   https://rustup.rs/
   ```
   O directo: https://win.rustup.rs/x86_64

2. **Ejecutar `rustup-init.exe`**
   - Se abrirá una ventana de terminal
   - Presiona `1` + `Enter` (instalación por defecto)
   - Espera 3-5 minutos

3. **Reiniciar VS Code** (o la terminal que estés usando)

4. **Verificar instalación:**
   ```bash
   rustc --version
   cargo --version
   ```
   
   Deberías ver:
   ```
   rustc 1.75.0 (o superior)
   cargo 1.75.0 (o superior)
   ```

✅ **Rust instalado correctamente**

---

## 2️⃣ CONFIGURAR EL PROYECTO

### Mover la carpeta del proyecto:

El código ya está en: `c:\Users\henry.moreno\Downloads\en-control-main\en-control-main\rust-project\`

**Moverlo a la ubicación final:**

```bash
# Opción A: Desde PowerShell o CMD
cd C:\Users\henry.moreno\Downloads
move "en-control-main\en-control-main\rust-project" "en-control-print-service"
```

**O manualmente:**
1. Abre el Explorador de Windows
2. Navega a: `C:\Users\henry.moreno\Downloads\en-control-main\en-control-main\`
3. Corta la carpeta `rust-project`
4. Pégala en `C:\Users\henry.moreno\Downloads\`
5. Renómbrala a `en-control-print-service`

✅ **Proyecto ubicado en: `C:\Users\henry.moreno\Downloads\en-control-print-service\`**

---

## 3️⃣ COMPILAR Y PROBAR

### Abrir el proyecto:

```bash
cd C:\Users\henry.moreno\Downloads\en-control-print-service
```

### Primera compilación (tarda 2-5 minutos):

```bash
cargo build --release
```

**Lo que hace:**
- ⬇️  Descarga todas las dependencias (actix-web, serde, etc.)
- 🔨 Compila el proyecto en modo optimizado
- 📦 Genera el binario en: `target\release\en-control-print-service.exe`

**Salida esperada:**
```
   Compiling libc v0.2.150
   Compiling serde v1.0.193
   ...
   Compiling en-control-print-service v0.1.0
    Finished release [optimized] target(s) in 3m 42s
```

### Ejecutar el servicio:

```bash
# Desde la raíz del proyecto
cargo run --release
```

**O directamente:**
```bash
target\release\en-control-print-service.exe
```

**Deberías ver:**
```
🚀 ===========================================
🚀 En-Control Print Service v0.1.0
🚀 ===========================================
📡 Escuchando en: http://127.0.0.1:9100
🔧 Endpoints disponibles:
   GET  /hello      - Health check
   GET  /available  - Lista de impresoras
   GET  /default    - Impresora por defecto
   POST /write      - Imprimir datos ZPL
   GET  /update     - Verificar actualizaciones
🚀 ===========================================
```

✅ **Servicio corriendo correctamente**

### Probar los endpoints:

**Abrir otra terminal** y ejecutar:

```bash
# Health check
curl http://localhost:9100/hello

# Lista de impresoras
curl http://localhost:9100/available

# Impresora por defecto
curl http://localhost:9100/default
```

---

## 4️⃣ PROBAR CON TU PWA

### 1. Deja el servicio corriendo en la terminal

### 2. Abre tu PWA:
```
https://www.en-control.app/Impresion
```

### 3. Abre la consola del navegador (F12)

### 4. Ejecuta este código en la consola:

```javascript
// Test 1: Health check
fetch('http://localhost:9100/hello')
  .then(r => r.text())
  .then(console.log);

// Test 2: Obtener impresoras
fetch('http://localhost:9100/available')
  .then(r => r.json())
  .then(data => {
    console.log('Impresoras detectadas:', data);
  });
```

### 5. Modificar tu código Angular (ImpresionEtiquetas.component.ts):

Tu código actual usa `BrowserPrint.getLocalDevices()`.

**No necesitas cambiar nada** porque el servicio imita la API de Zebra.

Pero si quieres usar directamente el nuevo servicio:

```typescript
// En lugar de:
BrowserPrint.getLocalDevices(...)

// Puedes usar:
async initPrintService() {
  try {
    const response = await fetch('http://localhost:9100/available');
    const data = await response.json();
    
    this.ListaImpresoras = data.printer;
    this.ImpresoraSeleccionada = this.ListaImpresoras[0];
    
    this.gf.DismisLoadingPopOver(true, "Impresora detectada");
  } catch (error) {
    this.gf.DismisLoadingPopOver(false, "Error al detectar impresora");
  }
}
```

---

## 5️⃣ GITHUB ACTIONS (Compilación Automática)

### Paso 1: Crear repositorio en GitHub

1. Ve a: https://github.com/new
2. Nombre: `en-control-print-service`
3. Privado o Público (tu elección)
4. Crear repositorio

### Paso 2: Subir el código

```bash
cd C:\Users\henry.moreno\Downloads\en-control-print-service

git init
git add .
git commit -m "Initial commit: En-Control Print Service"
git branch -M main
git remote add origin https://github.com/TU-USUARIO/en-control-print-service.git
git push -u origin main
```

### Paso 3: Crear un release para compilar

```bash
# Crear tag v0.1.0
git tag v0.1.0
git push origin v0.1.0
```

**Qué sucede:**
1. GitHub Actions detecta el tag `v0.1.0`
2. Compila automáticamente para:
   - ✅ Windows (x64)
   - ✅ macOS Intel (x64)
   - ✅ macOS Apple Silicon (ARM64)
3. Crea un Release en GitHub con los 3 binarios listos para descargar

### Paso 4: Descargar los binarios compilados

1. Ve a: `https://github.com/TU-USUARIO/en-control-print-service/releases`
2. Verás el release `v0.1.0` con 3 archivos:
   - `en-control-print-service-windows-x64.exe`
   - `en-control-print-service-macos-x64`
   - `en-control-print-service-macos-arm64`

---

## 📊 COMPILACIÓN: Local vs GitHub

| Aspecto | Compilar Local | GitHub Actions |
|---------|----------------|----------------|
| **Windows** | ✅ Compilas en tu PC | ✅ GitHub lo compila |
| **macOS** | ❌ Necesitas Mac | ✅ GitHub lo compila |
| **Velocidad** | Rápido (3-5 min) | Medio (8-10 min) |
| **Uso** | Desarrollo | Producción |

**Recomendación:**
- **Local (Windows)**: Para desarrollo y pruebas rápidas
- **GitHub Actions**: Para generar binarios de producción

---

## 🛠️ COMANDOS ÚTILES

```bash
# Compilar en modo debug (rápido, binario grande)
cargo build

# Compilar en modo release (lento, binario pequeño optimizado)
cargo build --release

# Ejecutar con logs detallados
RUST_LOG=debug cargo run

# En Windows:
set RUST_LOG=debug && cargo run

# Ver tamaño del binario
# Windows:
dir target\release\en-control-print-service.exe

# Limpiar compilaciones anteriores
cargo clean
```

---

## 🐛 SOLUCIÓN DE PROBLEMAS

### Error: `cargo: command not found`
**Solución:** Reinicia VS Code o la terminal después de instalar Rust

### Error: `linking with 'link.exe' failed`
**Solución:** Instala Visual Studio Build Tools:
```
https://visualstudio.microsoft.com/downloads/
→ Build Tools for Visual Studio 2022
→ Workload: "Desktop development with C++"
```

### Error: `Address already in use` (puerto 9100)
**Solución:** Ya hay algo corriendo en el puerto 9100 (probablemente Zebra Browser Print)
```bash
# Windows: Ver qué usa el puerto
netstat -ano | findstr :9100

# Matar el proceso (reemplaza PID)
taskkill /PID <PID> /F
```

### El servicio no detecta impresoras
**Solución:** Verifica que tienes impresoras instaladas:
```
Panel de Control → Dispositivos e Impresoras
```

---

## 🎉 ¡LISTO!

Ahora tienes:
- ✅ Servicio Rust compilando localmente
- ✅ Compatible con tu PWA existente
- ✅ GitHub Actions configurado para compilación automática
- ✅ Binarios optimizados (~5-8 MB)
- ✅ Consumo mínimo de RAM (~10 MB)

**Siguiente paso:**
- Crear instaladores NSIS (Windows) y PKG (macOS)
- Implementar auto-inicio del servicio
- Agregar icono a la bandeja del sistema (opcional)

¿Necesitas ayuda con alguno de estos pasos? 🚀
