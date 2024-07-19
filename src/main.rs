mod auth;
mod cloud_sync;
mod local_storage;
mod task;
mod utils;

use auth::user::UserManager;
use cloud_sync::CloudSync;
use local_storage::LocalStorage;
use mongodb::Client;
use std::sync::Arc;
use task::Task;
use tokio::main;
use utils::cli_parser::build_cli;

#[main]
async fn main() {
    let cli = build_cli();
    let matches = cli.get_matches();

    let local_storage = LocalStorage::new("local_tasks.db");
    let user_manager = UserManager::new(local_storage.db.clone());

    // Hardcoded MongoDB Atlas URI
    let db_url = "mongodb+srv://Sambit:Sambit@rust-cli.mvjq4zh.mongodb.net/";
    let client = Arc::new(Client::with_uri_str(db_url).await.unwrap());
    let cloud_sync = CloudSync::new(client.clone(), "task_db", "tasks").await;

    match matches.subcommand() {
        Some(("register", sub_matches)) => {
            let username = sub_matches.get_one::<String>("username").unwrap();
            let password = sub_matches.get_one::<String>("password").unwrap();
            match user_manager.register_user(username, password) {
                Ok(_) => println!("User registered successfully."),
                Err(e) => println!("Failed to register user: {}", e),
            }
        }
        Some(("login", sub_matches)) => {
            let username = sub_matches.get_one::<String>("username").unwrap();
            let password = sub_matches.get_one::<String>("password").unwrap();
            match user_manager.login_user(username, password) {
                Ok(_) => println!("User logged in successfully."),
                Err(e) => println!("Failed to login: {}", e),
            }
        }
        Some(("logout", _)) => {
            user_manager.logout_user();
            println!("User logged out successfully.");
        }
        Some(("add_task", sub_matches)) => {
            let description = sub_matches.get_one::<String>("description").unwrap();
            let task = Task {
                description: description.to_string(),
                completed: false,
            };
            local_storage.add_task(task);
        }
        Some(("list_tasks", _)) => {
            let tasks = local_storage.get_tasks();
            for task in tasks {
                println!("{:?}", task);
            }
        }
        Some(("remove_task", sub_matches)) => {
            let description = sub_matches.get_one::<String>("description").unwrap();
            local_storage.remove_task(description);
        }
        Some(("push", _)) => {
            let tasks = local_storage.get_tasks();
            match cloud_sync.push(tasks).await {
                Ok(_) => println!("Tasks pushed to cloud successfully."),
                Err(e) => eprintln!("Failed to push tasks to cloud: {:?}", e),
            }
        }
        Some(("pull", _)) => match cloud_sync.pull().await {
            Ok(tasks) => {
                for task in tasks {
                    local_storage.add_task(Task {
                        description: task.description,
                        completed: task.completed,
                    });
                }
                println!("Tasks pulled from cloud successfully.");
            }
            Err(e) => eprintln!("Failed to pull tasks from cloud: {:?}", e),
        },
        _ => {}
    }
}
