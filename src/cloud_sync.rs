use crate::task::Task;
use futures::stream::TryStreamExt;
use mongodb::{bson::doc, bson::to_document, Client, Collection};
use std::collections::HashMap;
use std::sync::Arc;

pub struct CloudSync {
    collection: Collection<Task>,
}

impl CloudSync {
    pub async fn new(
        client: Arc<Client>,
        db_name: &str,
        collection_name: &str,
    ) -> mongodb::error::Result<Self> {
        let database = client.database(db_name);
        let collection = database.collection::<Task>(collection_name);

        Ok(CloudSync { collection })
    }

    pub async fn push(&self, tasks: HashMap<String, Vec<Task>>) -> mongodb::error::Result<()> {
        for (list_name, task_list) in tasks {
            for task in task_list {
                let filter = doc! { "description": &task.description, "list_name": &list_name };
                let update_doc = doc! { "$set": to_document(&task).unwrap() };
                self.collection.update_one(filter, update_doc, None).await?;
            }
        }
        Ok(())
    }

    pub async fn pull(&self) -> mongodb::error::Result<HashMap<String, Vec<Task>>> {
        let cursor = self.collection.find(None, None).await?;
        let mut tasks_by_list = HashMap::new();
        let tasks: Vec<Task> = cursor.try_collect().await?;
        for task in tasks {
            tasks_by_list
                .entry(task.list_name.clone())
                .or_insert_with(Vec::new)
                .push(task);
        }
        Ok(tasks_by_list)
    }
}
