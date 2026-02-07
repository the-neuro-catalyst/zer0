use config::MasterConfig;
use std::sync::{Arc, Mutex};
use tui::app::{MonitorApp, SortMode};

#[test]
fn test_monitor_app_initialization() {
    let settings = Arc::new(MasterConfig::default());
    let log_buffer = Arc::new(Mutex::new(Vec::new()));
    let app = MonitorApp::new(settings, log_buffer).unwrap();

    assert_eq!(app.nodes.len(), 0);
    assert_eq!(app.sort_mode, SortMode::NAME);
}

#[test]
fn test_monitor_app_sorting() {
    let settings = Arc::new(MasterConfig::default());
    let log_buffer = Arc::new(Mutex::new(Vec::new()));
    let mut app = MonitorApp::new(settings, log_buffer).unwrap();

    // Manual node injection for testing sort
    app.nodes = vec![
        tui::app::DataNode {
            name: "b".to_string(),
            path: std::path::PathBuf::from("b"),
            is_dir: false,
            format: "TXT".into(),
            size_kb: 10.0,
            permissions: "".into(),
            modified: "2026-01-01".into(),
            line_count: None,
            entropy: None,
            depth: None,
            compromised: false,
        },
        tui::app::DataNode {
            name: "a".to_string(),
            path: std::path::PathBuf::from("a"),
            is_dir: false,
            format: "TXT".into(),
            size_kb: 5.0,
            permissions: "".into(),
            modified: "2026-01-02".into(),
            line_count: None,
            entropy: None,
            depth: None,
            compromised: false,
        },
    ];

    app.sort_nodes();
    assert_eq!(app.nodes[0].name, "a");
}
