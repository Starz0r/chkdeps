use std::{fs::File, io::Read, path::Path};

use anyhow::{Context, Result};

pub fn file_contents<P: AsRef<Path> + std::fmt::Debug>(path: P) -> Result<String> {
    let mut f = File::open(&path)?;
    let mut buf = Vec::new();
    f.read_to_end(&mut buf)?;
    String::from_utf8(buf)
        .with_context(|| format!("File at {path:?} did not have valid UTF-8 characters."))
}
