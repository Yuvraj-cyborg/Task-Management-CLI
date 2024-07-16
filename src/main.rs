mod auth;
mod utils;

use auth::auth_manager::AuthManager;
use mongodb::bson::doc;
use mongodb::options::{ClientOptions, ServerApi, ServerApiVersion};
use mongodb::Client;
use tokio;
use utils::cli_parser::build_cli;

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let matches = build_cli().get_matches();

    let db_url = "mongodb+srv://Sambit:Nibedita%401981Singha@rust-cli.3lcghw1.mongodb.net/?retryWrites=true&w=majority";
    let db_name = "minor_project_db";
    let collection_name = "users";

    let mut client_options = ClientOptions::parse(db_url).await?;
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);

    let client = Client::with_options(client_options)?;
    client
        .database("admin")
        .run_command(doc! {"ping": 1}, None)
        .await?;
    println!("Pinged your deployment. You successfully connected to MongoDB!");

    let auth_manager = AuthManager::new(db_url, db_name, collection_name).await;

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
            Ok(_) => println!("User logged in successfully."),
            Err(e) => eprintln!("Failed to login user: {}", e),
        }
    }

    Ok(())
}
