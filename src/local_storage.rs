use crate::task::Task;
use sled::Db;

pub struct LocalStorage {
    pub db: Db,
}

impl LocalStorage {
    pub fn new(db_path: &str) -> Self {
        let db = sled::open(db_path).unwrap();
        Self { db }
    }

    pub fn add_task(&self, task: Task) {
        let tasks = self.db.open_tree("tasks").unwrap();
        tasks
            .insert(task.description.clone(), serde_json::to_vec(&task).unwrap())
            .unwrap();
    }

    pub fn get_tasks(&self) -> Vec<Task> {
        let tasks = self.db.open_tree("tasks").unwrap();
        tasks
            .iter()
            .values()
            .map(|v| serde_json::from_slice(&v.unwrap()).unwrap())
            .collect()
    }

    pub fn remove_task(&self, description: &str) {
        let tasks = self.db.open_tree("tasks").unwrap();
        tasks.remove(description).unwrap();
    }
}
