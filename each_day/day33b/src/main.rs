use std::env;
use std::fs::{self, DirEntry};
use std::io;
use std::path::Path;
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let home = env::var("HOME").expect("HOME environment variable not found");
    let downloads = Path::new(&home).join("Downloads");
    let extensions = vec!["zip", "pdf", "txt", "png", "jpg"];

    let mut file_count = 0;
    let mut bytes_written: HashMap<&str, u64> = HashMap::new();
    let mut bytes_in_dir: HashMap<&str, u64> = HashMap::new();

    for extension in &extensions {
        let target_dir = downloads.join(extension);
        fs::create_dir_all(&target_dir)?;
        bytes_written.insert(extension, 0);
        bytes_in_dir.insert(extension, 0);
    }

    for entry in fs::read_dir(&downloads)? {
        let entry: DirEntry = entry?;
        let path = entry.path();
        if let Some(extension) = path.extension() {
            if let Some(extension_str) = extension.to_str() {
                if extensions.contains(&extension_str) {
                    let target_dir = downloads.join(extension_str);
                    let target_path = target_dir.join(path.file_name().unwrap());
                    let metadata = fs::metadata(&path)?;
                    let file_size = metadata.len();
                    fs::rename(&path, &target_path)?;

                    file_count += 1;
                    *bytes_written.get_mut(extension_str).unwrap() += file_size;
                    *bytes_in_dir.get_mut(extension_str).unwrap() += file_size;
                }
            }
        }
    }

    println!("Total files copied: {}", file_count);
    for extension in &extensions {
        let bytes_written_str = bytes_written.get(extension).unwrap();
        let bytes_in_dir_str = bytes_in_dir.get(extension).unwrap();
        println!(
            "{}: bytes_written = {}, bytes_in_dir = {}",
            extension, bytes_written_str, bytes_in_dir_str
        );
    }

    Ok(())
}
