use hpcidmtxn_rs::{GetentCommand,GroupFile};
use hpcidmtxn_rs::UserQueryableSource;
use std::path::PathBuf;
use clap::{arg, command, value_parser};
use std::error::Error;

fn get_uid_for_user(user: &String, server: &String) -> Result<i32, Box<dyn Error>> {
    let url = format!("http://{}/user/{}", server, user);
    let resp = reqwest::blocking::get(url);
    match resp {
        Ok(resp) => {
            let uid = resp.text()?.parse::<i32>()?;
            return Ok(uid);
        }
        Err(err) => return Err(Box::new(err)),
    }
}

// fn get_users_in_pirg(pirg: &String, query_source: UserQueryCommand) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    // let mut command;
    // match query_source {
        // UserQueryCommand::GroupFile(path) => {
            // command = Command::new("grep");
            // command.args(&[pirg, &path])
        // },
        // UserQueryCommand::GetentGroup => {
            // command = Command::new("getent");
            // command.args(&["group", &pirg])
        // }
    // };
    // let command_output = command.output()?;
    // let output_str = String::from_utf8(command_output.stdout)?;
    // let users = output_str
        // .split(':')
        // .last()
        // .expect("")
        // .split(',')
        // .map(|u| u.trim().to_string())
        // .filter(|s| !s.is_empty())
        // .collect();
    // Ok(users)
// }

fn main() {
    let matches = command!()
        .arg(arg!(-p --pirg <PIRG> "PIRG name").required(true))
        .arg(arg!(-s --server <SERVER> "hpcidmtxn server to connect to").required(true))
        .arg(arg!(-o --source_mode <TYPE> "either 'file' or 'getent'").value_parser(["file", "getent"]).required(true))
        .arg(arg!(-a --path <PATH> "path to the source file").value_parser(value_parser!(PathBuf)))
        .get_matches();

    let pirg = matches
        .get_one::<String>("pirg")
        .expect("pirg is required");

    let server = matches
        .get_one::<String>("server")
        .expect("server is required");

    let source_mode = matches
        .get_one::<String>("source_mode")
        .expect("source_mode is required");

    let query_source: Box<dyn UserQueryableSource> = match source_mode.as_str() {
        "file" => {
            let source_path = matches.get_one::<PathBuf>("path");
            if let Some(p) = source_path {
                let path_s = p.as_path().display().to_string();
                Box::new(GroupFile{ path: path_s })
            } else {
                eprintln!("--path is required when using source_mode file");
                std::process::exit(1)
            }
        },
        "getent" => Box::new(GetentCommand),
        _ => {
            eprintln!("couldnt match a query command using provided --source_mode and --path");
            std::process::exit(1)
        },
    };

    let users = query_source.get_users_in_group(pirg).unwrap_or_else(|err| {
        eprintln!("Error parsing users in pirg: {err}");
        std::process::exit(1)
    });

    // let users: Vec<String> = get_users_in_pirg(pirg, query_command).unwrap_or_else(|err| {
        // eprintln!("Error parsing users in pirg: {err}");
        // std::process::exit(1)
    // });

    if users.is_empty() {
        println!("No users in pirg");
        std::process::exit(0)
    }

    for user in users {
        let uid = match get_uid_for_user(&user, &server) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1)
            }
        };
        if uid == 0 {
            println!("user {user} has no uid in AD")
        } else {
            println!("{user} : {uid}")
        }
    }
}
