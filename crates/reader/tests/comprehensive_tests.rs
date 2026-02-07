#[cfg(test)]
mod tests {
    use reader::engine::file_engine::{FileReaderOptions, read_file_content};
    use reader::output::{OutputFormat, OutputMode};
    use reader::reader_result::DataReaderResult;
    use std::path::PathBuf;

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
    async fn test_read_csv_content() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("../../test_data/staff.csv");

        // Ensure test data exists, otherwise skip (or mock it)
        if !path.exists() {
            println!("Skipping CSV test: file not found at {:?}", path);
            return;
        }

        let result = read_file_content(&path, get_test_options()).await;
        assert!(result.is_ok(), "Failed to read CSV: {:?}", result.err());

        if let Ok(DataReaderResult::Csv(data, meta)) = result {
            assert!(data.num_rows > 0);
            assert!(meta.size_bytes > 0);
            println!("CSV Read Success: {} rows", data.num_rows);
        } else {
            panic!("Expected Csv result variant");
        }
    }

    #[tokio::test]
    async fn test_read_json_content() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("../../test_data/users.json");

        if !path.exists() {
            println!("Skipping JSON test: file not found");
            return;
        }

        let result = read_file_content(&path, get_test_options()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_information_density() {
        // Create a temporary text file
        let content = "Hello World! This is a test file for entropy calculation.";
        let file_path = "temp_entropy_test.txt";
        std::fs::write(file_path, content).unwrap();

        let path = PathBuf::from(file_path);
        let result = read_file_content(&path, get_test_options()).await.unwrap();

        let metadata = match result {
            DataReaderResult::Text(_, m) => m,
            DataReaderResult::RawContent(_, m) => m,
            _ => panic!("Expected text result"),
        };

        // Entropy/Density should be calculable
        assert!(metadata.information_density.is_some());
        let density = metadata.information_density.unwrap();
        assert!(density > 0.0 && density <= 1.0);

        std::fs::remove_file(file_path).unwrap();
    }

    #[cfg(feature = "database")]
    #[tokio::test]
    async fn test_sqlite_query() {
        use rusqlite::Connection;

        // Create a temporary SQLite DB
        let db_path = "test_db.sqlite";
        let conn = Connection::open(db_path).unwrap();
        conn.execute(
            "CREATE TABLE person (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL,
                  data            BLOB
                  )",
            (),
        )
        .unwrap();
        conn.execute(
            "INSERT INTO person (name, data) VALUES (?1, ?2)",
            ("Steven", &vec![1u8, 2u8, 3u8]),
        )
        .unwrap();
        conn.close().unwrap();

        // Test reading via DB Engine
        use reader::engine::db_engine::{
            DatabaseReaderOptions, DatabaseType, read_database_content,
        };

        let options = DatabaseReaderOptions {
            db_type: DatabaseType::Sqlite,
            db_url: format!("sqlite://{}", db_path),
            query: "SELECT * FROM person".to_string(),
        };

        let result = read_database_content(options).await;
        assert!(result.is_ok(), "Failed to read SQLite: {:?}", result.err());

        // Cleanup
        std::fs::remove_file(db_path).unwrap();
    }
}
