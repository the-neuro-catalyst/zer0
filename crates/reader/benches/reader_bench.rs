use criterion::{Criterion, criterion_group, criterion_main};
use reader::engine::file_engine::{FileReaderOptions, read_file_content};
use reader::output::{OutputFormat, OutputMode};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use tokio::runtime::Runtime;

fn create_large_mock_json(path: &Path, size_mb: usize) {
    let mut file = File::create(path).unwrap();
    file.write_all(b"[").unwrap();
    for i in 0..(size_mb * 5000) {
        if i > 0 {
            file.write_all(b",").unwrap();
        }
        write!(file, r#"{{"id":{},"name":"User{}","email":"user{}@example.com","data":"some random data here","compromised":false}}"#, i, i, i).unwrap();
    }
    file.write_all(b"]").unwrap();
}

fn bench_json_perception(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let file_path = Path::new("large_bench.json");

    if !file_path.exists() {
        create_large_mock_json(file_path, 10);
    }

    let options = FileReaderOptions {
        head: Some(100),
        file_type_override: None,
        output_mode: OutputMode::Default,
        output_format: OutputFormat::Json,
        pii_redaction: false,
        zero_copy: true,
        recursive: false,
        filter_exts: None,
        output_path: None,
    };

    c.bench_function("json_perception_10mb", |b| {
        b.to_async(&rt).iter(|| {
            read_file_content(
                std::hint::black_box(file_path),
                std::hint::black_box(options.clone()),
            )
        });
    });
}

fn bench_parquet_perception(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let file_path = Path::new("test_data/OpenOrca/1M-GPT4-Augmented.parquet");

    if !file_path.exists() {
        return;
    }

    let options = FileReaderOptions {
        head: Some(100),
        file_type_override: None,
        output_mode: OutputMode::Default,
        output_format: OutputFormat::Json,
        pii_redaction: false,
        zero_copy: true,
        recursive: false,
        filter_exts: None,
        output_path: None,
    };

    c.bench_function("parquet_perception_1gb_sample_100", |b| {
        b.to_async(&rt).iter(|| {
            read_file_content(
                std::hint::black_box(file_path),
                std::hint::black_box(options.clone()),
            )
        });
    });
}

fn custom_criterion() -> Criterion {
    Criterion::default().measurement_time(std::time::Duration::new(30, 0))
}

criterion_group!(
    name = benches;
    config = custom_criterion();
    targets = bench_json_perception, bench_parquet_perception
);
criterion_main!(benches);
