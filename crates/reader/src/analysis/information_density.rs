use std::char;

/// Calculates a simple information density score for a given byte buffer.
///
/// This implementation counts printable ASCII characters (excluding whitespace)
/// and divides by the total buffer size. A higher score indicates more
/// non-whitespace printable characters, suggesting a higher "density" of information.
pub fn calculate_information_density(buffer: &[u8]) -> f64 {
    if buffer.is_empty() {
        return 0.0;
    }

    let printable_chars_count = buffer
        .iter()
        .filter(|&&b| {
            let c = b as char;
            c.is_ascii_graphic() // Includes alphanumeric, symbols, etc., but not whitespace
        })
        .count();

    (printable_chars_count as f64) / (buffer.len() as f64)
}
