use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use actix_cors::Cors;
use serde::{Deserialize, Serialize};
use log::{info, error};
use std::env;

mod printer;
mod updater;

// Estructura compatible con Zebra Browser Print
#[derive(Debug, Serialize, Deserialize)]
struct PrinterDevice {
    #[serde(rename = "deviceType")]
    device_type: String,
    uid: String,
    provider: String,
    name: String,
    connection: String,
    version: i32,
    manufacturer: String,
}

#[derive(Debug, Serialize)]
struct PrintersResponse {
    printer: Vec<PrinterDevice>,
}

#[derive(Debug, Deserialize)]
struct PrintRequest {
    device: Option<DeviceInfo>,
    data: String,
}

#[derive(Debug, Deserialize)]
struct DeviceInfo {
    uid: String,
}

/// GET /available - Devuelve lista de impresoras (compatible con Zebra Browser Print)
async fn get_available_printers() -> HttpResponse {
    info!("📡 Petición recibida: GET /available");
    
    match printer::get_system_printers() {
        Ok(printers) => {
            // Convertir a formato compatible con Zebra
            let devices: Vec<PrinterDevice> = printers
                .iter()
                .map(|p| PrinterDevice {
                    device_type: "printer".to_string(),
                    uid: p.clone(),
                    provider: "com.zebra.ds.webdriver.desktop.provider.DefaultDeviceProvider".to_string(),
                    name: p.clone(),
                    connection: "usb".to_string(),
                    version: 3,
                    manufacturer: "System Printer".to_string(),
                })
                .collect();

            let response = PrintersResponse { printer: devices };
            
            info!("✅ Devolviendo {} impresoras", response.printer.len());
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            error!("❌ Error obteniendo impresoras: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Error obteniendo impresoras: {}", e)
            }))
        }
    }
}

/// GET /default - Devuelve impresora por defecto
async fn get_default_printer() -> HttpResponse {
    info!("📡 Petición recibida: GET /default");
    
    match printer::get_system_printers() {
        Ok(printers) => {
            if let Some(first) = printers.first() {
                let device = PrinterDevice {
                    device_type: "printer".to_string(),
                    uid: first.clone(),
                    provider: "com.zebra.ds.webdriver.desktop.provider.DefaultDeviceProvider".to_string(),
                    name: first.clone(),
                    connection: "usb".to_string(),
                    version: 3,
                    manufacturer: "System Printer".to_string(),
                };
                
                info!("✅ Impresora por defecto: {}", first);
                HttpResponse::Ok().json(device)
            } else {
                error!("❌ No hay impresoras disponibles");
                HttpResponse::NotFound().json(serde_json::json!({
                    "error": "No printers found"
                }))
            }
        }
        Err(e) => {
            error!("❌ Error obteniendo impresora por defecto: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Error: {}", e)
            }))
        }
    }
}

/// POST /write - Envía datos ZPL a imprimir
async fn write_to_printer(req: web::Json<PrintRequest>) -> HttpResponse {
    info!("📡 Petición recibida: POST /write");
    info!("📄 Datos ZPL recibidos ({} bytes)", req.data.len());
    
    // Determinar qué impresora usar
    let printer_name = if let Some(ref device) = req.device {
        device.uid.clone()
    } else {
        // Usar la primera impresora disponible
        match printer::get_system_printers() {
            Ok(printers) => {
                if let Some(first) = printers.first() {
                    first.clone()
                } else {
                    error!("❌ No hay impresoras disponibles");
                    return HttpResponse::NotFound().json(serde_json::json!({
                        "error": "No printers available"
                    }));
                }
            }
            Err(e) => {
                error!("❌ Error obteniendo impresoras: {}", e);
                return HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": format!("Error: {}", e)
                }));
            }
        }
    };
    
    info!("🖨️  Imprimiendo en: {}", printer_name);
    
    // Enviar a imprimir
    match printer::print_raw(&printer_name, &req.data) {
        Ok(_) => {
            info!("✅ Impresión enviada correctamente");
            HttpResponse::Ok().body("") // Zebra espera respuesta vacía 200 OK
        }
        Err(e) => {
            error!("❌ Error imprimiendo: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Print error: {}", e)
            }))
        }
    }
}

/// GET /hello - Health check (igual que Zebra Browser Print)
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().body("En-Control Print Service v0.1.0")
}

/// GET /update - Endpoint para verificar actualizaciones
async fn check_updates() -> HttpResponse {
    info!("🔄 Verificando actualizaciones...");
    
    match updater::check_for_updates().await {
        Ok(update_info) => {
            HttpResponse::Ok().json(update_info)
        }
        Err(e) => {
            error!("❌ Error verificando actualizaciones: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Update check failed: {}", e)
            }))
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Inicializar logger
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    let port = 9100; // Puerto usado por Zebra Browser Print
    let bind_address = format!("127.0.0.1:{}", port);

    info!("🚀 ===========================================");
    info!("🚀 En-Control Print Service v0.1.0");
    info!("🚀 ===========================================");
    info!("📡 Escuchando en: http://{}", bind_address);
    info!("🔧 Endpoints disponibles:");
    info!("   GET  /hello      - Health check");
    info!("   GET  /available  - Lista de impresoras");
    info!("   GET  /default    - Impresora por defecto");
    info!("   POST /write      - Imprimir datos ZPL");
    info!("   GET  /update     - Verificar actualizaciones");
    info!("🚀 ===========================================");

    HttpServer::new(|| {
        // Configurar CORS para permitir conexiones desde tu PWA
        let cors = Cors::default()
            .allowed_origin("https://www.en-control.app")
            .allowed_origin("http://localhost:8100") // Para desarrollo local
            .allowed_origin("http://localhost:4200") // Angular dev server
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                actix_web::http::header::AUTHORIZATION,
                actix_web::http::header::ACCEPT,
                actix_web::http::header::CONTENT_TYPE,
            ])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .route("/hello", web::get().to(health_check))
            .route("/available", web::get().to(get_available_printers))
            .route("/default", web::get().to(get_default_printer))
            .route("/write", web::post().to(write_to_printer))
            .route("/update", web::get().to(check_updates))
    })
    .bind(&bind_address)?
    .run()
    .await
}
