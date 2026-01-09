use printers::get_printers;
use std::error::Error;
use serde::Serialize;

#[cfg(target_os = "windows")]
use windows::{
    core::{PCWSTR, PWSTR},
    Win32::Foundation::HANDLE,
    Win32::Graphics::Printing::{
        OpenPrinterW, StartDocPrinterW, StartPagePrinter, WritePrinter, 
        EndPagePrinter, EndDocPrinter, ClosePrinter, GetPrinterW,
        PRINTER_DEFAULTSW, DOC_INFO_1W, PRINTER_ACCESS_USE, PRINTER_INFO_2W,
        PRINTER_STATUS_ERROR, PRINTER_STATUS_OFFLINE, PRINTER_STATUS_PAPER_JAM,
        PRINTER_STATUS_PAPER_OUT, PRINTER_STATUS_DOOR_OPEN, PRINTER_STATUS_NO_TONER,
        PRINTER_STATUS_OUT_OF_MEMORY, PRINTER_STATUS_NOT_AVAILABLE, PRINTER_STATUS_INITIALIZING,
        PRINTER_STATUS_WARMING_UP, PRINTER_STATUS_POWER_SAVE, PRINTER_STATUS_SERVER_UNKNOWN,
        PRINTER_STATUS_PAUSED
    },
};

/// Obtiene lista de impresoras del sistema
pub fn get_system_printers() -> std::result::Result<Vec<String>, Box<dyn Error>> {
    let printers = get_printers();
    
    let printer_names: Vec<String> = printers
        .into_iter()
        .map(|p| p.name)
        .filter(|name| {
            let n = name.to_lowercase();
            // Filtro negativo: Excluir impresoras virtuales conocidas
            !n.contains("pdf") && 
            !n.contains("xps") && 
            !n.contains("onenote") && 
            !n.contains("fax") &&
            !n.contains("microsoft print")
        })
        .collect();
    
    // Devolvemos la lista aunque esté vacía (Ok) para evitar Error 500 en el frontend
    Ok(printer_names)
}

#[derive(Debug, Serialize)]
pub struct PrinterStatus {
    pub name: String,
    pub online: bool,
    pub ready: bool,
    pub status_flags: u32,
    pub status_text: String,
}

#[cfg(target_os = "windows")]
pub fn get_printer_status(printer_name: &str) -> std::result::Result<PrinterStatus, Box<dyn Error>> {
    unsafe {
        // Abrir impresora
        let device_name: Vec<u16> = printer_name.encode_utf16().chain(std::iter::once(0)).collect();
        let mut h_printer = HANDLE::default();
        let mut defaults = PRINTER_DEFAULTSW {
            pDatatype: PWSTR::null(),
            pDevMode: std::ptr::null_mut(),
            DesiredAccess: PRINTER_ACCESS_USE,
        };
        OpenPrinterW(PCWSTR(device_name.as_ptr()), &mut h_printer, Some(&mut defaults))?;

        struct PrinterHandle(HANDLE);
        impl Drop for PrinterHandle { fn drop(&mut self) { unsafe { let _ = ClosePrinter(self.0); } } }
        let _guard = PrinterHandle(h_printer);

        // Obtener tamaño necesario para PRINTER_INFO_2W
        let mut needed: u32 = 0;
        // Primera llamada sin buffer: esperamos ERROR_INSUFFICIENT_BUFFER, que rellena 'needed'
        let _ = GetPrinterW(h_printer, 2, None, &mut needed);
        if needed == 0 {
            // Si no indicó tamaño, devolver error
            return Err(Box::new(windows::core::Error::from_win32()));
        }
        let mut buf: Vec<u8> = vec![0u8; needed as usize];
        GetPrinterW(h_printer, 2, Some(buf.as_mut_slice()), &mut needed)?;

        let info: &PRINTER_INFO_2W = &*(buf.as_ptr() as *const PRINTER_INFO_2W);
        let flags = info.Status;

        // Evaluar estado
        let offline = (flags & PRINTER_STATUS_OFFLINE) != 0;
        let has_error = (flags & (
            PRINTER_STATUS_ERROR |
            PRINTER_STATUS_PAPER_JAM |
            PRINTER_STATUS_PAPER_OUT |
            PRINTER_STATUS_DOOR_OPEN |
            PRINTER_STATUS_NO_TONER |
            PRINTER_STATUS_OUT_OF_MEMORY |
            PRINTER_STATUS_NOT_AVAILABLE |
            PRINTER_STATUS_INITIALIZING |
            PRINTER_STATUS_WARMING_UP |
            PRINTER_STATUS_POWER_SAVE |
            PRINTER_STATUS_SERVER_UNKNOWN |
            PRINTER_STATUS_PAUSED
        )) != 0;

        let mut parts: Vec<&str> = Vec::new();
        if offline { parts.push("offline"); }
        if has_error { parts.push("error"); }
        if parts.is_empty() { parts.push("idle/ready"); }
        let status_text = parts.join(", ");

        Ok(PrinterStatus {
            name: printer_name.to_string(),
            online: !offline,
            ready: !offline && !has_error,
            status_flags: flags,
            status_text,
        })
    }
}

#[cfg(target_os = "macos")]
pub fn get_printer_status(printer_name: &str) -> std::result::Result<PrinterStatus, Box<dyn Error>> {
    use std::process::Command;
    let out = Command::new("lpstat").arg("-p").arg(printer_name).output()?;
    let stdout = String::from_utf8_lossy(&out.stdout).to_lowercase();
    let online = stdout.contains("printer ") && !stdout.contains("disabled");
    let ready = stdout.contains("idle") || stdout.contains("printing");
    let status_text = stdout.trim().to_string();
    Ok(PrinterStatus {
        name: printer_name.to_string(),
        online,
        ready,
        status_flags: 0,
        status_text,
    })
}

/// Envía datos RAW a la impresora (ZPL, ESC/POS, etc.)
#[cfg(target_os = "windows")]
pub fn print_raw(printer_name: &str, data: &str) -> std::result::Result<(), Box<dyn Error>> {
    use log::{info, error};

    info!("🖨️  Enviando trabajo RAW a: {}", printer_name);

    unsafe {
        match print_raw_unsafe(printer_name, data) {
            Ok(_) => {
                info!("✅ Impresión enviada correctamente a spooler");
                Ok(())
            },
            Err(e) => {
                error!("❌ Error enviando a spooler: {:?}", e);
                Err(Box::new(e))
            }
        }
    }
}

#[cfg(target_os = "windows")]
unsafe fn print_raw_unsafe(printer_name: &str, data: &str) -> windows::core::Result<()> {
    // 1. Abrir la impresora
    let device_name: Vec<u16> = printer_name.encode_utf16().chain(std::iter::once(0)).collect();
    
    let mut h_printer = HANDLE::default();
    
    let mut defaults = PRINTER_DEFAULTSW {
        pDatatype: PWSTR::null(),
        pDevMode: std::ptr::null_mut(),
        DesiredAccess: PRINTER_ACCESS_USE,
    };

    OpenPrinterW(PCWSTR(device_name.as_ptr()), &mut h_printer, Some(&mut defaults))?;

    struct PrinterHandle(HANDLE);
    impl Drop for PrinterHandle {
        fn drop(&mut self) {
            unsafe { 
                let _ = ClosePrinter(self.0); 
            }
        }
    }
    let _handle_guard = PrinterHandle(h_printer);

    // 2. Iniciar Documento
    let mut doc_name: Vec<u16> = "EnControl Print Job\0".encode_utf16().collect();
    let mut datatype: Vec<u16> = "RAW\0".encode_utf16().collect();
    
    let doc_info = DOC_INFO_1W {
        pDocName: PWSTR(doc_name.as_mut_ptr()),
        pOutputFile: PWSTR::null(),
        pDatatype: PWSTR(datatype.as_mut_ptr()),
    };

    // StartDocPrinterW en esta version del crate espera *const DOC_INFO_1W
    let job_id = StartDocPrinterW(h_printer, 1, &doc_info);
    if job_id == 0 {
        return Err(windows::core::Error::from_win32());
    }

    // 3. Iniciar Página
    if !StartPagePrinter(h_printer).as_bool() {
         EndDocPrinter(h_printer);
         return Err(windows::core::Error::from_win32());
    }

    // 4. Escribir Datos
    let mut written: u32 = 0;
    let data_bytes = data.as_bytes();
    
    let success = WritePrinter(
        h_printer,
        data_bytes.as_ptr() as *const std::ffi::c_void,
        data_bytes.len() as u32,
        &mut written,
    );

    if !success.as_bool() {
        EndPagePrinter(h_printer);
        EndDocPrinter(h_printer);
        return Err(windows::core::Error::from_win32());
    }

    // 5. Finalizar Página y Documento
    EndPagePrinter(h_printer);
    EndDocPrinter(h_printer);

    Ok(())
}


/// Envía datos RAW a la impresora en macOS
#[cfg(target_os = "macos")]
pub fn print_raw(printer_name: &str, data: &str) -> std::result::Result<(), Box<dyn Error>> {
    use std::process::{Command, Stdio};
    use std::io::Write;
    
    let mut child = Command::new("lpr")
        .arg("-P")
        .arg(printer_name)
        .arg("-o")
        .arg("raw")
        .stdin(Stdio::piped())
        .spawn()?;
    
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(data.as_bytes())?;
    }
    
    let status = child.wait()?;
    
    if status.success() {
        Ok(())
    } else {
        Err("Error imprimiendo con lpr".into())
    }
}
