use printers::get_printers;
use std::error::Error;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

/// Obtiene lista de impresoras del sistema
pub fn get_system_printers() -> Result<Vec<String>, Box<dyn Error>> {
    let printers = get_printers();
    
    let printer_names: Vec<String> = printers
        .into_iter()
        .map(|p| p.name)
        .collect();
    
    if printer_names.is_empty() {
        return Err("No se encontraron impresoras en el sistema".into());
    }
    
    Ok(printer_names)
}

/// Envía datos RAW a la impresora (ZPL, ESC/POS, etc.)
#[cfg(target_os = "windows")]
pub fn print_raw(printer_name: &str, data: &str) -> Result<(), Box<dyn Error>> {
    use std::process::Command;
    use std::io::Write;
    use std::fs::File;
    
    // En Windows, crear archivo temporal y usar print command
    let temp_file = std::env::temp_dir().join("encontrol_print.zpl");
    let mut file = File::create(&temp_file)?;
    file.write_all(data.as_bytes())?;
    
    // Enviar a impresora usando comando de Windows
    const CREATE_NO_WINDOW: u32 = 0x08000000;
    
    let output = Command::new("cmd")
        .args(&["/C", &format!("print /D:\"{}\" \"{}\"", printer_name, temp_file.display())])
        .creation_flags(CREATE_NO_WINDOW)
        .output()?;
    
    // Limpiar archivo temporal
    let _ = std::fs::remove_file(&temp_file);
    
    if output.status.success() {
        Ok(())
    } else {
        Err(format!("Error imprimiendo: {}", String::from_utf8_lossy(&output.stderr)).into())
    }
}

/// Envía datos RAW a la impresora en macOS
#[cfg(target_os = "macos")]
pub fn print_raw(printer_name: &str, data: &str) -> Result<(), Box<dyn Error>> {
    use std::process::{Command, Stdio};
    use std::io::Write;
    
    // En macOS, usar lpr directamente
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_printers() {
        let result = get_system_printers();
        assert!(result.is_ok());
        
        let printers = result.unwrap();
        println!("Impresoras encontradas: {:?}", printers);
    }
}
