// Calculates Shannon Entropy to identify random-looking strings (like tokens)
pub fn calculate_shannon_entropy(data: &[u8]) -> f64 {
    let mut frequency = [0u32; 256];
    for &byte in data {
        frequency[byte as usize] += 1;
    }
    let len = data.len() as f64;
    let mut entropy = 0.0;
    for &count in frequency.iter() {
        if count > 0 {
            let p = count as f64 / len;
            entropy -= p * p.log2();
        }
    }
    entropy
}

// Checks if a byte slice is likely a high-entropy token (e.g., API Key, JWT)
pub fn is_high_entropy_token(data: &[u8], threshold: f64) -> bool {
    // 1. Must be mostly printable ASCII (Base64, Hex, etc.)
    let printable_ratio = data
        .iter()
        .filter(|&&b| b.is_ascii_graphic() || b == b' ' || b == b'\n' || b == b'\r')
        .count() as f64
        / data.len() as f64;

    // 2. Must have high randomness (typical tokens have entropy > 5.5)
    printable_ratio > 0.8 && calculate_shannon_entropy(data) > threshold
}