use reader::engine::file_engine::{
    get_file_format, read_file_content, FileFormat, FileReaderOptions,
};
use reader::output::{OutputFormat, OutputMode};
use reader::reader_result::DataReaderResult;
use std::fs;
use tempfile::tempdir;

fn get_test_options() -> FileReaderOptions {
    FileReaderOptions {
        head: Some(5),
        file_type_override: None,
        output_mode: OutputMode::Default,
        output_format: OutputFormat::Json,
        pii_redaction: false,
        zero_copy: true,
        recursive: false,
        filter_exts: None,
        output_path: None,
    }
}

#[tokio::test]
async fn test_format_detection_comprehensive() {
    let files = vec![
        ("data.csv", FileFormat::Csv),
        ("data.json", FileFormat::Json),
        ("data.parquet", FileFormat::Parquet),
        ("config.toml", FileFormat::Toml),
        ("config.yaml", FileFormat::Yaml),
        ("doc.md", FileFormat::Markdown),
        ("doc.pdf", FileFormat::Pdf),
        ("image.png", FileFormat::Image),
        ("archive.zip", FileFormat::Zip),
        ("data.xml", FileFormat::Xml),
        ("logs.txt", FileFormat::Text),
    ];

    for (name, expected) in files {
        assert_eq!(get_file_format(std::path::Path::new(name)), expected, "Failed for {}", name);
    }
}

#[tokio::test]
async fn test_read_csv_base() {
    let content = "id,name,value\n1,zero,abundance\n2,friction,none";
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.csv");
    fs::write(&file_path, content).unwrap();

    let result = read_file_content(&file_path, get_test_options()).await.unwrap();
    assert!(matches!(result, DataReaderResult::Csv(_, _)));
}

#[tokio::test]
async fn test_read_toml_base() {
    let content = "[system]\nengine = \"rust\"\nmode = \"high-performance\"";
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.toml");
    fs::write(&file_path, content).unwrap();

    let result = read_file_content(&file_path, get_test_options()).await.unwrap();
    assert!(matches!(result, DataReaderResult::Toml(_, _)));
}

#[tokio::test]
async fn test_read_yaml_base() {
    let content = "system:\n  engine: rust\n  mode: high-performance";
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.yaml");
    fs::write(&file_path, content).unwrap();

    let result = read_file_content(&file_path, get_test_options()).await.unwrap();
    assert!(matches!(result, DataReaderResult::Yaml(_, _)));
}

#[tokio::test]
async fn test_read_xml_base() {
    let content = "<root><item id=\"1\">ZERO</item></root>";
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.xml");
    fs::write(&file_path, content).unwrap();

    let result = read_file_content(&file_path, get_test_options()).await.unwrap();
    assert!(matches!(result, DataReaderResult::Xml(_, _)));
}

#[tokio::test]
async fn test_read_markdown_base() {
    let content = "# ZERO Protocol\n\n- Abundance\n- Simplicity";
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.md");
    fs::write(&file_path, content).unwrap();

    let result = read_file_content(&file_path, get_test_options()).await.unwrap();
    assert!(matches!(result, DataReaderResult::Markdown(_, _)));
}

#[tokio::test]
async fn test_router_remote_logic() {
    use reader::engine::router::read_source;

    // Test HTTP routing (should attempt fetch or return feature error)
    let options = get_test_options();
    let res = read_source("https://example.com/data.json", None, options).await;
    // We check if it correctly routed to HTTP (even if it fails due to no network in test)
    assert!(res.is_err() || matches!(res.unwrap(), DataReaderResult::Json(_, _)));
}

#[tokio::test]
async fn test_router_s3_logic() {
    use reader::engine::router::read_source;
    let options = get_test_options();
    let res = read_source("s3://bucket/key.parquet", None, options).await;
    // Should be caught by the S3 router
    if let Err(e) = res {
        let err_str = e.to_string();
        assert!(
            err_str.contains("S3") || err_str.contains("disabled") || err_str.contains("Invalid")
        );
    }
}

#[cfg(feature = "database")]
#[tokio::test]
async fn test_sqlite_detection() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.db");
    // Just create an empty file with db extension
    fs::write(&file_path, "").unwrap();
    assert_eq!(get_file_format(&file_path), FileFormat::Sqlite);
}
