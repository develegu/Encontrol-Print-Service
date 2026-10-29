use serde::{Deserialize, Serialize};
use std::error::Error;

// Repositorio de GitHub donde el CI publica los releases.
// Los releases deben ser públicos: el servicio descarga sin token.
const REPO_OWNER: &str = "develegu";
const REPO_NAME: &str = "Encontrol-Print-Service";
const BIN_NAME: &str = "en-control-print-service";

type UpdateResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateInfo {
    pub available: bool,
    pub current_version: String,
    pub latest_version: String,
    pub download_url: Option<String>,
}

fn build_updater() -> UpdateResult<Box<dyn self_update::update::ReleaseUpdate>> {
    // El asset del release se identifica por el target triple (ej. x86_64-pc-windows-msvc),
    // ver .github/workflows/build.yml
    Ok(self_update::backends::github::Update::configure()
        .repo_owner(REPO_OWNER)
        .repo_name(REPO_NAME)
        .bin_name(BIN_NAME)
        .current_version(env!("CARGO_PKG_VERSION"))
        // Corre como servicio: sin consola, sin prompt de confirmación por stdin
        .show_download_progress(false)
        .show_output(false)
        .no_confirm(true)
        .build()?)
}

/// Consulta el último release en GitHub (bloqueante: llamar vía spawn_blocking)
fn check_for_updates_blocking() -> UpdateResult<UpdateInfo> {
    let current_version = env!("CARGO_PKG_VERSION");
    let release = build_updater()?.get_latest_release()?;
    let available = self_update::version::bump_is_greater(current_version, &release.version)?;
    let target = self_update::get_target();
    let download_url = release.asset_for(target, None).map(|a| a.download_url);

    Ok(UpdateInfo {
        available,
        current_version: current_version.to_string(),
        latest_version: release.version,
        download_url,
    })
}

/// Descarga y reemplaza el binario (bloqueante). Devuelve true si se instaló una versión nueva.
fn install_update_blocking() -> UpdateResult<bool> {
    let status = build_updater()?.update()?;
    Ok(status.updated())
}

/// Verifica si hay actualizaciones en GitHub
pub async fn check_for_updates() -> UpdateResult<UpdateInfo> {
    tokio::task::spawn_blocking(check_for_updates_blocking).await?
}

/// Descarga e instala la última actualización
pub async fn install_update() -> UpdateResult<bool> {
    tokio::task::spawn_blocking(install_update_blocking).await?
}

/// Función para auto-actualización periódica (llamar en background)
pub async fn auto_update_loop() {
    use tokio::time::{interval, sleep, Duration};

    // Dar tiempo a que el servidor arranque antes de la primera verificación
    sleep(Duration::from_secs(60)).await;

    let mut ticker = interval(Duration::from_secs(21600)); // Cada 6 horas

    loop {
        ticker.tick().await;

        log::info!("🔄 Verificando actualizaciones automáticas...");

        match check_for_updates().await {
            Ok(info) if info.available => {
                log::info!("📦 Nueva versión disponible: {} (actual: {})", info.latest_version, info.current_version);
                log::info!("⬇️  Descargando actualización...");
                match install_update().await {
                    Ok(true) => {
                        // Salimos con código de error para que el supervisor (SCM en Windows con
                        // acciones de recuperación, launchd con KeepAlive en macOS) relance el
                        // proceso con el binario nuevo.
                        log::info!("✅ Actualización a v{} instalada. Reiniciando servicio...", info.latest_version);
                        std::process::exit(1);
                    }
                    Ok(false) => log::info!("✅ Versión actual es la más reciente"),
                    Err(e) => log::error!("❌ Error instalando actualización: {}", e),
                }
            }
            Ok(_) => log::info!("✅ Versión actual es la más reciente"),
            Err(e) => log::error!("❌ Error verificando actualizaciones: {}", e),
        }
    }
}
