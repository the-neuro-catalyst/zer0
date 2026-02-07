use reader::engine::file_engine::{FileReaderOptions, read_file_content};

use reader::output::{OutputFormat, OutputMode};

use reader::reader_result::DataReaderResult;

use std::fs::File;

use std::io::Write;

use tempfile::tempdir;

#[tokio::test]
async fn test_secret_redaction_and_metadata() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("secrets.txt");
    let mut file = File::create(&file_path).unwrap();

    let content = "My AWS key is AKIA1234567890123456 and my email is test@example.com";
    writeln!(file, "{}", content).unwrap();

    let options = FileReaderOptions {
        head: None,
        file_type_override: Some("txt".to_string()),
        output_mode: OutputMode::Default,
        output_format: OutputFormat::Text,
        pii_redaction: true,
        zero_copy: true,
        recursive: false,
        filter_exts: None,
        output_path: None,
    };

    let result = read_file_content(&file_path, options).await.unwrap();

    if let DataReaderResult::Text(data, metadata) = result {
        assert!(metadata.compromised);
        assert!(!data.content.contains("AKIA1234567890123456"));
        assert!(!data.content.contains("test@example.com"));
        assert!(metadata.information_density.unwrap() > 0.0);
    } else {
        panic!("Expected DataReaderResult::Text");
    }
}

#[tokio::test]
async fn test_json_depth_and_entropy() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("nested.json");
    let mut file = File::create(&file_path).unwrap();

    let content = r#"{"a": {"b": {"c": {"d": 1}}}}"#;
    writeln!(file, "{}", content).unwrap();

    let options = FileReaderOptions {
        head: None,
        file_type_override: None,
        output_mode: OutputMode::Default,
        output_format: OutputFormat::Json,
        pii_redaction: false,
        zero_copy: true,
        recursive: false,
        filter_exts: None,
        output_path: None,
    };

    let result = read_file_content(&file_path, options).await.unwrap();

    if let DataReaderResult::Json(_, metadata) = result {
        assert_eq!(metadata.structural_depth, Some(4));
        assert!(metadata.information_density.unwrap() > 0.0);
        assert!(!metadata.compromised);
    } else {
        panic!("Expected DataReaderResult::Json");
    }
}

#[tokio::test]
async fn test_toml_pii_redaction() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("config.toml");
    let mut file = File::create(&file_path).unwrap();

    let content = r#"[database]
url = "postgres://admin:password123@localhost:5432/db"
token = "ghp_abcdefghijklmnopqrstuvwxyz1234567890abcdef"

[api]
key = "AIzaSyA1234567890-abcdefghijklmnopqrstuvwxyzABC"
secret = "sk_live_1234567890abcdefghijklmnopqrstuvwxyz"
password = "super-secret-password-123456"
"#;
    writeln!(file, "{}", content).unwrap();

    let options = FileReaderOptions {
        head: None,
        file_type_override: None,
        output_mode: OutputMode::Default,
        output_format: OutputFormat::Json,
        pii_redaction: true,
        zero_copy: true,
        recursive: false,
        filter_exts: None,
        output_path: None,
    };

    let result = read_file_content(&file_path, options).await.unwrap();

    if let DataReaderResult::Toml(data, metadata) = result {
        assert!(metadata.compromised, "Metadata should mark TOML as compromised");
        let preview = serde_json::to_string(&data.value).unwrap();

        assert!(
            !preview.contains("ghp_abcdefghijklmnopqrstuvwxyz1234567890abcdef"),
            "GitHub token should be redacted"
        );
        assert!(
            !preview.contains("AIzaSyA1234567890-abcdefghijklmnopqrstuvwxyzABC"),
            "GCP key should be redacted"
        );
        assert!(
            !preview.contains("sk_live_1234567890abcdefghijklmnopqrstuvwxyz"),
            "Stripe key should be redacted"
        );
        assert!(!preview.contains("password123"), "Database password should be redacted");
        assert!(
            !preview.contains("super-secret-password-123456"),
            "Generic password should be redacted"
        );
    } else {
        panic!("Expected DataReaderResult::Toml");
    }
}
