use anyhow::Result;
use chrono::Local;
use config::MasterConfig as Settings;
use ratatui::widgets::{ListState, TableState};
use reader::engine::file_engine::get_file_format;
use reader::reader_result::DataReaderResult;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ViewMode {
    Standard,
    HexView,
    TableView,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum SortMode {
    NAME,
    SIZE,
    MODIFIED,
    FORMAT,
}

#[derive(Clone, Debug)]
pub struct DataNode {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
    pub format: String,
    pub size_kb: f64,
    pub permissions: String,
    pub modified: String,
    pub line_count: Option<usize>,
    pub entropy: Option<f64>,
    pub depth: Option<usize>,
    pub compromised: bool,
}

pub struct MonitorApp {
    pub settings: Arc<Settings>,
    pub log_buffer: Arc<std::sync::Mutex<Vec<String>>>,
    pub current_path: PathBuf,
    pub nodes: Vec<DataNode>,
    pub list_state: ListState,
    pub table_state: TableState,
    pub selected_resource: Option<DataNode>,
    pub log_entries: Vec<String>,
    pub view_mode: ViewMode,
    pub sort_mode: SortMode,
    pub show_quit_confirm: bool,
    pub show_help: bool,
    pub last_tick: Instant,
    pub config_last_modified: Option<std::time::SystemTime>,
    pub should_quit: bool,
    pub inspection_result: Option<DataReaderResult>,
    pub is_inspecting: bool,
    pub hex_data: Option<Vec<u8>>,
    pub hex_scroll: usize,
    pub table_headers: Vec<String>,
    pub is_jumping: bool,
    pub jump_input: String,
    pub is_connecting: bool,
    pub remote_url_input: String,
    pub remote_url: Option<String>,
    pub session_scanned: usize,
    pub session_pii_hits: usize,
    pub session_ram_usage: f64,
    pub current_axiom_index: usize,
    pub is_loading_dir: bool,
}

impl MonitorApp {
    pub fn new(
        settings: Arc<Settings>,
        log_buffer: Arc<std::sync::Mutex<Vec<String>>>,
    ) -> Result<Self> {
        let app = Self {
            settings,
            log_buffer,
            current_path: std::env::current_dir()?,
            nodes: Vec::new(),
            list_state: ListState::default(),
            table_state: TableState::default(),
            selected_resource: None,
            log_entries: Vec::new(),
            view_mode: ViewMode::Standard,
            sort_mode: SortMode::NAME,
            show_quit_confirm: false,
            show_help: false,
            last_tick: Instant::now(),
            config_last_modified: fs::metadata("zero.config.toml")
                .ok()
                .and_then(|m| m.modified().ok()),
            should_quit: false,
            inspection_result: None,
            is_inspecting: false,
            hex_data: None,
            hex_scroll: 0,
            table_headers: Vec::new(),
            is_jumping: false,
            jump_input: String::new(),
            is_connecting: false,
            remote_url_input: String::new(),
            remote_url: None,
            session_scanned: 0,
            session_pii_hits: 0,
            session_ram_usage: 0.0,
            current_axiom_index: 0,
            is_loading_dir: false,
        };

        Ok(app)
    }

    pub fn update_ram_usage(&mut self) {
        #[cfg(target_os = "linux")]
        {
            if let Ok(content) = std::fs::read_to_string("/proc/self/statm") {
                let parts: Vec<&str> = content.split_whitespace().collect();
                if parts.len() > 1 {
                    if let Ok(pages) = parts[1].parse::<u64>() {
                        self.session_ram_usage = ((pages * 4096) as f64) / 1024.0 / 1024.0;
                    }
                }
            }
        }
    }

    pub fn cycle_axiom(&mut self) {
        self.current_axiom_index = (self.current_axiom_index + 1) % 5;
    }

    pub fn log(&mut self, message: String) {
        let mut logs = self.log_buffer.lock().unwrap();
        let timestamp = Local::now().format("%H:%M:%S");
        logs.push(format!("[{}] INFO {}", timestamp, message));
        if logs.len() > 100 {
            logs.remove(0);
        }
    }

    pub fn scan_directory(current_path: PathBuf, sort_mode: SortMode) -> Vec<DataNode> {
        let mut nodes = Vec::new();
        if let Some(parent) = current_path.parent() {
            nodes.push(DataNode {
                name: "../".into(),
                path: parent.canonicalize().unwrap_or_else(|_| parent.to_path_buf()),
                is_dir: true,
                format: "DIR".into(),
                size_kb: 0.0,
                permissions: "rwxr-xr-x".into(),
                modified: "-".into(),
                line_count: None,
                entropy: None,
                depth: None,
                compromised: false,
            });
        }
        if let Ok(entries) = fs::read_dir(&current_path) {
            for entry in entries.filter_map(|e| e.ok()) {
                let path = entry.path();
                let abs_path = if path.is_absolute() {
                    path.clone()
                } else {
                    current_path.join(&path).canonicalize().unwrap_or(path)
                };
                let name = entry.file_name().to_string_lossy().to_string();
                let is_dir = abs_path.is_dir();

                let mut format_str = "DIR".to_string();
                let mut size = 0.0;
                let mut perms = "---------".to_string();
                let mut mod_time = "-".to_string();

                if let Ok(metadata) = fs::metadata(&abs_path) {
                    size = (metadata.len() as f64) / 1024.0;

                    #[cfg(unix)]
                    {
                        use std::os::unix::fs::PermissionsExt;
                        let mode = metadata.permissions().mode();
                        perms = format!(
                            "{}{}{}{}{}{}{}{}{}",
                            if (mode & 0o400) != 0 { "r" } else { "-" },
                            if (mode & 0o200) != 0 { "w" } else { "-" },
                            if (mode & 0o100) != 0 { "x" } else { "-" },
                            if (mode & 0o040) != 0 { "r" } else { "-" },
                            if (mode & 0o020) != 0 { "w" } else { "-" },
                            if (mode & 0o010) != 0 { "x" } else { "-" },
                            if (mode & 0o004) != 0 { "r" } else { "-" },
                            if (mode & 0o002) != 0 { "w" } else { "-" },
                            if (mode & 0o001) != 0 { "x" } else { "-" }
                        );
                    }

                    if let Ok(m) = metadata.modified() {
                        let dt: chrono::DateTime<Local> = m.into();
                        mod_time = dt.format("%Y-%m-%d %H:%M").to_string();
                    }

                    if !is_dir {
                        let fmt = get_file_format(&abs_path);
                        format_str = format!("{:?}", fmt).to_uppercase();
                    }
                }

                nodes.push(DataNode {
                    name,
                    path: abs_path,
                    is_dir,
                    format: format_str,
                    size_kb: size,
                    permissions: perms,
                    modified: mod_time,
                    line_count: None,
                    entropy: None,
                    depth: None,
                    compromised: false,
                });
            }
        }
        Self::sort_nodes_list(&mut nodes, sort_mode);
        nodes
    }

    pub fn set_nodes(&mut self, nodes: Vec<DataNode>) {
        tracing::debug!("Loaded {} nodes", nodes.len());
        self.nodes = nodes;
        self.is_loading_dir = false;
    }

    fn sort_nodes_list(nodes: &mut [DataNode], sort_mode: SortMode) {
        nodes.sort_by(|a, b| {
            if a.name == "../" {
                return std::cmp::Ordering::Less;
            }
            if b.name == "../" {
                return std::cmp::Ordering::Greater;
            }

            let dir_cmp = b.is_dir.cmp(&a.is_dir);
            if dir_cmp != std::cmp::Ordering::Equal {
                return dir_cmp;
            }

            match sort_mode {
                SortMode::NAME => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
                SortMode::SIZE => {
                    a.size_kb.partial_cmp(&b.size_kb).unwrap_or(std::cmp::Ordering::Equal)
                }
                SortMode::MODIFIED => a.modified.cmp(&b.modified),
                SortMode::FORMAT => a.format.cmp(&b.format),
            }
        });
    }

    pub fn sort_nodes(&mut self) {
        Self::sort_nodes_list(&mut self.nodes, self.sort_mode);
    }

    pub fn select_node(&mut self) -> Option<PathBuf> {
        if let Some(i) = self.list_state.selected() {
            let node = self.nodes[i].clone();
            if node.is_dir {
                self.current_path = node.path;
                self.is_loading_dir = true;
                self.list_state.select(Some(0));
                self.selected_resource = None;
                self.inspection_result = None;
                self.remote_url = None;
                self.table_state.select(None);
                self.table_headers.clear();
                Some(self.current_path.clone())
            } else {
                let path_clone = node.path.clone();
                self.selected_resource = Some(node);
                self.inspection_result = None;
                self.remote_url = None;
                self.table_state.select(None);
                self.table_headers.clear();
                Some(path_clone)
            }
        } else {
            None
        }
    }

    pub async fn inspect_selected_node(&mut self) {
        if let Some(node) = &self.selected_resource {
            if node.is_dir {
                return;
            }

            self.is_inspecting = true;
            self.inspection_result = None;
            let path_str = node.path.to_string_lossy().to_string();

            use reader::engine::file_engine::FileReaderOptions;
            use reader::output::{OutputFormat, OutputMode};

            let options = FileReaderOptions {
                head: Some(100),
                file_type_override: None,
                output_mode: OutputMode::Default,
                output_format: OutputFormat::Json,
                pii_redaction: true,
                zero_copy: true,
                recursive: false,
                filter_exts: None,
                output_path: None,
            };

            let result = reader::engine::router::read_source(&path_str, None, options).await;

            if let Ok(res) = result {
                self.session_scanned += 1;
                if res.get_metadata().compromised {
                    self.session_pii_hits += 1;
                }
                self.inspection_result = Some(res);
            }
            self.is_inspecting = false;
        }
    }

    pub async fn inspect_remote_url(&mut self) -> Result<()> {
        if self.remote_url_input.is_empty() {
            return Ok(());
        }

        self.is_inspecting = true;
        self.inspection_result = None;
        let url = self.remote_url_input.clone();

        use reader::engine::file_engine::FileReaderOptions;
        use reader::output::{OutputFormat, OutputMode};

        let options = FileReaderOptions {
            head: Some(100),
            file_type_override: None,
            output_mode: OutputMode::Default,
            output_format: OutputFormat::Json,
            pii_redaction: true,
            zero_copy: true,
            recursive: false,
            filter_exts: None,
            output_path: None,
        };

        let url_clone = url.clone();
        let result = reader::engine::router::read_source(&url_clone, None, options).await;

        if let Ok(res) = result {
            self.inspection_result = Some(res);
            self.remote_url = Some(url);
            self.selected_resource = None;
        }
        self.is_inspecting = false;
        Ok(())
    }

    pub fn load_hex_data(&mut self) -> Result<()> {
        if let Some(node) = &self.selected_resource {
            if node.is_dir {
                return Ok(());
            }

            use std::io::Read;
            let mut file = fs::File::open(&node.path)?;
            let mut buffer = vec![0u8; 65536];
            let n = file.read(&mut buffer)?;
            buffer.truncate(n);
            self.hex_data = Some(buffer);
            self.hex_scroll = 0;
        }
        Ok(())
    }

    pub async fn on_tick(&mut self) {
        if let Ok(metadata) = fs::metadata("zero.config.toml") {
            if let Ok(modified) = metadata.modified() {
                if Some(modified) != self.config_last_modified {
                    if let Ok(new_settings) = Settings::load() {
                        self.settings = Arc::new(new_settings);
                        self.config_last_modified = Some(modified);
                    }
                }
            }
        }

        self.update_ram_usage();

        if self.last_tick.elapsed().as_secs() >= 5 {
            self.cycle_axiom();
            self.last_tick = Instant::now();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_app_log() {
        let settings = Arc::new(Settings::default());
        let log_buffer = Arc::new(Mutex::new(Vec::new()));
        let mut app = MonitorApp::new(settings, log_buffer.clone()).unwrap();

        app.log("test message".to_string());

        let logs = log_buffer.lock().unwrap();
        assert_eq!(logs.len(), 1);
        assert!(logs[0].contains("test message"));
    }

    #[test]
    fn test_log_rotation() {
        let settings = Arc::new(Settings::default());
        let log_buffer = Arc::new(Mutex::new(Vec::new()));
        let mut app = MonitorApp::new(settings, log_buffer.clone()).unwrap();

        for i in 0..110 {
            app.log(format!("message {}", i));
        }

        let logs = log_buffer.lock().unwrap();
        assert_eq!(logs.len(), 100);
        // Should contain the last 100 messages
        assert!(logs[0].contains("message 10"));
        assert!(logs[99].contains("message 109"));
    }
}
