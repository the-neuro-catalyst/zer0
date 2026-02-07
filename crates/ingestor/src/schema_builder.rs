pub struct SqlSchemaBuilder;
impl SqlSchemaBuilder {
    pub fn new() -> Self {
        Self
    }
}

impl Default for SqlSchemaBuilder {
    fn default() -> Self {
        Self::new()
    }
}
