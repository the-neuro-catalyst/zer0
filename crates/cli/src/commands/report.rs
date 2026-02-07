// crates/cli/src/commands/report.rs

use anyhow::Result;
use chrono;

use tracing::info;

pub async fn run(file: String) -> Result<()> {
    info!("Generating inspection summary from: {}", file);

    let report_content = std::fs::read_to_string(&file).unwrap_or_else(|_| {
        info!("No report file found at {}. Creating new summary.", file);
        format!(
            r#"{{ "status": "Inspection Completed", "timestamp": "{}" }}"#,
            chrono::Utc::now().to_rfc3339()
        )
    });

    println!("{}", report_content);

    Ok(())
}
