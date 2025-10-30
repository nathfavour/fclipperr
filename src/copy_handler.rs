use std::io::{self, Write};

pub enum CopyStrategy {
    Local,
    Remote,
}

pub fn detect_environment() -> CopyStrategy {
    if std::env::var("SSH_TTY").is_ok() 
        || std::env::var("SSH_CONNECTION").is_ok()
        || std::env::var("SSH_CLIENT").is_ok() {
        CopyStrategy::Remote
    } else {
        CopyStrategy::Local
    }
}

pub fn copy_to_clipboard_local(content: &str) -> Result<(), String> {
    #[cfg(not(target_os = "unknown"))]
    {
        use arboard::Clipboard;
        
        let mut clipboard = Clipboard::new()
            .map_err(|e| format!("Failed to access clipboard: {}", e))?;
        
        clipboard.set_text(content)
            .map_err(|e| format!("Failed to copy to clipboard: {}", e))?;
        
        Ok(())
    }
    
    #[cfg(target_os = "unknown")]
    Err("Local clipboard not supported on this platform".to_string())
}

pub fn copy_to_clipboard_remote(content: &str) -> Result<(), String> {
    let encoded = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, content.as_bytes());
    
    let osc52_sequence = format!("\x1b]52;c;{}\x07", encoded);
    
    io::stdout()
        .write_all(osc52_sequence.as_bytes())
        .map_err(|e| format!("Failed to write OSC 52 sequence: {}", e))?;
    
    io::stdout()
        .flush()
        .map_err(|e| format!("Failed to flush stdout: {}", e))?;
    
    Ok(())
}

pub fn copy_content(content: &str, strategy: CopyStrategy) -> Result<(), String> {
    match strategy {
        CopyStrategy::Local => copy_to_clipboard_local(content),
        CopyStrategy::Remote => copy_to_clipboard_remote(content),
    }
}
