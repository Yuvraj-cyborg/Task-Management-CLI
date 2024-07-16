use bcrypt::{hash, verify, DEFAULT_COST};
use mongodb::{bson::doc, options::ClientOptions, Client, Collection};

use super::user::User;

pub struct AuthManager {
    collection: Collection<User>,
}

impl AuthManager {
    pub async fn new(db_url: &str, db_name: &str, collection_name: &str) -> Self {
        let client_options = ClientOptions::parse(db_url).await.unwrap();
        let client = Client::with_options(client_options).unwrap();
        let database = client.database(db_name);
        let collection = database.collection::<User>(collection_name);

        AuthManager { collection }
    }

    pub async fn register(&self, username: &str, password: &str) -> Result<(), String> {
        let password_hash = hash(password, DEFAULT_COST).unwrap();

        let new_user = User {
            username: username.to_string(),
            password_hash,
        };

        match self.collection.insert_one(new_user, None).await {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Failed to register user: {}", e)),
        }
    }

    pub async fn login(&self, username: &str, password: &str) -> Result<(), String> {
        let filter = doc! { "username": username };

        match self.collection.find_one(filter, None).await {
            Ok(Some(user)) => {
                if verify(password, &user.password_hash).unwrap() {
                    Ok(())
                } else {
                    Err("Invalid username or password".to_string())
                }
            }
            Ok(None) => Err("Invalid username or password".to_string()),
            Err(e) => Err(format!("Failed to login user: {}", e)),
        }
    }
}
