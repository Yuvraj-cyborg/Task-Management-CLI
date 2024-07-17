// use async_trait::async_trait;
use bcrypt::{hash, verify, DEFAULT_COST};
use mongodb::{bson::doc, Client, Collection};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub username: String,
    pub password: String,
}

pub struct AuthManager {
    collection: Collection<User>,
}

impl AuthManager {
    pub async fn new(db_url: &str, db_name: &str, collection_name: &str) -> Self {
        let client = Client::with_uri_str(db_url).await.unwrap();
        let database = client.database(db_name);
        let collection = database.collection::<User>(collection_name);

        AuthManager { collection }
    }

    pub async fn register(&self, username: &str, password: &str) -> mongodb::error::Result<()> {
        let hashed_password = hash(password, DEFAULT_COST).unwrap();
        let new_user = User {
            username: username.to_string(),
            password: hashed_password,
        };

        self.collection.insert_one(new_user, None).await?;
        Ok(())
    }

    pub async fn login(&self, username: &str, password: &str) -> mongodb::error::Result<bool> {
        let filter = doc! { "username": username };
        if let Some(user) = self.collection.find_one(filter, None).await? {
            return Ok(verify(password, &user.password).unwrap());
        }
        Ok(false)
    }
}
