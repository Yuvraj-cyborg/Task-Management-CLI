use clap::{Arg, Command};

pub fn build_cli() -> Command {
    Command::new("Minor Project")
        .version("1.0")
        .author("Your Name <youremail@example.com>")
        .about("Task Management CLI")
        .subcommand(
            Command::new("register")
                .about("Register a new user")
                .arg(Arg::new("username").required(true))
                .arg(Arg::new("password").required(true)),
        )
        .subcommand(
            Command::new("login")
                .about("Login a user")
                .arg(Arg::new("username").required(true))
                .arg(Arg::new("password").required(true)),
        )
        .subcommand(
            Command::new("add_task")
                .about("Add a new task")
                .arg(Arg::new("description").required(true)),
        )
        .subcommand(Command::new("list_tasks").about("List all tasks"))
        .subcommand(
            Command::new("complete_task")
                .about("Mark a task as complete")
                .arg(Arg::new("description").required(true)),
        )
        .subcommand(
            Command::new("incomplete_task")
                .about("Mark a task as incomplete")
                .arg(Arg::new("description").required(true)),
        )
        .subcommand(
            Command::new("remove_task")
                .about("Remove a task")
                .arg(Arg::new("description").required(true)),
        )
        .subcommand(Command::new("push").about("Push local tasks to cloud"))
        .subcommand(Command::new("pull").about("Pull tasks from cloud"))
}
