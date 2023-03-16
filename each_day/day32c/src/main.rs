use std::process::Command;
use std::env;
use std::str;
use std::fs;

fn main() {
    let home_dir = env::var("HOME").expect("Failed to get user's home directory");
    let downloads_dir = format!("{}/Downloads", home_dir);
    let pdf_dir = format!("{}/pdf", downloads_dir);

    // Create the spork directory if it doesn't exist
    if let Err(_) = fs::create_dir_all(&pdf_dir) {
        eprintln!("Failed to create directory: {}", pdf_dir);
        return;
    }

    let output = Command::new("ls")
        .arg("-l")
        .arg(&downloads_dir)
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        let result = str::from_utf8(&output.stdout).unwrap();
        let pdf_files = result
            .lines()
            .filter(|line| line.contains("pdf"))
            .collect::<Vec<&str>>();

        let mut file_count = 0;
        let mut total_bytes_moved = 0;

        for entry in pdf_files {
            let fields: Vec<&str> = entry.split_whitespace().collect();
            let file_size: u64 = fields[4].parse().unwrap();
            let file_name = fields.last().unwrap();

            let source_path = format!("{}/{}", downloads_dir, file_name);
            let destination_path = format!("{}/{}", pdf_dir, file_name);

            if let Err(e) = fs::rename(&source_path, &destination_path) {
                eprintln!("Failed to move file: {}\nError: {}", file_name, e);
            } else {
                println!("Moved file: {}", file_name);
                file_count += 1;
                total_bytes_moved += file_size;
            }
        }

        println!("Moved {} file(s), total bytes: {}", file_count, total_bytes_moved);
    } else {
        eprintln!("Error: {:?}", output.stderr);
    }
}
