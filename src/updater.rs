use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateInfo {
    pub available: bool,
    pub current_version: String,
    pub latest_version: String,
    pub download_url: Option<String>,
}

/// Verifica e instala actualizaciones desde GitHub
pub async fn check_for_updates() -> Result<UpdateInfo, Box<dyn Error + Send + Sync>> {
    let current_version = env!("CARGO_PKG_VERSION");
    
    // Configuración de GitHub (Cambiar esto cuando tengas el repo definitivo)
    let repo_owner = "tu-usuario-o-organizacion";
    let repo_name = "en-control-print-service";

    let status = self_update::backends::github::Update::configure()
        .repo_owner(repo_owner)
        .repo_name(repo_name)
        .bin_name("en-control-print-service")
        .show_download_progress(true)
        .current_version(current_version)
        .build()?
        .get_latest_release()?;

    let available = self_update::version::bump_is_greater(current_version, &status.version)?;

    Ok(UpdateInfo {
        available,
        current_version: current_version.to_string(),
        latest_version: status.version.to_string(),
        download_url: None, // <- Corregido: La URL la maneja internamente en install_update
    })
}

/// Descarga e instala una actualización
pub async fn install_update(_download_url: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
    let current_version = env!("CARGO_PKG_VERSION");
    let repo_owner = "tu-usuario-o-organizacion";
    let repo_name = "en-control-print-service";

    self_update::backends::github::Update::configure()
        .repo_owner(repo_owner)
        .repo_name(repo_name)
        .bin_name("en-control-print-service")
        .show_download_progress(true)
        .current_version(current_version)
        .build()?
        .update()?;
    
    Ok(())
}

/// Función para auto-actualización periódica (llamar en background)
pub async fn auto_update_loop() {
    use tokio::time::{interval, Duration};
    
    let mut ticker = interval(Duration::from_secs(21600)); // Cada 6 horas
    
    loop {
        ticker.tick().await;
        
        log::info!("🔄 Verificando actualizaciones automáticas...");
        
        match check_for_updates().await {
            Ok(info) => {
                if info.available {
                    log::info!("📦 Nueva versión disponible: {}", info.latest_version);
                    
                    // La URL ya no es necesaria, self_update sabe dónde buscar
                    log::info!("⬇️  Descargando actualización...");
                    if let Err(e) = install_update("").await {
                        log::error!("❌ Error instalando actualización: {}", e);
                    } else {
                        log::info!("✅ Actualización instalada correctamente. Reinicie el servicio para aplicar.");
                    }
                } else {
                    log::info!("✅ Versión actual es la más reciente");
                }
            }
            Err(e) => {
                log::error!("❌ Error verificando actualizaciones: {}", e);
            }
        }
    }
}
