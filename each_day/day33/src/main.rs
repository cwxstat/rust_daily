// Define an abstraction (trait) for data storage
pub trait DataStorage {
    fn save_data(&self, data: &str) -> Result<(), String>;
}

// Implement a low-level module for file storage
pub struct FileStorage;

impl DataStorage for FileStorage {
    fn save_data(&self, data: &str) -> Result<(), String> {
        println!("Saving data to a file: {}", data);
        // Implement file saving logic here
        Ok(())
    }
}

// Implement another low-level module for cloud storage
pub struct CloudStorage;

impl DataStorage for CloudStorage {
    fn save_data(&self, data: &str) -> Result<(), String> {
        println!("Saving data to the cloud: {}", data);
        // Implement cloud saving logic here
        Ok(())
    }
}

// High-level module that depends on the abstraction (trait) rather than concrete implementations
pub struct DataManager<T: DataStorage> {
    storage: T,
}

impl<T: DataStorage> DataManager<T> {
    pub fn new(storage: T) -> Self {
        DataManager { storage }
    }

    pub fn save_data(&self, data: &str) -> Result<(), String> {
        self.storage.save_data(data)
    }
}

fn main() {
    // Use FileStorage with DataManager
    let file_storage = FileStorage;
    let data_manager_file = DataManager::new(file_storage);
    data_manager_file.save_data("Some data for file storage").unwrap();

    // Use CloudStorage with DataManager
    let cloud_storage = CloudStorage;
    let data_manager_cloud = DataManager::new(cloud_storage);
    data_manager_cloud.save_data("Some data for cloud storage").unwrap();
}
