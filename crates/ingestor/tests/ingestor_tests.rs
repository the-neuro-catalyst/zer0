use ingestor::engine::{DataProcessor, ProcessorInput};
use serde_json::json;

#[test]
fn test_data_processor_workflow() {
    let mut processor = DataProcessor::new(None);

    // Should fail if not initialized
    let input =
        ProcessorInput { data: json!({"test": "data"}), context: "test context".to_string() };
    assert!(processor.process_data(input.clone()).is_err());

    processor.initialize().unwrap();

    let output = processor.process_data(input).unwrap();
    assert_eq!(output.quality_score, 1.0);
    assert_eq!(output.result["test"], "data");
}

#[test]
fn test_transformation_module() {
    use ingestor::transformation::WasmTransformer;
    let mut transformer = WasmTransformer::load(std::path::Path::new("dummy")).unwrap();
    let data = "{\"test\": \"data\"}";
    let transformed = transformer.transform(data).unwrap();
    assert_eq!(transformed, data);
}
