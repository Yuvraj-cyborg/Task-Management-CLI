use crate::task::Task;
use sled::Db;
use std::collections::HashMap;

pub struct LocalStorage {
    pub db: Db,
}

impl LocalStorage {
    pub fn new(db_path: &str) -> Self {
        let db = sled::open(db_path).unwrap();
        Self { db }
    }

    pub fn add_task(&self, list_name: &str, task: Task) {
        let list_tree = self.db.open_tree(list_name).unwrap();
        list_tree
            .insert(task.description.clone(), serde_json::to_vec(&task).unwrap())
            .unwrap();
    }

    pub fn get_tasks(&self, list_name: &str) -> Vec<Task> {
        let list_tree = self.db.open_tree(list_name).unwrap();
        list_tree
            .iter()
            .values()
            .map(|v| serde_json::from_slice(&v.unwrap()).unwrap())
            .collect()
    }

    pub fn get_completed_tasks(&self, list_name: &str) -> Vec<Task> {
        self.get_tasks(list_name)
            .into_iter()
            .filter(|task| task.completed)
            .collect()
    }

    pub fn get_incomplete_tasks(&self, list_name: &str) -> Vec<Task> {
        self.get_tasks(list_name)
            .into_iter()
            .filter(|task| !task.completed)
            .collect()
    }

    pub fn get_all_tasks(&self) -> HashMap<String, Vec<Task>> {
        self.db
            .tree_names()
            .iter()
            .map(|name| {
                let list_name = String::from_utf8(name.to_vec()).unwrap();
                (list_name.clone(), self.get_tasks(&list_name))
            })
            .collect()
    }

    pub fn get_all_completed_tasks(&self) -> Vec<Task> {
        self.get_all_tasks()
            .values()
            .flat_map(|tasks| tasks.iter().cloned().filter(|task| task.completed))
            .collect()
    }

    pub fn get_all_incomplete_tasks(&self) -> Vec<Task> {
        self.get_all_tasks()
            .values()
            .flat_map(|tasks| tasks.iter().cloned().filter(|task| !task.completed))
            .collect()
    }

    pub fn get_all_list_names(&self) -> Vec<String> {
        self.db
            .tree_names()
            .iter()
            .map(|name| String::from_utf8(name.to_vec()).unwrap())
            .collect()
    }

    pub fn complete_task(&self, list_name: &str, item_number: usize) {
        let mut tasks = self.get_tasks(list_name);
        if item_number < tasks.len() {
            tasks[item_number].completed = true;
            self.update_tasks(list_name, &tasks);
        }
    }

    pub fn incomplete_task(&self, list_name: &str, item_number: usize) {
        let mut tasks = self.get_tasks(list_name);
        if item_number < tasks.len() {
            tasks[item_number].completed = false;
            self.update_tasks(list_name, &tasks);
        }
    }

    pub fn remove_task(&self, list_name: &str, item_number: usize) {
        let mut tasks = self.get_tasks(list_name);
        if item_number < tasks.len() {
            tasks.remove(item_number);
            self.update_tasks(list_name, &tasks);
        }
    }

    pub fn remove_list(&self, list_name: &str) {
        self.db.drop_tree(list_name).unwrap();
    }

    pub fn remove_all_lists(&self) {
        for list_name in self.get_all_list_names() {
            self.remove_list(&list_name);
        }
    }

    fn update_tasks(&self, list_name: &str, tasks: &[Task]) {
        let list_tree = self.db.open_tree(list_name).unwrap();
        list_tree.clear().unwrap();
        for task in tasks {
            list_tree
                .insert(task.description.clone(), serde_json::to_vec(task).unwrap())
                .unwrap();
        }
    }
}
