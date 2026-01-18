# EnControl — Servicio de Impresión (Rust)

**Resumen breve**

EnControl es un servicio de impresión desarrollado en Rust para gestionar colas y comunicación con impresoras desde aplicaciones cliente. Está diseñado para ejecutarse como servicio/demonio en Windows y macOS, con foco en robustez, reintentos automáticos y actualización segura.

**Tecnologías**

- Lenguaje: Rust
- Ver `Cargo.toml` para dependencias y versiones
- Plataformas objetivo: Windows (servicio), macOS (daemon)

**Características principales**

- Servicio/demonio que recibe y gestiona trabajos de impresión.
- Manejo de colas, reintentos y logging persistente.
- Soporte para actualización automática (auto-update).
- Scripts y recursos para crear instaladores y desplegar en Windows/macOS.

**Archivos clave**

- `src/main.rs` — punto de entrada y arranque del servicio.
- `src/printer.rs` — lógica de comunicación con impresoras y adaptadores.
- `src/updater.rs` — manejo de actualizaciones automáticas.
- `extras/` — scripts de instalación para Windows y macOS.
- `installer/` — recursos para generación de instaladores.

**Cómo compilar y probar (resumen rápido)**

1. Compilar en modo release:

```bash
cargo build --release
```

2. Ejecutar localmente para pruebas (Windows ejemplo):

```powershell
target\\release\\en-control-print-service.exe
```

3. Instalación como servicio:

- Windows: usar `extras/windows/install-manual.bat` o los instaladores generados.
- macOS: ver `extras/macos/install-service.sh`.

Revisa `Cargo.toml` y los scripts en `extras/` para parámetros y opciones adicionales.

**Retos y soluciones técnicas**

- Adaptación a múltiples controladores: se creó una capa de abstracción para adaptadores de impresora.
- Robustez como servicio: logging persistente, reintentos y manejo de fallos para evitar pérdida de trabajos.
- Actualizaciones: validación de binarios y proceso seguro de reemplazo para minimizar tiempo de inactividad.

**Impacto / Resultados**

- Servicio estable para entornos con impresión continua.
- Reducción de errores por reinicios/crashes del proceso gracias a reintentos y persistencia.
- Flujo de despliegue y actualización más sencillo para entornos cliente.

**Sobre mí — Henry Gabriel M.**

- Perfil: Full Stack Development
- Ubicación: Naguanagua, Venezuela
- Disponibilidad: Más de 30 hrs/semana
- Idiomas: Español (Nativo/Bilingüe), Inglés (Fluido)
- Verificaciones: ID verificado, GitHub desde 2018
- Educación: Universidad Nacional Abierta; Colegio Universitario de Administración y Mercadeo

Puedes enlazar mi perfil de Upwork y GitHub para ver más trabajos y el historial del repositorio.

---

_Archivo generado para portafolio: `PROJECT-ENCONTROL.md`_
