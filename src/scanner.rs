use memchr::memmem;
use procmod_core::Process;
use crate::entropy::is_high_entropy_token;
use anyhow::Result;

pub fn scan_process_for_pattern(
    pid: i32,
    search_pattern: &[u8],
    entropy_threshold: f64,
) -> Result<Vec<String>> {
    let process = Process::attach(pid as u32)?;
    let regions = process.regions()?;

    let mut found_tokens = Vec::new();

    for r in regions {
        if !r.protection.read {
            continue;
        }
        if let Ok(buffer) = process.read_bytes(r.base as usize, r.size as usize) {
            let mut start_idx = 0;
            while let Some(pos) = memmem::find(&buffer[start_idx..], search_pattern) {
                let absolute_pos = start_idx + pos;
                let ctx_start = absolute_pos.saturating_sub(32);
                let ctx_end = (absolute_pos + search_pattern.len() + 64).min(buffer.len());
                if let Ok(string_slice) = String::from_utf8(buffer[ctx_start..ctx_end].to_vec()) {
                    if is_high_entropy_token(string_slice.as_bytes(), entropy_threshold) {
                        found_tokens.push(string_slice);
                    }
                }
                start_idx = absolute_pos + 1;
            }
        }
    }

    Ok(found_tokens)
}