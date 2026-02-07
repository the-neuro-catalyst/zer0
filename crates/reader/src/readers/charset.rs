// crates/reader/src/readers/charset.rs
#[cfg(feature = "extra-docs")]
use chardetng::EncodingDetector;
#[cfg(feature = "extra-docs")]
use encoding_rs::Encoding;

#[cfg(feature = "extra-docs")]
pub fn detect_encoding_from_buffer(buffer: &[u8]) -> &'static Encoding {
    let mut detector = EncodingDetector::new();
    detector.feed(buffer, true);
    detector.guess(None, true)
}

#[allow(dead_code)]
pub fn decode_to_string(bytes: &[u8]) -> String {
    #[cfg(feature = "extra-docs")]
    {
        let encoding = detect_encoding_from_buffer(bytes);
        let (res, _, _) = encoding.decode(bytes);
        res.into_owned()
    }
    #[cfg(not(feature = "extra-docs"))]
    {
        String::from_utf8_lossy(bytes).into_owned()
    }
}

pub fn get_decoded_reader(file: std::fs::File) -> std::io::Result<Box<dyn std::io::Read + Send>> {
    #[cfg(feature = "extra-docs")]
    {
        use chardetng::EncodingDetector;

        use std::io::Read;
        let mut detector = EncodingDetector::new();
        let mut buffer = [0u8; 4096];

        let mut sniff_reader = &file;
        let bytes_read = sniff_reader.read(&mut buffer)?;
        detector.feed(&buffer[..bytes_read], bytes_read < buffer.len());
        let encoding = detector.guess(None, true);

        use std::io::{Seek, SeekFrom};
        let mut file_to_reset = file;
        file_to_reset.seek(SeekFrom::Start(0))?;

        Ok(Box::new(
            encoding_rs_io::DecodeReaderBytesBuilder::new()
                .encoding(Some(encoding))
                .build(file_to_reset),
        ))
    }
    #[cfg(not(feature = "extra-docs"))]
    {
        Ok(Box::new(file))
    }
}
