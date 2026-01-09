# 🦀 En-Control Print Service

Servicio local de impresión para la PWA **En-Control**, compatible con la API de Zebra Browser Print.

## 🎯 Características

- ✅ **Memoria mínima**: ~5-10 MB RAM
- ✅ **Multiplataforma**: Windows y macOS
- ✅ **API compatible**: Drop-in replacement para Zebra Browser Print
- ✅ **Auto-actualización**: Mantiene el servicio actualizado automáticamente
- ✅ **Binarios pequeños**: ~4-8 MB por plataforma

---

## 📦 Instalación

### Opción 1: Descarga el instalador (Recomendado)

1. Ve a [Releases](https://github.com/tu-usuario/en-control-print-service/releases)
2. Descarga el instalador para tu plataforma:
   - **macOS Intel**: `en-control-print-service-macos-x64`
   - **macOS ARM (M1/M2)**: `en-control-print-service-macos-arm64`
   - Nota: El instalador de Windows (MSI/EXE) se genera localmente y puede subirse manualmente al release.
3. Ejecuta el instalador

### Opción 2: Compilar desde código fuente (Windows)

Si deseas realizar cambios y generar un nuevo instalador:

1. Asegúrate de tener **Rust** e **WiX Toolset v3.14** instalados.
2. Genera el fondo del diálogo (y opcionalmente el banner con logo):

   ```cmd
   installer\generar-dialogo.bat #F3F4F6 --banner-logo
   ```

   - Solo diálogo (color por defecto): `installer\generar-dialogo.bat`
   - Solo diálogo con color: `installer\generar-dialogo.bat #E5E7EB`

3. Ejecuta el script automatizado para construir el instalador:
   ```cmd
   crear-instalador.bat
   ```
   Este script compila el binario y genera `EnControlPrintService.msi` y `EnControlPrintServiceSetup.exe` (bundle con icono).

> Requisitos Windows locales: Rust (cargo), WiX Toolset 3.x y, opcionalmente, ImageMagick (el script intentará localizarlo automáticamente).

---

## ⚙️ CI (GitHub Actions)

- El CI publica automáticamente binarios para macOS (x64 y ARM) como `.tar.gz`.
- Los instaladores de Windows se construyen localmente con los scripts indicados y, si se desea, se anexan manualmente al release.

---

## 🚀 Uso

El servicio inicia automáticamente como un servicio de Windows después de la instalación. Escucha en `http://localhost:9100`.

### Endpoints disponibles (Compatibles con Zebra)

| Método | Endpoint       | Descripción                           |
| ------ | -------------- | ------------------------------------- |
| `GET`  | `/hello`       | Health check (Status del servicio)    |
| `GET`  | `/available`   | Lista impresoras físicas (No PDF/XPS) |
| `GET`  | `/default`     | Impresora por defecto del sistema     |
| `GET`  | `/status`      | Estado de impresora (online/ready)    |
| `GET`  | `/status-page` | Página HTML con estado de impresora   |
| `POST` | `/write`       | Envía datos ZPL/RAW a imprimir        |
| `GET`  | `/update`      | Verifica actualizaciones disponibles  |

### Ejemplo de uso desde JavaScript

```javascript
// Obtener lista de impresoras
const response = await fetch("http://localhost:9100/available");
const data = await response.json();
// data.printer será [] si no hay impresoras (Evita Error 500)

// Comprobar estado de impresora (online/ready)
async function checkPrinterStatus(baseUrl, name) {
  const url = new URL(`${baseUrl}/status`);
  if (name) url.searchParams.set("name", name);
  const res = await fetch(url.toString(), { method: "GET" });
  if (!res.ok) throw new Error("Status request failed");
  const st = await res.json(); // { online, ready, status_text, ... }
  if (!st.online)
    throw new Error(
      `Impresora offline: ${st.status_text || "apagada/desconectada"}`
    );
  if (!st.ready)
    throw new Error(`Impresora no lista: ${st.status_text || "en error"}`);
  return st;
}

// Imprimir datos ZPL
const zpl = "^XA^FO50,50^ADN,36,20^FDHola Mundo^FS^XZ";
const res = await fetch("http://localhost:9100/write", {
  method: "POST",
  mode: "cors",
  headers: { "Content-Type": "application/json" },
  body: JSON.stringify({ data: zpl }),
});
if (!res.ok) {
  const err = await res.json().catch(() => ({}));
  const hdrMsg = res.headers.get("X-Printer-Message");
  const msg = hdrMsg || err?.error || `Error al imprimir (HTTP ${res.status})`;
  alert(msg);
}
```

### Contrato de errores visibles

- Si la impresora está apagada/offline, `POST /write` devuelve `409 Conflict` con:
  - Header: `X-Printer-Status: offline`
  - Header: `X-Printer-Code: PRN_OFFLINE`
  - Header: `X-Printer-Message: Printer offline: <detalle>`
  - Body: `{ "error": "Printer offline", "printer": { ... } }`
- Si la impresora no está lista (papel atascado, sin papel/toner, etc.), `POST /write` devuelve `503 Service Unavailable` con:
  - Header: `X-Printer-Status: not-ready`
  - Header: `X-Printer-Code: PRN_NOT_READY`
  - Header: `X-Printer-Message: Printer not ready: <detalle>`
  - Body: `{ "error": "Printer not ready", "printer": { ... } }`
- Errores generales de impresión (spooler): `500 Internal Server Error` con:
  - Header: `X-Printer-Status: error`
  - Header: `X-Printer-Code: PRN_ERROR`
  - Header: `X-Printer-Message: Print error: <detalle>`
  - Body: `{ "error": "Print error: <detalle>" }`

Con esto, el frontend puede mostrar alertas/toasts leyendo solo los headers si lo prefiere, o el body JSON.

---

## 🔧 Configuración

### CORS

El servicio acepta peticiones desde los siguientes orígenes:

- `https://www.en-control.app`
- `https://en-control.app` (Añadido para evitar errores de bloqueo)
- `http://localhost:8100` (Ionic)
- `http://localhost:4200` (Angular)

### Persistencia y Reinicio

El instalador configura el servicio con una dependencia de la **Cola de Impresión (Spooler)** de Windows. Esto garantiza que el servicio arranque correctamente de forma automática al encender la PC, esperando a que el sistema de impresión esté listo.

### Ocultar ventana en Windows

El ejecutable se compila con `#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]`, por lo que no muestra ventana de consola al ejecutarse.

---

## 🛠️ Desarrollo

### Ejecutar en modo desarrollo

```bash
# Con logs en tiempo real (Windows PowerShell/CMD)
set RUST_LOG=info
cargo run

# Si el puerto 9100 está ocupado, usa otro:
set ENCONTROL_PORT=9200
cargo run
```

### Tests

```bash
cargo test
```

### Compilar para otra plataforma

```bash
# Desde Windows, compilar para macOS
rustup target add x86_64-apple-darwin
cargo build --release --target x86_64-apple-darwin

# Desde cualquier OS, compilar para Windows
rustup target add x86_64-pc-windows-msvc
cargo build --release --target x86_64-pc-windows-msvc
```

---

## 📋 Compatibilidad

### Sistemas Operativos

- ✅ Windows 10/11 (64-bit)
- ✅ macOS 10.15+ (Intel y Apple Silicon)
- ✅ Linux (experimental)

### Impresoras

- ✅ Cualquier impresora instalada en el sistema
- ✅ Impresoras térmicas (Zebra, Star, Epson, Brother, etc.)
- ✅ Impresoras de oficina estándar
- ✅ Impresoras de red

---

## 🔐 Seguridad

- ✅ Solo escucha en `localhost` (no expuesto a la red)
- ✅ CORS configurado para dominios específicos
- ✅ Sin almacenamiento de datos sensibles
- ✅ Código compilado sin runtime externo (Rust seguro)

---

## 🤝 Contribuir

1. Fork el proyecto
2. Crea una rama: `git checkout -b feature/nueva-funcionalidad`
3. Commit: `git commit -am 'Añadir nueva funcionalidad'`
4. Push: `git push origin feature/nueva-funcionalidad`
5. Abre un Pull Request

---

## 📄 Licencia

MIT License - ver [LICENSE](LICENSE) para más detalles

---

## 🆘 Soporte

- 📧 Email: soporte@en-control.app
- 🐛 Issues: [GitHub Issues](https://github.com/tu-usuario/en-control-print-service/issues)
- 📚 Documentación: [Wiki](https://github.com/tu-usuario/en-control-print-service/wiki)

---

## 🙏 Agradecimientos

- [Actix-Web](https://actix.rs/) - Framework web ultra-rápido
- [Printers](https://crates.io/crates/printers) - Librería de detección de impresoras
- Comunidad Rust 🦀

# printer-zebra
