use procmod_core::Process;
use std::fs::File;
use std::io::Write;
use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};

pub fn dump_full_process_memory(pid: i32, output_path: &str) -> Result<()> {
    let process = Process::attach(pid as u32)?;
    let regions = process.regions()?;
    let mut output_file = File::create(output_path)?;

    let bar = ProgressBar::new(regions.len() as u64);
    bar.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} Regions {percent}%")?
            .progress_chars("##-"),
    );

    for r in regions {
        if r.protection.read {
            if let Ok(buffer) = process.read_bytes(r.base as usize, r.size as usize) {
                output_file.write_all(&buffer)?;
            }
        }
        bar.inc(1);
    }
    bar.finish_with_message("✅ Dump completed!");
    Ok(())
}