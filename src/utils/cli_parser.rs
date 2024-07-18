use clap::{Arg, Command};

pub fn build_cli() -> Command {
    Command::new("Task Manager")
        .version("1.0")
        .author("Author Name <author@example.com>")
        .about("Manages tasks")
        .subcommand(
            Command::new("register")
                .about("Registers a new user")
                .arg(Arg::new("username").required(true))
                .arg(Arg::new("password").required(true)),
        )
        .subcommand(
            Command::new("login")
                .about("Logs in a user")
                .arg(Arg::new("username").required(true))
                .arg(Arg::new("password").required(true)),
        )
        .subcommand(Command::new("logout").about("Logs out the current user"))
        .subcommand(
            Command::new("add_task")
                .about("Adds a new task")
                .arg(Arg::new("description").required(true)),
        )
        .subcommand(Command::new("list_tasks").about("Lists all tasks"))
        .subcommand(
            Command::new("remove_task")
                .about("Removes a task")
                .arg(Arg::new("description").required(true)),
        )
        .subcommand(Command::new("push").about("Pushes local tasks to the cloud"))
        .subcommand(Command::new("pull").about("Pulls tasks from the cloud to local storage"))
}
