
use notify::{Watcher,  RecursiveMode, Result};
use std::path::Path;
use std::thread;
use std::time::Duration;

fn main() -> Result<()> {
    // Automatically select the best implementation for your platform.
    let mut watcher = notify::recommended_watcher(|res| {
        match res {
            Ok(event) => println!("event: {:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }
    })?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(Path::new("./doc"), RecursiveMode::Recursive)?;
    thread::sleep(Duration::from_secs(24 * 60 * 60));

    Ok(())
}