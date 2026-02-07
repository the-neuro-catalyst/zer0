use std::fs::File;

use std::io::{self, BufRead, Read};

use std::path::{Path, PathBuf};

use crate::analysis::information_density;
use crate::analysis::security;
use crate::analysis::structure;
use crate::error::DataReaderError;

use crate::output::{OutputFormat, OutputMode};

use crate::reader_result::{DataReaderResult, FileMetadata};

#[derive(Debug, PartialEq, Clone)]
pub enum FileFormat {
    Csv,
    Gzip,
    Image,
    Json,
    Markdown,
    Parquet,
    Pdf,
    Spreadsheet,
    Sqlite,
    Toml,
    Text,
    Xml,
    Yaml,
    Zip,
    Unknown,
}

fn detect_format_from_magic_bytes(file_path: &Path) -> Option<FileFormat> {
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(_) => {
            return None;
        }
    };

    let mut buffer = [0; 8];
    if file.read(&mut buffer).is_err() {
        return None;
    }

    if buffer.starts_with(b"PAR1") {
        return Some(FileFormat::Parquet);
    }
    if buffer.starts_with(b"PK\x03\x04") {
        return Some(FileFormat::Zip);
    }
    if buffer.starts_with(b"\x1f\x8b") {
        return Some(FileFormat::Gzip);
    }
    if buffer.starts_with(b"%PDF") {
        return Some(FileFormat::Pdf);
    }
    if buffer.starts_with(b"\x89PNG\r\n\x1a\n") {
        return Some(FileFormat::Image);
    }
    if buffer.starts_with(b"\xff\xd8\xff") {
        return Some(FileFormat::Image);
    }

    // JSON Detection (Basic check for { or [ after potential whitespace/BOM)
    let trimmed = buffer.iter().find(|&&b| !b.is_ascii_whitespace());
    if let Some(&b'{') | Some(&b'[') = trimmed {
        return Some(FileFormat::Json);
    }

    None
}

fn create_metadata(
    file_path: &Path,
    size: u64,
    line_count: Option<usize>,
    structural_depth: Option<usize>,
) -> FileMetadata {
    let mut metadata = FileMetadata::new(size);
    metadata.line_count = line_count;
    metadata.structural_depth = structural_depth;

    if let Ok(file) = File::open(file_path) {
        let mut buffer = Vec::new();
        // Read at most 64KB for information density and secret scanning for performance
        if file.take(65536).read_to_end(&mut buffer).is_ok() {
            metadata.information_density =
                Some(information_density::calculate_information_density(&buffer));

            if let Ok(content) = String::from_utf8(buffer) {
                let (_, compromised) = security::SecretScanner::redact(&content);
                metadata.compromised = compromised;
            }
        }
    }

    metadata
}

pub fn get_file_format(file_path: &Path) -> FileFormat {
    match file_path.extension().and_then(|s| s.to_str()) {
        Some("xlsx") | Some("xls") | Some("ods") => {
            return FileFormat::Spreadsheet;
        }
        Some("csv") => {
            return FileFormat::Csv;
        }
        Some("json") | Some("jsonl") => {
            return FileFormat::Json;
        }
        Some("md") => {
            return FileFormat::Markdown;
        }
        Some("parquet") => {
            return FileFormat::Parquet;
        }
        Some("pdf") => {
            return FileFormat::Pdf;
        }
        Some("sqlite") | Some("db") => {
            return FileFormat::Sqlite;
        }
        Some("toml") => {
            return FileFormat::Toml;
        }
        Some("txt") | Some("log") => {
            return FileFormat::Text;
        }
        Some("xml") => {
            return FileFormat::Xml;
        }
        Some("yaml") | Some("yml") => {
            return FileFormat::Yaml;
        }
        Some("zip") => {
            return FileFormat::Zip;
        }
        Some("gz") => {
            return FileFormat::Gzip;
        }
        Some("jpg") | Some("jpeg") | Some("png") | Some("gif") | Some("bmp") | Some("webp")
        | Some("svg") => {
            return FileFormat::Image;
        }
        _ => {}
    }

    if let Some(format) = detect_format_from_magic_bytes(file_path) {
        return format;
    }

    FileFormat::Unknown
}

#[derive(Clone)]
pub struct FileReaderOptions {
    pub head: Option<usize>,
    #[allow(dead_code)]
    pub file_type_override: Option<String>,
    pub output_mode: OutputMode,
    pub output_format: OutputFormat,
    pub pii_redaction: bool,
    #[allow(dead_code)]
    pub zero_copy: bool,
    #[allow(dead_code)]
    pub recursive: bool,
    #[allow(dead_code)]
    pub filter_exts: Option<Vec<String>>,
    #[allow(dead_code)]
    pub output_path: Option<PathBuf>,
}

pub async fn read_file_to_data(
    file_path: &Path,
    head: Option<usize>,
    file_format: FileFormat,
) -> Result<DataReaderResult, DataReaderError> {
    let base_metadata = std::fs::metadata(file_path)
        .map_err(|e| DataReaderError::FileReadError { path: file_path.to_path_buf(), source: e })?;
    let file_size = base_metadata.len();

    match file_format {
        FileFormat::Csv => {
            #[cfg(feature = "base-formats")]
            {
                crate::readers::csv_reader::read_csv_data(file_path, head).map(|data| {
                    let num_rows = data.num_rows as usize;
                    let metadata = create_metadata(file_path, file_size, Some(num_rows), None);
                    DataReaderResult::Csv(data, metadata)
                })
            }
            #[cfg(not(feature = "base-formats"))]
            {
                let raw_lines = read_file_raw_lines(file_path, head).unwrap_or_default();
                let content = raw_lines.join("\n");
                Ok(DataReaderResult::RawContent(
                    content,
                    create_metadata(file_path, file_size, Some(raw_lines.len()), None),
                ))
            }
        }
        FileFormat::Gzip => {
            #[cfg(feature = "extra-docs")]
            {
                crate::readers::gzip_reader::read_gzip_data(file_path).map(|data| {
                    DataReaderResult::Gzip(data, create_metadata(file_path, file_size, None, None))
                })
            }
            #[cfg(not(feature = "extra-docs"))]
            {
                Ok(DataReaderResult::Gzip(
                    crate::reader_result::GzipDataStub,
                    create_metadata(file_path, file_size, None, None),
                ))
            }
        }
        FileFormat::Image => {
            #[cfg(feature = "extra-docs")]
            {
                crate::readers::image_reader::read_image_data(file_path).map(|data| {
                    DataReaderResult::Image(data, create_metadata(file_path, file_size, None, None))
                })
            }
            #[cfg(not(feature = "extra-docs"))]
            {
                Ok(DataReaderResult::Image(
                    crate::reader_result::ImageDataStub,
                    create_metadata(file_path, file_size, None, None),
                ))
            }
        }
        FileFormat::Json => {
            crate::readers::json_reader::read_json_value(file_path, head).map(|data| {
                let line_count = data.line_count;
                let depth = Some(structure::calculate_schema_depth(&data.value));
                DataReaderResult::Json(
                    data,
                    create_metadata(file_path, file_size, line_count, depth),
                )
            })
        }
        FileFormat::Markdown => {
            #[cfg(feature = "extra-docs")]
            {
                crate::readers::md_reader::read_md_content(file_path, head).map(|data| {
                    DataReaderResult::Markdown(
                        data,
                        create_metadata(file_path, file_size, None, None),
                    )
                })
            }
            #[cfg(not(feature = "extra-docs"))]
            {
                Ok(DataReaderResult::Markdown(
                    crate::reader_result::MarkdownDataStub {
                        content: std::fs::read_to_string(file_path).unwrap_or_default(),
                    },
                    create_metadata(file_path, file_size, None, None),
                ))
            }
        }
        FileFormat::Parquet => {
            let data = crate::readers::parquet_reader::read_parquet_data(file_path, head)?;
            let num_rows = data.num_rows as usize;
            Ok(DataReaderResult::Parquet(
                data,
                create_metadata(file_path, file_size, Some(num_rows), None),
            ))
        }
        FileFormat::Pdf => {
            #[cfg(feature = "extra-docs")]
            {
                crate::readers::pdf_reader::read_pdf_text(file_path, head).map(|data| {
                    DataReaderResult::Pdf(data, create_metadata(file_path, file_size, None, None))
                })
            }
            #[cfg(not(feature = "extra-docs"))]
            {
                Ok(DataReaderResult::Pdf(
                    crate::reader_result::PdfDataStub { content: String::new(), line_count: 0 },
                    create_metadata(file_path, file_size, None, None),
                ))
            }
        }
        FileFormat::Spreadsheet => {
            #[cfg(feature = "extra-docs")]
            {
                crate::readers::spreadsheet_reader::read_spreadsheet_data(file_path).map(|data| {
                    DataReaderResult::Spreadsheet(
                        data,
                        create_metadata(file_path, file_size, None, None),
                    )
                })
            }
            #[cfg(not(feature = "extra-docs"))]
            {
                Ok(DataReaderResult::Spreadsheet(
                    crate::reader_result::SpreadsheetDataStub,
                    create_metadata(file_path, file_size, None, None),
                ))
            }
        }
        FileFormat::Sqlite => {
            #[cfg(feature = "database")]
            {
                let res = crate::readers::sqlite_reader::read_sqlite_data(file_path).await?;
                Ok(res)
            }
            #[cfg(not(feature = "database"))]
            {
                Ok(DataReaderResult::Sqlite(
                    crate::reader_result::SqliteDataStub,
                    create_metadata(file_path, file_size, None, None),
                ))
            }
        }
        FileFormat::Toml => {
            #[cfg(feature = "base-formats")]
            {
                let data = crate::readers::toml_reader::read_toml_value(file_path, head)?;
                let depth = Some(structure::calculate_json_depth(&data.value));
                Ok(DataReaderResult::Toml(data, create_metadata(file_path, file_size, None, depth)))
            }
            #[cfg(not(feature = "base-formats"))]
            {
                Ok(DataReaderResult::Toml(
                    crate::reader_result::TomlDataStub,
                    create_metadata(file_path, file_size, None, None), // depth will be None for stub
                ))
            }
        }
        FileFormat::Text => {
            crate::readers::txt_reader::read_txt_content(file_path, head).map(|data| {
                let line_count = data.line_count;
                DataReaderResult::Text(
                    data,
                    create_metadata(file_path, file_size, Some(line_count), None),
                )
            })
        }
        FileFormat::Xml => {
            #[cfg(feature = "base-formats")]
            {
                let data = crate::readers::xml_reader::read_xml_content(file_path, head)?;
                let depth = data.inferred_schema.as_ref().map(structure::calculate_xml_depth);
                Ok(DataReaderResult::Xml(data, create_metadata(file_path, file_size, None, depth)))
            }
            #[cfg(not(feature = "base-formats"))]
            {
                Ok(DataReaderResult::Xml(
                    crate::reader_result::XmlDataStub {
                        content: String::new(),
                        root_element: None,
                        element_counts: std::collections::HashMap::new(),
                        first_lines: None,
                    },
                    create_metadata(file_path, file_size, None, None),
                ))
            }
        }
        FileFormat::Yaml => {
            #[cfg(feature = "base-formats")]
            {
                let data = crate::readers::yaml_reader::read_yaml_value(file_path, head)?;
                let depth = Some(structure::calculate_json_depth(&data.value));
                Ok(DataReaderResult::Yaml(data, create_metadata(file_path, file_size, None, depth)))
            }
            #[cfg(not(feature = "base-formats"))]
            {
                Ok(DataReaderResult::Yaml(
                    crate::reader_result::YamlDataStub,
                    create_metadata(file_path, file_size, None, None), // depth will be None for stub
                ))
            }
        }
        FileFormat::Zip => {
            #[cfg(feature = "extra-docs")]
            {
                crate::readers::zip_reader::read_zip_data(file_path).map(|data| {
                    DataReaderResult::Zip(data, create_metadata(file_path, file_size, None, None))
                })
            }
            #[cfg(not(feature = "extra-docs"))]
            {
                Ok(DataReaderResult::Zip(
                    crate::reader_result::ZipDataStub,
                    create_metadata(file_path, file_size, None, None),
                ))
            }
        }
        FileFormat::Unknown => Err(DataReaderError::InternalError(format!(
            "Unsupported format: {}",
            file_path.display()
        ))),
    }
}

pub fn read_file_to_raw_content(
    file_path: &Path,
    head: Option<usize>,
    _output_format: OutputFormat,
) -> Result<String, DataReaderError> {
    let format = get_file_format(file_path);
    match format {
        #[cfg(feature = "base-formats")]
        FileFormat::Csv => crate::readers::csv_reader::get_csv_raw_content(file_path, head),
        FileFormat::Json => crate::readers::json_reader::get_json_raw_content(file_path, head),
        #[cfg(feature = "base-formats")]
        FileFormat::Toml => crate::readers::toml_reader::get_toml_raw_content(file_path, head),
        #[cfg(feature = "base-formats")]
        FileFormat::Yaml => crate::readers::yaml_reader::get_yaml_raw_content(file_path, head),
        _ => Err(DataReaderError::InternalError(
            "Raw content support disabled or unknown for this format".to_string(),
        )),
    }
}

#[allow(dead_code)]
pub fn read_file_raw_lines(
    file_path: &Path,
    limit: Option<usize>,
) -> Result<Vec<String>, DataReaderError> {
    let format = get_file_format(file_path);
    match format {
        #[cfg(feature = "base-formats")]
        FileFormat::Csv => crate::readers::csv_reader::read_csv_raw_lines(file_path, limit),
        FileFormat::Text | FileFormat::Markdown => {
            let file = File::open(file_path).map_err(|e| DataReaderError::FileReadError {
                path: file_path.to_path_buf(),
                source: e,
            })?;
            let reader = io::BufReader::new(file);
            let lines: Vec<String> = if let Some(l) = limit {
                reader.lines().take(l).map_while(Result::ok).collect()
            } else {
                reader.lines().map_while(Result::ok).collect()
            };
            Ok(lines)
        }
        _ => Err(DataReaderError::InternalError(format!(
            "Raw line reading not supported for format: {:?}",
            format
        ))),
    }
}

pub async fn read_file_to_stream(
    file_path: &Path,
    file_format: FileFormat,
) -> Result<DataReaderResult, DataReaderError> {
    let base_metadata = std::fs::metadata(file_path)
        .map_err(|e| DataReaderError::FileReadError { path: file_path.to_path_buf(), source: e })?;
    let file_size = base_metadata.len();
    let metadata = create_metadata(file_path, file_size, None, None);

    match file_format {
        FileFormat::Json => {
            let stream = crate::readers::json_reader::read_json_stream(file_path)?;
            Ok(DataReaderResult::Stream(stream, metadata))
        }
        _ => read_file_to_data(file_path, None, file_format).await,
    }
}

pub async fn read_file_content(
    file_path: &Path,
    options: FileReaderOptions,
) -> Result<DataReaderResult, DataReaderError> {
    let path = file_path.to_path_buf();
    let determined_format = get_file_format(&path);

    match options.output_mode {
        OutputMode::FullRaw => {
            let raw_content = read_file_to_raw_content(&path, options.head, options.output_format)?;
            Ok(DataReaderResult::RawContent(raw_content, create_metadata(&path, 0, None, None)))
        }
        OutputMode::SchemaOnly | OutputMode::Default => {
            let mut result = read_file_to_data(&path, options.head, determined_format).await?;
            if options.pii_redaction {
                result.redact();
            }
            Ok(result)
        }
        OutputMode::Stream => read_file_to_stream(&path, determined_format).await,
        OutputMode::Analyze => {
            let mut result = read_file_to_data(&path, options.head, determined_format).await?;
            if options.pii_redaction {
                result.redact();
            }
            Ok(result)
        }
    }
}

pub async fn read_directory_content(
    directory_path: &Path,
    options: FileReaderOptions,
) -> Result<DataReaderResult, DataReaderError> {
    let mut results: Vec<(PathBuf, DataReaderResult)> = Vec::new();
    let mut set = tokio::task::JoinSet::new();

    if let Ok(entries) = std::fs::read_dir(directory_path) {
        for entry in entries.filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_file() {
                let opt = options.clone();
                set.spawn(async move {
                    let res = read_file_content(&path, opt).await;
                    (path, res)
                });
            }
        }
    }

    while let Some(res) = set.join_next().await {
        if let Ok((path, Ok(data))) = res {
            results.push((path, data));
        }
    }

    Ok(DataReaderResult::DirectoryResults(results, FileMetadata::new(0)))
}
