mod auth;
mod cloud_sync;
mod tasks;
mod utils;

use auth::auth_manager::AuthManager;
use cloud_sync::CloudSync; // Corrected import path
use tasks::task_manager::TaskManager;
use utils::cli_parser::build_cli;

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let matches = build_cli().get_matches();

    let db_url = "mongodb://localhost:27017";
    let db_name = "minor_project_db";
    let user_collection = "users";
    let task_collection = "tasks";

    let auth_manager = AuthManager::new(db_url, db_name, user_collection).await;
    let task_manager = TaskManager::new(db_url, db_name, task_collection).await;
    let cloud_sync = CloudSync::new(db_url, db_name, task_collection).await;

    if let Some(matches) = matches.subcommand_matches("register") {
        let username = matches.get_one::<String>("username").unwrap();
        let password = matches.get_one::<String>("password").unwrap();
        match auth_manager.register(username, password).await {
            Ok(_) => println!("User registered successfully."),
            Err(e) => eprintln!("Failed to register user: {}", e),
        }
    } else if let Some(matches) = matches.subcommand_matches("login") {
        let username = matches.get_one::<String>("username").unwrap();
        let password = matches.get_one::<String>("password").unwrap();
        match auth_manager.login(username, password).await {
            Ok(logged_in) => {
                if logged_in {
                    println!("User logged in successfully.");
                } else {
                    println!("Invalid username or password.");
                }
            }
            Err(e) => eprintln!("Failed to login user: {}", e),
        }
    } else if let Some(matches) = matches.subcommand_matches("add_task") {
        let description = matches.get_one::<String>("description").unwrap();
        match task_manager.add_task(description).await {
            Ok(_) => println!("Task added successfully."),
            Err(e) => eprintln!("Failed to add task: {}", e),
        }
    } else if let Some(_) = matches.subcommand_matches("list_tasks") {
        match task_manager.list_tasks().await {
            Ok(tasks) => {
                for task in tasks {
                    println!("{:?}", task);
                }
            }
            Err(e) => eprintln!("Failed to list tasks: {}", e),
        }
    } else if let Some(matches) = matches.subcommand_matches("complete_task") {
        let description = matches.get_one::<String>("description").unwrap();
        match task_manager.complete_task(description).await {
            Ok(_) => println!("Task marked as complete."),
            Err(e) => eprintln!("Failed to complete task: {}", e),
        }
    } else if let Some(matches) = matches.subcommand_matches("incomplete_task") {
        let description = matches.get_one::<String>("description").unwrap();
        match task_manager.incomplete_task(description).await {
            Ok(_) => println!("Task marked as incomplete."),
            Err(e) => eprintln!("Failed to mark task as incomplete: {}", e),
        }
    } else if let Some(matches) = matches.subcommand_matches("remove_task") {
        let description = matches.get_one::<String>("description").unwrap();
        match task_manager.remove_task(description).await {
            Ok(_) => println!("Task removed successfully."),
            Err(e) => eprintln!("Failed to remove task: {}", e),
        }
    } else if let Some(_) = matches.subcommand_matches("push") {
        let tasks_to_push: Vec<cloud_sync::Task> = task_manager
            .list_tasks()
            .await
            .unwrap()
            .into_iter()
            .map(|task| task.into())
            .collect();

        match cloud_sync.push(tasks_to_push).await {
            Ok(_) => println!("Tasks pushed to cloud successfully."),
            Err(e) => eprintln!("Failed to push tasks to cloud: {}", e),
        }
    } else if let Some(_) = matches.subcommand_matches("pull") {
        match cloud_sync.pull().await {
            Ok(tasks) => {
                for task in tasks {
                    println!("{:?}", task);
                }
            }
            Err(e) => eprintln!("Failed to pull tasks from cloud: {}", e),
        }
    }

    Ok(())
}
