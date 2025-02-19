use std::{fs::File, io::Read, io::Write};

pub struct DB {
    path: String,
    data: String,
}

impl DB {
    pub fn new(path: String, data: String) -> Self {
        Self { path, data }
    }

    pub fn save(&self) -> String {
        let mut file = File::create(format!("./data/{}.json", self.path)).unwrap();
        file.write_all(self.data.as_bytes()).unwrap();
        self.path.clone()
    }

    pub fn find<T: serde::de::DeserializeOwned>(id: &str) -> Option<T> {
        let mut file = File::open(format!("./data/{}.json", id)).unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        serde_json::from_str::<T>(&data).ok()
    }

    pub fn list<T: serde::de::DeserializeOwned>(path: &str) -> Vec<T> {
        let files = std::fs::read_dir(path).unwrap();
        files
            .map(|file| {
                let content = std::fs::read_to_string(file.unwrap().path()).unwrap();
                serde_json::from_str::<T>(&content).unwrap()
            })
            .collect()
    }

    pub fn update<T: serde::de::DeserializeOwned + serde::Serialize>(
        id: &str,
        data: T,
    ) -> Option<T> {
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(format!("./data/{}.json", id))
            .unwrap();

        file.write_all(serde_json::to_string(&data).unwrap().as_bytes())
            .unwrap();
        Some(data)
    }

    pub fn delete(path: &str) -> bool {
        std::fs::remove_file(format!("./data/{}.json", path)).is_ok()
    }
}
