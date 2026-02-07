use regex::{Regex, RegexSet};

use std::sync::OnceLock;

struct ScannerPatterns {
    names: Vec<&'static str>,
    regexes: Vec<Regex>,
    set: RegexSet,
}

fn get_patterns() -> &'static ScannerPatterns {
    static PATTERNS: OnceLock<ScannerPatterns> = OnceLock::new();
    PATTERNS.get_or_init(|| {
        let raw_patterns = vec![
            ("AWS Access Key", r"(AKIA|ASIA)[0-9A-Z]{16}"),
            ("AWS Secret Key", r"([A-Za-z0-9/+=]{40})"),
            ("GCP API Key", r"AIza[0-9A-Za-z-_]{35}"),
            ("GitHub Token", r"(ghp|gho|ghu|ghs|ghr)_[a-zA-Z0-9]{36}"),
            ("Slack Webhook", r"https://hooks\.slack\.com/services/T[a-zA-Z0-9_]+/B[a-zA-Z0-9_]+/[a-zA-Z0-9_]+"),
            ("Stripe API Key", r"sk_live_[0-9a-zA-Z]{24}"),
            ("Slack Token", r"xox[baprs]-[a-zA-Z0-9-]+"),
            ("Google OAuth", r"ya29\.[a-zA-Z0-9_-]+"),
            ("Credit Card (Visa)", r"\b4[0-9]{12}(?:[0-9]{3})?\b"),
            ("Credit Card (MasterCard)", r"\b(?:5[1-5][0-9]{2}|222[1-9]|22[3-9][0-9]|2[3-6][0-9]{2}|27[01][0-9]|2720)[0-9]{12}\b"),
            ("Private Key", r"-----BEGIN [A-Z ]+ PRIVATE KEY-----[\s\S]+?-----END [A-Z ]+ PRIVATE KEY-----"),
            ("Email Address", r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}"),
            ("Phone Number", r"\b\d{3}[-.]?\d{3}[-.]?\d{4}\b"),
            ("IPv4 Address", r"\b\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}\b"),
            ("SSN", r"\b\d{3}-\d{2}-\d{4}\b"),
            ("URL Credentials", r#"[a-zA-Z0-9]+://[^:\s]+:[^@\s]+@[^#\s]+"#),
            ("Generic API Key", r#"(?i)(api[_-]?key|secret|token|password)[ \t]*[:=][ \t]*['\"][a-zA-Z0-9_-]{16,}['"]"#),
        ];

        let names: Vec<&str> = raw_patterns.iter().map(|p| p.0).collect();
        let expressions: Vec<&str> = raw_patterns.iter().map(|p| p.1).collect();
        let regexes: Vec<Regex> = expressions.iter().map(|e| Regex::new(e).unwrap()).collect();
        let set = RegexSet::new(&expressions).unwrap();

        ScannerPatterns { names, regexes, set }
    })
}

fn is_sensitive_key(key: &str) -> bool {
    let k = key.to_lowercase();
    k.contains("password")
        || k.contains("secret")
        || k.contains("token")
        || k.contains("api_key")
        || k.contains("apikey")
        || k.contains("credentials")
        || k.contains("auth_key")
        || k.contains("private_key")
}

#[derive(Debug, Clone)]
pub struct SecretMatch {
    #[allow(dead_code)]
    pub pattern_name: &'static str,
    pub start: usize,
    pub end: usize,
    #[allow(dead_code)]
    pub value: String,
}

pub struct SecretScanner;

impl SecretScanner {
    pub fn find_matches(content: &str) -> Vec<SecretMatch> {
        let patterns = get_patterns();

        // Optimization: Check the entire set first.
        // If no matches found in the entire string, return empty immediately.
        if !patterns.set.is_match(content) {
            return Vec::new();
        }

        let mut matches = Vec::new();
        // Only iterate over the indices that actually matched according to the RegexSet
        for i in patterns.set.matches(content) {
            let name = patterns.names[i];
            let re = &patterns.regexes[i];

            for m in re.find_iter(content) {
                matches.push(SecretMatch {
                    pattern_name: name,
                    start: m.start(),
                    end: m.end(),
                    value: m.as_str().to_string(),
                });
            }
        }
        matches
    }

    pub fn redact(content: &str) -> (String, bool) {
        let matches = Self::find_matches(content);
        if matches.is_empty() {
            return (content.to_string(), false);
        }

        let mut redacted = content.to_string();
        // Sort matches in reverse order to keep indices valid during replacement
        let mut sorted_matches = matches;
        sorted_matches.sort_by(|a, b| b.start.cmp(&a.start));

        for m in sorted_matches {
            // Ensure we are not overlapping if multiple patterns match the same area
            if m.start < redacted.len() && m.end <= redacted.len() {
                redacted.replace_range(m.start..m.end, "[REDACTED]");
            }
        }

        (redacted, true)
    }

    #[allow(dead_code)]
    pub fn redact_json_value(value: &mut serde_json::Value) -> bool {
        let mut compromised = false;
        match value {
            serde_json::Value::String(s) => {
                let (redacted, found) = Self::redact(s);
                if found {
                    *s = redacted;
                    compromised = true;
                }
            }
            serde_json::Value::Array(arr) => {
                for item in arr {
                    if Self::redact_json_value(item) {
                        compromised = true;
                    }
                }
            }
            serde_json::Value::Object(obj) => {
                for (key, val) in obj.iter_mut() {
                    if is_sensitive_key(key) {
                        if let serde_json::Value::String(s) = val {
                            if !s.contains("[REDACTED]") {
                                *s = "[REDACTED]".to_string();
                                compromised = true;
                            }
                        }
                    }
                    if Self::redact_json_value(val) {
                        compromised = true;
                    }
                }
            }
            _ => {}
        }
        compromised
    }

    #[allow(dead_code)]
    pub fn redact_schema_value(value: &mut schema::SchemaValue) -> bool {
        use schema::SchemaValue;
        let mut compromised = false;

        match value {
            SchemaValue::String(s) => {
                let (redacted, found) = Self::redact(s);
                if found {
                    *s = std::borrow::Cow::Owned(redacted);
                    compromised = true;
                }
            }
            SchemaValue::Array(arr) => {
                for item in arr {
                    if Self::redact_schema_value(item) {
                        compromised = true;
                    }
                }
            }
            SchemaValue::Object(obj) => {
                for (key, val) in obj.iter_mut() {
                    if is_sensitive_key(key) {
                        if let SchemaValue::String(s) = val {
                            if !s.contains("[REDACTED]") {
                                *s = std::borrow::Cow::Owned("[REDACTED]".to_string());
                                compromised = true;
                            }
                        }
                    }
                    if Self::redact_schema_value(val) {
                        compromised = true;
                    }
                }
            }
            SchemaValue::Union(variants) => {
                for v in variants {
                    if Self::redact_schema_value(v) {
                        compromised = true;
                    }
                }
            }
            _ => {}
        }
        compromised
    }
}
