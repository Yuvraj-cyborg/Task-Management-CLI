use serde::{Deserialize, Serialize};
use sled::Db;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub username: String,
    pub password: String,
}

pub struct UserManager {
    db: Db,
}

impl UserManager {
    pub fn new(db: Db) -> Self {
        Self { db }
    }

    pub fn register_user(&self, username: &str, password: &str) -> Result<(), String> {
        let users = self.db.open_tree("users").unwrap();
        if users.contains_key(username).unwrap() {
            return Err("User already exists".to_string());
        }

        let user = User {
            username: username.to_string(),
            password: password.to_string(),
        };

        users
            .insert(username, serde_json::to_vec(&user).unwrap())
            .unwrap();
        Ok(())
    }

    pub fn login_user(&self, username: &str, password: &str) -> Result<(), String> {
        let users = self.db.open_tree("users").unwrap();
        if let Some(user_data) = users.get(username).unwrap() {
            let user: User = serde_json::from_slice(&user_data).unwrap();
            if user.password == password {
                return Ok(());
            } else {
                return Err("Incorrect password".to_string());
            }
        }
        Err("User not found".to_string())
    }

    pub fn logout_user(&self) {}
}
