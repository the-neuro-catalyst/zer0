use crate::error::DataReaderError;

use std::fs::File;

use std::io::Read;

use std::path::Path;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct TomlData {
    pub value: serde_json::Value,
}

pub fn read_toml_value(path: &Path, _head: Option<usize>) -> Result<TomlData, DataReaderError> {
    let mut file = File::open(path)
        .map_err(|e| DataReaderError::FileReadError { path: path.to_path_buf(), source: e })?;

    let mut content = String::new();
    file.read_to_string(&mut content)
        .map_err(|e| DataReaderError::FileReadError { path: path.to_path_buf(), source: e })?;

    let value: serde_json::Value = toml::from_str(&content).map_err(|e| {
        DataReaderError::ParseError { path: path.to_path_buf(), source: e.to_string().into() }
    })?;

    Ok(TomlData { value })
}

pub fn get_toml_raw_content(path: &Path, _head: Option<usize>) -> Result<String, DataReaderError> {
    let mut file = File::open(path)
        .map_err(|e| DataReaderError::FileReadError { path: path.to_path_buf(), source: e })?;

    let mut content = String::new();
    file.read_to_string(&mut content)
        .map_err(|e| DataReaderError::FileReadError { path: path.to_path_buf(), source: e })?;
    Ok(content)
}
