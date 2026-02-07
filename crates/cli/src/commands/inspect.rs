// crates/cli/src/commands/audit.rs

use anyhow::Result;

use console::{Emoji, Style};

use reader::engine::file_engine::FileReaderOptions;

use reader::output::{OutputFormat, OutputMode};

use std::path::PathBuf;

use tracing::{error, info, warn};

pub async fn run(data_path: Option<PathBuf>, text: Option<String>, detailed: bool) -> Result<()> {
    let target = if let Some(path) = data_path {
        path.to_string_lossy().to_string()
    } else if let Some(t) = text {
        // Create temporary file for text audit
        let temp_path = std::env::temp_dir().join("zero_audit_stream.txt");
        std::fs::write(&temp_path, t)?;
        temp_path.to_string_lossy().to_string()
    } else {
        // This case is technically handled by clap ArgGroup, but we keep it for safety.
        return Err(anyhow::anyhow!("No audit target provided (path or text)."));
    };

    let cyan = Style::new().cyan().bold();
    let magenta = Style::new().magenta().bold();
    let yellow = Style::new().yellow().bold();

    println!("\n{} {}", Emoji("🔍 ", ""), cyan.apply_to("Initiating security baseline audit..."));
    println!("{}", magenta.apply_to("-------------------------------------------"));

    let options = FileReaderOptions {
        head: if detailed { None } else { Some(500) },
        file_type_override: None,
        output_mode: OutputMode::Default,
        output_format: OutputFormat::Json,
        pii_redaction: false, // We WANT to see PII during audit
        zero_copy: true,
        recursive: detailed,
        filter_exts: None,
        output_path: None,
    };

    match reader::engine::router::read_source(&target, None, options).await {
        Ok(result) => {
            let metadata = result.get_metadata();
            if metadata.compromised {
                warn!("{} SECURITY ALERT: Sensitive data signatures detected!", Emoji("⚠️ ", ""));
                println!(
                    "{}",
                    yellow.apply_to(">>> VULNERABILITIES IDENTIFIED IN RESOURCE STRUCTURE")
                );
            } else {
                info!("Integrity Check: Pass. No known sensitive patterns found.");
                println!(
                    "{} {}",
                    Emoji("✅ ", ""),
                    Style::new().green().apply_to("No security violations detected.")
                );
            }

            println!("\nMetric Analysis:");
            println!(" - Entropy Score: {:.4}", metadata.information_density.unwrap_or(0.0));
            println!(" - Structural Depth: {}", metadata.structural_depth.unwrap_or(0));
        }
        Err(e) => {
            error!("Audit Failed: {}", e);
            return Err(e.into());
        }
    }

    Ok(())
}
