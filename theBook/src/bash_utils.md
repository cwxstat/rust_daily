# Bash Utils


```rust,editable
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let home_dir = env::var("HOME").expect("Failed to get user's home directory");
    let downloads_dir = format!("{}/Downloads", home_dir);
    let pdf_dir = format!("{}/pdf", downloads_dir);

    // Create the directory if it doesn't exist
    if let Err(_) = fs::create_dir_all(&pdf_dir) {
        eprintln!("Failed to create directory: {}", pdf_dir);
        return;
    }

    let entries = match fs::read_dir(&downloads_dir) {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!("Failed to read directory: {}\nError: {}", downloads_dir, e);
            return;
        }
    };

    let mut file_count = 0;
    let mut total_bytes_moved = 0;

    for entry in entries {
        if let Ok(entry) = entry {
            let metadata = match entry.metadata() {
                Ok(metadata) => metadata,
                Err(e) => {
                    eprintln!("Failed to get metadata: {}", e);
                    continue;
                }
            };

            if metadata.is_file() {
                let file_name = entry.file_name();
                let file_name_str = file_name.to_string_lossy();

                if file_name_str.contains("pdf") {
                    let source_path = entry.path();
                    let destination_path = PathBuf::from(&pdf_dir).join(file_name.clone());

                    match entry.metadata() {
                        Ok(metadata) => {
                            let file_size = metadata.len();

                            if let Err(e) = fs::rename(&source_path, &destination_path) {
                                eprintln!("Failed to move file: {}\nError: {}", file_name_str, e);
                            } else {
                                println!("Moved file: {}", file_name_str);
                                file_count += 1;
                                total_bytes_moved += file_size;
                            }
                        }
                        Err(e) => {
                            eprintln!(
                                "Failed to get metadata for file: {}\nError: {}",
                                file_name_str, e
                            );
                        }
                    }
                }
            }
        }
    }

    println!(
        "Moved {} file(s), total bytes: {}",
        file_count, total_bytes_moved
    );
}


```