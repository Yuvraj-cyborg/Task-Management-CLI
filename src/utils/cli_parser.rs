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
}
