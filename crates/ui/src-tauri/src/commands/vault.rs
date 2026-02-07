use aes_gcm::{
    Aes256Gcm, Key, Nonce,
    aead::{Aead, AeadCore, KeyInit, OsRng, rand_core::RngCore},
};

use serde::{Deserialize, Serialize};

use std::fs;

use std::path::{Path, PathBuf};

use tauri::Manager;

use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

use log::{error, info, warn};

const VAULT_FILE: &str = "vault.json";
const KEY_FILE: &str = ".master.key";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VaultEntry {
    pub id: String,
    pub label: String,
    pub provider: String,

    pub last_used: String,
    pub masked: String,
    pub ciphertext: String,
    pub nonce: String,
}

fn get_app_dir(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    app.path().app_data_dir().map_err(|e| format!("Failed to get app data dir: {}", e))
}

fn get_master_key(app_dir: &Path) -> Result<Key<Aes256Gcm>, String> {
    let key_path = app_dir.join(KEY_FILE);
    if key_path.exists() {
        let key_bytes = fs::read(&key_path).map_err(|e| e.to_string())?;
        if key_bytes.len() != 32 {
            error!("Master key corruption detected!");
            return Err("Hardware key length mismatch".to_string());
        }
        Ok(*Key::<Aes256Gcm>::from_slice(&key_bytes))
    } else {
        info!("Initializing new master key infrastructure...");
        let mut key_bytes = [0u8; 32];
        OsRng.fill_bytes(&mut key_bytes);
        fs::write(&key_path, key_bytes).map_err(|e| e.to_string())?;
        Ok(*Key::<Aes256Gcm>::from_slice(&key_bytes))
    }
}

fn load_vault(app_dir: &Path) -> Result<Vec<VaultEntry>, String> {
    let vault_path = app_dir.join(VAULT_FILE);
    if !vault_path.exists() {
        info!("No existing vault found at {:?}, starting fresh.", vault_path);
        return Ok(Vec::new());
    }
    let content = fs::read_to_string(&vault_path).map_err(|e| e.to_string())?;
    let entries: Vec<VaultEntry> = serde_json::from_str(&content).map_err(|e| {
        error!("Vault JSON corruption: {}", e);
        e.to_string()
    })?;
    info!("Vault Loaded: {} encrypted entries", entries.len());
    Ok(entries)
}

fn save_vault(app_dir: &Path, entries: &[VaultEntry]) -> Result<(), String> {
    let vault_path = app_dir.join(VAULT_FILE);
    let content = serde_json::to_string_pretty(entries).map_err(|e| e.to_string())?;
    fs::write(&vault_path, content).map_err(|e| e.to_string())?;
    info!("Vault Sync: Persisted {} entries to disk", entries.len());
    Ok(())
}

#[tauri::command]
pub fn get_vault_entries(app: tauri::AppHandle) -> Result<Vec<VaultEntry>, String> {
    let app_dir = get_app_dir(&app)?;
    fs::create_dir_all(&app_dir).map_err(|e| e.to_string())?;
    let entries = load_vault(&app_dir)?;
    // Redact sensitive data before sending to frontend
    let safe_entries = entries
        .into_iter()
        .map(|mut e| {
            e.ciphertext = String::new();
            e.nonce = String::new();
            e
        })
        .collect();
    Ok(safe_entries)
}

#[tauri::command]
pub fn save_secret(
    app: tauri::AppHandle,
    label: String,
    value: String,
    provider: String,
) -> Result<VaultEntry, String> {
    info!("Encrypting new secret: {}", label);
    let app_dir = get_app_dir(&app)?;
    let master_key = get_master_key(&app_dir)?;
    let cipher = Aes256Gcm::new(&master_key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    let ciphertext = cipher
        .encrypt(&nonce, value.as_bytes())
        .map_err(|_| "Encryption protocol failure".to_string())?;

    let id = uuid::Uuid::new_v4().to_string();
    let masked = if value.len() > 4 {
        format!("{}••••••••", value.chars().take(4).collect::<String>())
    } else {
        "••••••••".to_string()
    };

    let entry = VaultEntry {
        id,
        label,
        provider,

        last_used: "Just now".to_string(),
        masked,
        ciphertext: BASE64.encode(ciphertext),
        nonce: BASE64.encode(nonce),
    };

    let mut entries = load_vault(&app_dir)?;
    entries.insert(0, entry.clone());
    save_vault(&app_dir, &entries)?;

    let mut safe_entry = entry;
    safe_entry.ciphertext = String::new();
    safe_entry.nonce = String::new();
    Ok(safe_entry)
}

#[tauri::command]
pub fn delete_secret(app: tauri::AppHandle, id: String) -> Result<(), String> {
    info!("Destroying secret ID: {}", id);
    let app_dir = get_app_dir(&app)?;
    let mut entries = load_vault(&app_dir)?;
    let original_len = entries.len();
    entries.retain(|e| e.id != id);

    if entries.len() < original_len {
        save_vault(&app_dir, &entries)?;
        info!("Secret destroyed successfully.");
    } else {
        warn!("Secret ID {} not found for deletion.", id);
    }
    Ok(())
}

#[tauri::command]
pub fn reveal_secret(app: tauri::AppHandle, id: String) -> Result<String, String> {
    warn!("Decryption requested for ID: {}", id);
    let app_dir = get_app_dir(&app)?;
    let entries = load_vault(&app_dir)?;
    let entry = entries.iter().find(|e| e.id == id).ok_or("Identity fragment not found")?;

    let master_key = get_master_key(&app_dir)?;
    let cipher = Aes256Gcm::new(&master_key);

    let nonce_bytes =
        BASE64.decode(&entry.nonce).map_err(|_| "Corruption in nonce metadata".to_string())?;
    let ciphertext_bytes = BASE64
        .decode(&entry.ciphertext)
        .map_err(|_| "Corruption in encrypted payload".to_string())?;

    let nonce = Nonce::from_slice(&nonce_bytes);
    let plaintext = cipher
        .decrypt(nonce, ciphertext_bytes.as_ref())
        .map_err(|_| "Decryption key mismatch".to_string())?;

    info!("Decryption successful for ID: {}", id);
    String::from_utf8(plaintext).map_err(|_| "Invalid data encoding in secret".to_string())
}
