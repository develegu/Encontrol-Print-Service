#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use actix_web::http::header::{HeaderName, HeaderValue};
use actix_cors::Cors;
use serde::{Deserialize, Serialize};
use log::{info, error};
use std::env;
use std::time::Duration;

#[cfg(windows)]
use windows_service::{
    define_windows_service,
    service::{
        ServiceControl, ServiceControlAccept, ServiceExitCode, ServiceState, ServiceStatus,
        ServiceType,
    },
    service_control_handler::ServiceControlHandlerResult,
};

mod printer;
mod updater;

#[cfg(windows)]
define_windows_service!(ffi_service_main, my_service_main);

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

#[derive(Debug, Deserialize)]
struct StatusQuery {
    name: Option<String>,
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

/// GET /status - Devuelve estado de la impresora (online/ready)
async fn get_printer_status_api(query: web::Query<StatusQuery>) -> HttpResponse {
    info!("📡 Petición recibida: GET /status");
    let target_name = if let Some(ref n) = query.name {
        n.clone()
    } else {
        match printer::get_system_printers() {
            Ok(printers) => {
                if let Some(first) = printers.first() { first.clone() } else { String::new() }
            }
            Err(e) => {
                error!("❌ Error obteniendo impresoras: {}", e);
                return HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": format!("Error: {}", e)
                }));
            }
        }
    };

    if target_name.is_empty() {
        return HttpResponse::NotFound().json(serde_json::json!({
            "error": "No printers available"
        }));
    }

    match printer::get_printer_status(&target_name) {
        Ok(status) => HttpResponse::Ok().json(status),
        Err(e) => {
            error!("❌ Error obteniendo estado: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Status error: {}", e)
            }))
        }
    }
}

/// GET /status-page - Página HTML con el estado de impresoras
/// Si se pasa `?name=...`, muestra solo esa; si no, lista todas.
async fn get_printer_status_page(query: web::Query<StatusQuery>) -> HttpResponse {
    let title = "En-Control Print Service - Estados de impresoras";

    let mut html = String::new();
    html.push_str("<!DOCTYPE html><html><head><meta charset=\"utf-8\">\n");
    html.push_str(&format!("<title>{}</title>", title));
    html.push_str("<style>body{font-family:sans-serif;margin:24px} .ok{color:#16a34a} .warn{color:#dc2626} .box{border:1px solid #e5e7eb;padding:16px;border-radius:8px;margin-bottom:12px} .small{color:#6b7280;font-size:12px} .grid{display:grid;grid-template-columns:repeat(auto-fill,minmax(280px,1fr));gap:12px}</style></head><body>\n");
    html.push_str(&format!("<h2>{}</h2>", title));

    if let Some(ref name) = query.name {
        // Vista de una sola impresora
        match printer::get_printer_status(name) {
            Ok(st) => {
                let cls = if st.ready {"ok"} else {"warn"};
                html.push_str(&format!("<div class=\"box\"><h3>Impresora: {}</h3>", st.name));
                html.push_str(&format!("<p class=\"{}\"><strong>Estado:</strong> {} (online: {}, ready: {})</p>", cls, st.status_text, st.online, st.ready));
                html.push_str(&format!("<p class=\"small\">flags: {}</p>", st.status_flags));
                html.push_str("</div>");
            }
            Err(e) => {
                html.push_str(&format!("<div class=\"box warn\">Error obteniendo estado de '{}': {}</div>", name, e));
            }
        }
    } else {
        // Vista de todas las impresoras físicas filtradas por get_system_printers()
        match printer::get_system_printers() {
            Ok(printers) => {
                if printers.is_empty() {
                    html.push_str("<div class=\"box warn\">No hay impresoras disponibles.</div>");
                } else {
                    html.push_str("<div class=\"grid\">");
                    for p in printers {
                        match printer::get_printer_status(&p) {
                            Ok(st) => {
                                let cls = if st.ready {"ok"} else {"warn"};
                                html.push_str(&format!("<div class=\"box\"><h3>{}</h3>", st.name));
                                html.push_str(&format!("<p class=\"{}\"><strong>Estado:</strong> {}<br><span class=\"small\">online: {}, ready: {}</span></p>", cls, st.status_text, st.online, st.ready));
                                html.push_str(&format!("<p class=\"small\">flags: {}</p>", st.status_flags));
                                html.push_str("</div>");
                            }
                            Err(e) => {
                                html.push_str(&format!("<div class=\"box warn\"><h3>{}</h3><p>Error obteniendo estado: {}</p></div>", p, e));
                            }
                        }
                    }
                    html.push_str("</div>");
                }
            }
            Err(e) => {
                html.push_str(&format!("<div class=\"box warn\">Error obteniendo impresoras del sistema: {}</div>", e));
            }
        }
    }

    html.push_str("<p class=\"small\">Endpoints: <a href=\"/available\">/available</a> · <a href=\"/status\">/status</a></p>");
    html.push_str("</body></html>");

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

/// POST /write - Envía datos ZPL a imprimir
async fn write_to_printer(body: web::Bytes) -> HttpResponse {
    info!("📡 Petición recibida: POST /write");
    info!("📊 Bytes recibidos: {}", body.len());

    // Intentar deserializar el JSON manualmente (ignora Content-Type)
    let req: PrintRequest = match serde_json::from_slice(&body) {
        Ok(v) => v,
        Err(e) => {
            error!("❌ Error deserializando JSON: {}", e);
            // Intentar ver si es texto plano (para debug)
            if let Ok(text) = std::str::from_utf8(&body) {
                info!("📝 Cuerpo recibido (no JSON): {}", text);
            }
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid JSON format"
            }));
        }
    };
    
    // Log de los primeros 300 caracteres del ZPL para debugging
    let preview = req.data.chars().take(300).collect::<String>();
    info!("📋 Preview ZPL: {}", preview);
    
    // Determinar qué impresora usar
    let printer_name = if let Some(ref device) = req.device {
        info!("🎯 Impresora especificada: {}", device.uid);
        device.uid.clone()
    } else {
        info!("🔍 No se especificó impresora, buscando primera disponible...");
        // Usar la primera impresora disponible
        match printer::get_system_printers() {
            Ok(printers) => {
                if let Some(first) = printers.first() {
                    info!("✅ Usando primera impresora: {}", first);
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
    
    // Pre-chequeo de estado de impresora antes de imprimir
    match printer::get_printer_status(&printer_name) {
        Ok(st) => {
            if !st.online {
                info!("⚠️ Impresora offline: {} - {}", printer_name, st.status_text);
                let mut resp = HttpResponse::Conflict();
                resp.insert_header((HeaderName::from_static("x-printer-status"), HeaderValue::from_static("offline")));
                resp.insert_header((HeaderName::from_static("x-printer-code"), HeaderValue::from_static("PRN_OFFLINE")));
                let msg = format!("Printer offline: {}", st.status_text);
                resp.insert_header((HeaderName::from_static("x-printer-message"), HeaderValue::from_str(&msg).unwrap_or(HeaderValue::from_static("Printer offline"))));
                return resp.json(serde_json::json!({
                    "error": "Printer offline",
                    "printer": st
                }));
            }
            if !st.ready {
                info!("⚠️ Impresora no lista: {} - {}", printer_name, st.status_text);
                let mut resp = HttpResponse::ServiceUnavailable();
                resp.insert_header((HeaderName::from_static("x-printer-status"), HeaderValue::from_static("not-ready")));
                resp.insert_header((HeaderName::from_static("x-printer-code"), HeaderValue::from_static("PRN_NOT_READY")));
                let msg = format!("Printer not ready: {}", st.status_text);
                resp.insert_header((HeaderName::from_static("x-printer-message"), HeaderValue::from_str(&msg).unwrap_or(HeaderValue::from_static("Printer not ready"))));
                return resp.json(serde_json::json!({
                    "error": "Printer not ready",
                    "printer": st
                }));
            }
        }
        Err(e) => {
            error!("❌ No se pudo obtener estado de impresora en {}: {}", if cfg!(windows) { "Windows" } else { "Mac" }, e);
            // No bloqueamos la impresión si el estado no se pudo leer, pero informamos
        }
    }

    // Enviar a spooler
    match printer::print_raw(&printer_name, &req.data) {
        Ok(_) => {
            info!("✅ Impresión enviada correctamente");
            HttpResponse::Ok().body("") // Zebra espera respuesta vacía 200 OK
        }
        Err(e) => {
            error!("❌ Error imprimiendo: {}", e);
            let mut resp = HttpResponse::InternalServerError();
            resp.insert_header((HeaderName::from_static("x-printer-status"), HeaderValue::from_static("error")));
            resp.insert_header((HeaderName::from_static("x-printer-code"), HeaderValue::from_static("PRN_ERROR")));
            let msg = format!("Print error: {}", e);
            resp.insert_header((HeaderName::from_static("x-printer-message"), HeaderValue::from_str(&msg).unwrap_or(HeaderValue::from_static("Print error"))));
            resp.json(serde_json::json!({
                "error": msg,
                "version": env!("CARGO_PKG_VERSION")
            }))
        }
    }
}

/// POST /read - Endpoint dummy para compatibilidad con Zebra Browser Print (polling)
async fn read_from_printer() -> HttpResponse {
    info!("📡 Petición recibida: POST /read (Dummy)");
    // Devolvemos 200 OK vacio para que la librería no falle con 404.
    // Si la app espera una respuesta del comando (ej: nombre del modelo), 
    // al recibir vacío usará el fallback.
    HttpResponse::Ok().body("")
}

/// GET /hello - Health check (igual que Zebra Browser Print)
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().body(concat!("En-Control Print Service v", env!("CARGO_PKG_VERSION")))
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

// Lógica del servidor web (separada para usar en servicio y consola)
async fn run_server() -> std::io::Result<()> {
    // Permitir configurar puerto por variable de entorno ENCONTROL_PORT
    let port: u16 = std::env::var("ENCONTROL_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(9100); // Puerto por defecto
    let bind_address = format!("127.0.0.1:{}", port);

    info!("🚀 ===========================================");
    info!("🚀 En-Control Print Service v{}", env!("CARGO_PKG_VERSION"));
    info!("🚀 ===========================================");
    info!("📡 Escuchando en: http://{}", bind_address);

    // Iniciar ciclo de auto-actualización en segundo plano cada 6 horas
    tokio::spawn(async {
        updater::auto_update_loop().await;
    });

    info!("🔧 Endpoints disponibles:");
    info!("   GET  /hello      - Health check");
    info!("   GET  /available  - Lista de impresoras");
    info!("   GET  /default    - Impresora por defecto");
    info!("   GET  /status     - Estado de impresora");
    info!("   GET  /status-page- Página HTML con estado");
    info!("   POST /write      - Imprimir datos ZPL");
    info!("   GET  /update     - Verificar actualizaciones");
    info!("🚀 ===========================================");

    let server = HttpServer::new(|| {
        // Configurar CORS para entornos de producción y desarrollo
        let cors = Cors::default()
            .allowed_origin("https://www.en-control.app")
            .allowed_origin("https://en-control.app") // Corregido: Agregado origen sin www
            .allowed_origin("http://localhost:4200")
            .allowed_origin("http://127.0.0.1:4200")
            .allowed_origin("http://localhost:8100")
            .allowed_origin("http://127.0.0.1:8100")
            .allowed_methods(vec!["GET", "POST", "OPTIONS"])
            .allowed_headers(vec![
                actix_web::http::header::AUTHORIZATION,
                actix_web::http::header::ACCEPT,
                actix_web::http::header::CONTENT_TYPE,
            ])
            .max_age(3600);

        App::new()
            .app_data(web::PayloadConfig::new(50 * 1024 * 1024)) // 50 MB

            .wrap(cors)
            .wrap(middleware::Logger::default())
            .route("/hello", web::get().to(health_check))
            .route("/available", web::get().to(get_available_printers))
            .route("/default", web::get().to(get_default_printer))
            .route("/status", web::get().to(get_printer_status_api))
            .route("/status-page", web::get().to(get_printer_status_page))
            .route("/write", web::post().to(write_to_printer))
            .route("/read", web::post().to(read_from_printer))
            .route("/update", web::get().to(check_updates))
    });

    match server.bind(&bind_address) {
        Ok(s) => s.run().await,
        Err(e) => {
            error!("❌ No se pudo enlazar al puerto {}: {}", bind_address, e);
            error!("Sugerencia: establezca ENCONTROL_PORT a otro valor (p.ej. 9200) y vuelva a ejecutar.");
            Err(e)
        }
    }
}

#[cfg(windows)]
fn my_service_main(arguments: Vec<std::ffi::OsString>) {
    if let Err(_e) = run_service(arguments) {
        // Handle error in some way.
    }
}

#[cfg(windows)]
fn run_service(_arguments: Vec<std::ffi::OsString>) -> windows_service::Result<()> {
    let (shutdown_tx, shutdown_rx) = std::sync::mpsc::channel();
    
    let event_handler = move |control_event| -> windows_service::service_control_handler::ServiceControlHandlerResult {
        match control_event {
            ServiceControl::Stop => {
                // Signal shutdown
                let _ = shutdown_tx.send(());
                ServiceControlHandlerResult::NoError
            }
            ServiceControl::Interrogate => ServiceControlHandlerResult::NoError,
            _ => ServiceControlHandlerResult::NotImplemented,
        }
    };

    // Register system service event handler
    let status_handle = windows_service::service_control_handler::register("EnControlPrintService", event_handler)?;

    // Tell the system that the service is running
    let next_status = ServiceStatus {
        service_type: ServiceType::OWN_PROCESS,
        current_state: ServiceState::Running,
        controls_accepted: ServiceControlAccept::STOP,
        exit_code: ServiceExitCode::Win32(0),
        checkpoint: 0,
        wait_hint: Duration::default(),
        process_id: None,
    };

    status_handle.set_service_status(next_status)?;
    info!("✅ Service Started and Running");

    // Spawn the Actix server in a separate thread because block_on blocks the current thread
    // We need the current thread to wait for the shutdown signal from the event_handler
    let _server_handle = std::thread::spawn(|| {
        if let Err(e) = actix_web::rt::System::new().block_on(run_server()) {
             error!("Error running server: {}", e);
        }
    });

    // Wait for the shutdown signal from the ServiceControl handler
    let _ = shutdown_rx.recv();
    info!("🛑 Shutdown signal received. Stopping service...");

    // Can't easily stop the Actix server from here without a handle to the system, 
    // but since we are exiting the process soon, it's acceptable for a simple service.
    // Ideally, we would have passed a Server handle, but for now, we just proceed to report stopped.
    
    // Tell the system that the service has stopped
    let stop_status = ServiceStatus {
        service_type: ServiceType::OWN_PROCESS,
        current_state: ServiceState::Stopped,
        controls_accepted: ServiceControlAccept::empty(),
        exit_code: ServiceExitCode::Win32(0),
        checkpoint: 0,
        wait_hint: Duration::default(),
        process_id: None,
    };
    
    status_handle.set_service_status(stop_status)?;
    info!("✅ Service Stopped");

    Ok(())
}

fn main() -> std::io::Result<()> {
    // Inicializar logger
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    // Check if running as a windows service
    #[cfg(windows)]
    match windows_service::service_dispatcher::start("EnControlPrintService", ffi_service_main) {
        Ok(_) => Ok(()),
        Err(_) => {
            // If failing to start as service, run as console app (e.g. debugging)
            info!("Running as console application");
            actix_web::rt::System::new().block_on(run_server())
        }
    }

    #[cfg(not(windows))]
    {
        actix_web::rt::System::new().block_on(run_server())
    }
}
