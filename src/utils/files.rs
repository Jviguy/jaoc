use anyhow::{Context, Result};
use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

pub(crate) fn create_empty_file(path: &PathBuf) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("🔥 Failed to create data file: {}")
    };

    match File::create_new(path) {
        Ok(_) => println!("✅ Created data file: {:?}", path),
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
            eprintln!("⚠️ Data file already exists: {:?}", path);
        }
        Err(e) => eprintln!("🔥 Failed to create data file: {}", e),
    };
}

pub(crate) fn write_to_bin_file(path: &PathBuf, content: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .context(format!("Failed to create parent directories: {:?}", parent))?;
    }

    match File::create_new(path) {
        Ok(mut file) => {
            file.write_all(content.as_bytes())
                .context(format!("Failed to write to bin file: {:?}", path))?;
            println!("✅ Created binary: {:?}", path);
            Ok(())
        }
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
            eprintln!("⚠️ Binary file already exists: {:?}", path);
            Ok(())
        }
        Err(e) => Err(e).context(format!("Failed to create bin file: {:?}", path)),
    }
}
