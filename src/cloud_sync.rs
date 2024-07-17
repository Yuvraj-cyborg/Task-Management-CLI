use futures::TryStreamExt;
use mongodb::{bson::doc, Client, Collection};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub description: String,
    pub completed: bool,
}

pub struct CloudSync {
    collection: Collection<Task>,
}

impl CloudSync {
    pub async fn new(db_url: &str, db_name: &str, collection_name: &str) -> Self {
        let client = Client::with_uri_str(db_url).await.unwrap();
        let database = client.database(db_name);
        let collection = database.collection::<Task>(collection_name);

        CloudSync { collection }
    }

    pub async fn push(&self, tasks: Vec<Task>) -> mongodb::error::Result<()> {
        self.collection.delete_many(doc! {}, None).await?;
        self.collection.insert_many(tasks, None).await?;
        Ok(())
    }

    pub async fn pull(&self) -> mongodb::error::Result<Vec<Task>> {
        let cursor = self.collection.find(doc! {}, None).await?;
        let tasks: Vec<Task> = cursor.try_collect().await.unwrap();
        Ok(tasks)
    }
}

impl From<crate::tasks::task_manager::Task> for Task {
    fn from(item: crate::tasks::task_manager::Task) -> Self {
        Task {
            // Assuming both structs have the same fields `description` and `completed`
            description: item.description,
            completed: item.completed,
        }
    }
}
