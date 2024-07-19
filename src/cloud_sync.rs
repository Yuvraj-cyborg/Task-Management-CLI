use crate::task::Task;
use futures::stream::TryStreamExt;
use mongodb::{bson::doc, bson::to_document, Client, Collection};
use std::sync::Arc;

pub struct CloudSync {
    collection: Collection<Task>,
}

impl CloudSync {
    pub async fn new(client: Arc<Client>, db_name: &str, collection_name: &str) -> Self {
        let database = client.database(db_name);
        let collection = database.collection::<Task>(collection_name);

        CloudSync { collection }
    }

    pub async fn push(&self, tasks: Vec<Task>) -> mongodb::error::Result<()> {
        for task in tasks {
            let filter = doc! { "description": &task.description };
            let update_doc = doc! { "$set": to_document(&task).unwrap() };
            self.collection.update_one(filter, update_doc, None).await?;
        }
        Ok(())
    }

    pub async fn pull(&self) -> mongodb::error::Result<Vec<Task>> {
        let cursor = self.collection.find(None, None).await?;
        let tasks: Vec<Task> = cursor.try_collect().await?;
        Ok(tasks)
    }
}
