use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FileInspection {
    pub path: String,
    pub format: String,
    pub size_bytes: u64,
    pub content_preview: String,
    pub metadata: InspectionMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InspectionMetadata {
    pub line_count: Option<usize>,
    pub information_density: Option<f64>,
    pub structural_depth: Option<usize>,
    pub has_sensitive_data: bool,
    pub redacted: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemStats {
    pub total_sessions: u64,
    pub total_events: u64,
    pub active_nodes: u32,
    pub validity_confidence: f32,
}

#[cfg(test)]
mod tests {

    use super::*;

    use serde_json;

    #[test]
    fn test_system_stats_serialization() {
        let stats = SystemStats {
            total_sessions: 10,
            total_events: 100,
            active_nodes: 5,
            validity_confidence: 0.95,
        };

        let json = serde_json::to_string(&stats).unwrap();
        // Check for camelCase keys
        assert!(json.contains("\"totalSessions\":10"));
        assert!(json.contains("\"activeNodes\":5"));
    }
}
