use futures::TryStreamExt;
use mongodb::{bson::doc, Client, Collection};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub list_name: String,
    pub description: String,
    pub completed: bool,
}

pub struct TaskManager {
    collection: Collection<Task>,
}

impl TaskManager {
    pub async fn new(db_url: &str, db_name: &str, collection_name: &str) -> Self {
        let client = Client::with_uri_str(db_url).await.unwrap();
        let database = client.database(db_name);
        let collection = database.collection::<Task>(collection_name);

        TaskManager { collection }
    }

    pub async fn add_task(&self, list_name: &str, description: &str) -> mongodb::error::Result<()> {
        let new_task = Task {
            list_name: list_name.to_string(),
            description: description.to_string(),
            completed: false,
        };

        self.collection.insert_one(new_task, None).await?;
        Ok(())
    }

    pub async fn list_tasks(&self) -> mongodb::error::Result<Vec<Task>> {
        let cursor = self.collection.find(doc! {}, None).await?;
        let tasks: Vec<Task> = cursor.try_collect().await.unwrap();
        Ok(tasks)
    }

    pub async fn complete_task(
        &self,
        list_name: &str,
        description: &str,
    ) -> mongodb::error::Result<()> {
        self.collection
            .update_one(
                doc! { "list_name": list_name, "description": description },
                doc! { "$set": { "completed": true } },
                None,
            )
            .await?;
        Ok(())
    }

    pub async fn incomplete_task(
        &self,
        list_name: &str,
        description: &str,
    ) -> mongodb::error::Result<()> {
        self.collection
            .update_one(
                doc! { "list_name": list_name, "description": description },
                doc! { "$set": { "completed": false } },
                None,
            )
            .await?;
        Ok(())
    }

    pub async fn remove_task(
        &self,
        list_name: &str,
        description: &str,
    ) -> mongodb::error::Result<()> {
        self.collection
            .delete_one(
                doc! { "list_name": list_name, "description": description },
                None,
            )
            .await?;
        Ok(())
    }
}
