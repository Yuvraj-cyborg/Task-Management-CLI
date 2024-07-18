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

    let db_url = "mongodb+srv://Sambit:Sambit@rust-cli.mvjq4zh.mongodb.net/";
    let client = Arc::new(Client::with_uri_str(db_url).await.unwrap());
    let cloud_sync = CloudSync::new(client.clone(), "task_db", "tasks")
        .await
        .unwrap();

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
        Some(("add", sub_matches)) => {
            let list_name = sub_matches.get_one::<String>("list_name").unwrap();
            let item = sub_matches.get_one::<String>("item").unwrap();
            let task = Task {
                description: item.to_string(),
                completed: false,
            };
            local_storage.add_task(list_name, task);
        }
        Some(("show", sub_matches)) => {
            if sub_matches.is_present("list_name") {
                let list_name = sub_matches.get_one::<String>("list_name").unwrap();
                if sub_matches.is_present("completed") {
                    let tasks = local_storage.get_completed_tasks(list_name);
                    for task in tasks {
                        println!("{:?}", task);
                    }
                } else if sub_matches.is_present("incomplete") {
                    let tasks = local_storage.get_incomplete_tasks(list_name);
                    for task in tasks {
                        println!("{:?}", task);
                    }
                } else {
                    let tasks = local_storage.get_tasks(list_name);
                    for task in tasks {
                        println!("{:?}", task);
                    }
                }
            } else {
                if sub_matches.is_present("all") {
                    let all_tasks = local_storage.get_all_tasks();
                    for (list, tasks) in all_tasks {
                        println!("List: {}", list);
                        for task in tasks {
                            println!("{:?}", task);
                        }
                    }
                } else if sub_matches.is_present("completed") {
                    let tasks = local_storage.get_all_completed_tasks();
                    for task in tasks {
                        println!("{:?}", task);
                    }
                } else if sub_matches.is_present("incomplete") {
                    let tasks = local_storage.get_all_incomplete_tasks();
                    for task in tasks {
                        println!("{:?}", task);
                    }
                } else {
                    let lists = local_storage.get_all_list_names();
                    for list in lists {
                        println!("{}", list);
                    }
                }
            }
        }
        Some(("complete", sub_matches)) => {
            let list_name = sub_matches.get_one::<String>("list_name").unwrap();
            let item_number = sub_matches.get_one::<usize>("item_number").unwrap();
            local_storage.complete_task(list_name, *item_number);
        }
        Some(("incomplete", sub_matches)) => {
            let list_name = sub_matches.get_one::<String>("list_name").unwrap();
            let item_number = sub_matches.get_one::<usize>("item_number").unwrap();
            local_storage.incomplete_task(list_name, *item_number);
        }
        Some(("remove", sub_matches)) => {
            if let Some(list_name) = sub_matches.get_one::<String>("list_name") {
                if let Some(item_number) = sub_matches.get_one::<usize>("item_number") {
                    local_storage.remove_task(list_name, *item_number);
                } else {
                    local_storage.remove_list(list_name);
                }
            } else {
                local_storage.remove_all_lists();
            }
        }
        Some(("push", _)) => {
            let tasks = local_storage.get_all_tasks();
            match cloud_sync.push(tasks).await {
                Ok(_) => println!("Tasks pushed to cloud successfully."),
                Err(e) => eprintln!("Failed to push tasks to cloud: {:?}", e),
            }
        }
        Some(("pull", _)) => match cloud_sync.pull().await {
            Ok(tasks) => {
                for (list_name, task_list) in tasks {
                    for task in task_list {
                        local_storage.add_task(&list_name, task);
                    }
                }
                println!("Tasks pulled from cloud successfully.");
            }
            Err(e) => eprintln!("Failed to pull tasks from cloud: {:?}", e),
        },
        _ => {}
    }
}
