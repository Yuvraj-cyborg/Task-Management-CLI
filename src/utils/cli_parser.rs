use clap::{App, Arg, SubCommand};

pub fn build_cli() -> App<'static, 'static> {
    App::new("Todo CLI")
        .version("1.0")
        .about("Manage your tasks")
        .subcommand(
            SubCommand::with_name("register")
                .about("Register a new user")
                .arg(Arg::with_name("username").required(true))
                .arg(Arg::with_name("password").required(true)),
        )
        .subcommand(
            SubCommand::with_name("login")
                .about("Login as a user")
                .arg(Arg::with_name("username").required(true))
                .arg(Arg::with_name("password").required(true)),
        )
        .subcommand(SubCommand::with_name("logout").about("Logout the current user"))
        .subcommand(
            SubCommand::with_name("add")
                .about("Add a task to a list")
                .arg(Arg::with_name("list_name").required(true))
                .arg(Arg::with_name("item").required(true)),
        )
        .subcommand(
            SubCommand::with_name("show")
                .about("Show tasks")
                .arg(Arg::with_name("list_name"))
                .arg(Arg::with_name("all").short("a"))
                .arg(Arg::with_name("completed").short("c"))
                .arg(Arg::with_name("incomplete").short("i")),
        )
        .subcommand(
            SubCommand::with_name("complete")
                .about("Mark a task as completed")
                .arg(Arg::with_name("list_name").required(true))
                .arg(Arg::with_name("item_number").required(true)),
        )
        .subcommand(
            SubCommand::with_name("incomplete")
                .about("Mark a task as incomplete")
                .arg(Arg::with_name("list_name").required(true))
                .arg(Arg::with_name("item_number").required(true)),
        )
        .subcommand(
            SubCommand::with_name("remove")
                .about("Remove tasks or lists")
                .arg(Arg::with_name("list_name"))
                .arg(Arg::with_name("item_number")),
        )
        .subcommand(SubCommand::with_name("push").about("Push tasks to cloud"))
        .subcommand(SubCommand::with_name("pull").about("Pull tasks from cloud"))
}
