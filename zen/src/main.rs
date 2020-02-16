extern crate clap;
extern crate tokio;
extern crate zenhub_api;

mod command;

use std::env;
use std::process;
use clap::{value_t, App, Arg, SubCommand};

const ZEN_AUTH_TOKEN_KEY: &str = "ZEN_AUTH_TOKEN";

#[tokio::main]
async fn main() {
    let api_token = match env::var(ZEN_AUTH_TOKEN_KEY) {
        Ok(api_token) => api_token,
        Err(_) => {
            eprintln!("{} must be set", ZEN_AUTH_TOKEN_KEY);
            process::exit(1);
        }
    };
    
    let issue_command = SubCommand::with_name("issue")
        .about("Work with ZenHub Issue")
        .setting(clap::AppSettings::ArgRequiredElseHelp)
        .subcommands(vec![
            SubCommand::with_name("status")
                .arg(Arg::with_name("repository id").required(true))
                .arg(Arg::with_name("issue number").required(true)),
            SubCommand::with_name("events")
                .arg(Arg::with_name("repository id").required(true))
                .arg(Arg::with_name("issue number").required(true)),
        ]);

    let epic_command = SubCommand::with_name("epic")
        .about("Work with ZenHub Epic")
        .setting(clap::AppSettings::ArgRequiredElseHelp)
        .subcommands(vec![
            SubCommand::with_name("list").arg(Arg::with_name("repository id").required(true)),
            SubCommand::with_name("status")
                .arg(Arg::with_name("repository id").required(true))
                .arg(Arg::with_name("epic id").required(true)),
        ]);

    let workspace_command = SubCommand::with_name("workspace")
        .about("Work with ZenHub Workspace")
        .setting(clap::AppSettings::ArgRequiredElseHelp)
        .subcommands(vec![
            SubCommand::with_name("list").arg(Arg::with_name("repository id").required(true))
        ]);

    let matches = App::new("zen")
        .setting(clap::AppSettings::ArgRequiredElseHelp)
        .version("1.0")
        .author("Masato Sogame. <poketo7878@gmail.com>")
        .about("ZenHub API CLI")
        .subcommands(vec![issue_command, epic_command, workspace_command])
        .get_matches();

    if let Some(issue_matches) = matches.subcommand_matches("issue") {
        if let Some(status_matches) = issue_matches.subcommand_matches("status") {
            let repo_id: i32 = value_t!(status_matches, "repository id", i32).unwrap();
            let issue_num: i32 = value_t!(status_matches, "issue number", i32).unwrap();
            let cmd =
                command::issue::status::Command::new(api_token.to_owned(), repo_id, issue_num);
            cmd.run().await;
        }

        if let Some(events_matches) = issue_matches.subcommand_matches("events") {
            let repo_id: i32 = value_t!(events_matches, "repository id", i32).unwrap();
            let issue_num: i32 = value_t!(events_matches, "issue number", i32).unwrap();
            let cmd =
                command::issue::events::Command::new(api_token.to_owned(), repo_id, issue_num);
            cmd.run().await;
        }

        return;
    }

    if let Some(epic_matches) = matches.subcommand_matches("epic") {
        if let Some(list_matches) = epic_matches.subcommand_matches("list") {
            let repo_id: i32 = value_t!(list_matches, "repository id", i32).unwrap();
            let cmd = command::epic::list::Command::new(api_token.to_owned(), repo_id);
            cmd.run().await;
        }

        if let Some(status_matches) = epic_matches.subcommand_matches("status") {
            let repo_id: i32 = value_t!(status_matches, "repository id", i32).unwrap();
            let epic_id: i32 = value_t!(status_matches, "epic id", i32).unwrap();
            let cmd = command::epic::status::Command::new(api_token.to_owned(), repo_id, epic_id);
            cmd.run().await;
        }
    }

    if let Some(workspace_matches) = matches.subcommand_matches("workspace") {
        if let Some(list_matches) = workspace_matches.subcommand_matches("list") {
            let repo_id: i32 = value_t!(list_matches, "repository id", i32).unwrap();
            let cmd = command::workspace::list::Command::new(api_token.to_owned(), repo_id);
            cmd.run().await;
        }
    }
}
