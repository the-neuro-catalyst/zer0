pub struct WasmTransformer;
impl WasmTransformer {
    pub fn load(_path: &std::path::Path) -> anyhow::Result<Self> {
        Ok(Self)
    }
    pub fn transform(&mut self, record_json: &str) -> anyhow::Result<String> {
        Ok(record_json.to_string())
    }
}
