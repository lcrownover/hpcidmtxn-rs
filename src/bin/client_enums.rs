use clap::{arg, command, value_parser};
use hpcidmtxn_rs::QuerySource;
// use hpcidmtxn_rs::UserQueryableSource;
// use hpcidmtxn_rs::{GetentCommand, GroupFile};
use std::error::Error;
use std::path::PathBuf;

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

fn main() {
    let matches = command!()
        .arg(arg!(-g --group <GROUP> "group name").required(true))
        .arg(arg!(-s --server <SERVER> "hpcidmtxn server to connect to").required(true))
        .arg(
            arg!(-o --source_mode <TYPE> "either 'file' or 'getent'")
                .value_parser(["file", "getent"])
                .required(true),
        )
        .arg(
            arg!(-p --path <PATH> "path to the source file. default: /etc/group")
                .value_parser(value_parser!(PathBuf)),
        )
        .get_matches();

    let group = matches
        .get_one::<String>("group")
        .expect("group is required");

    let server = matches
        .get_one::<String>("server")
        .expect("server is required");

    let source_mode = matches
        .get_one::<String>("source_mode")
        .expect("source_mode is required");

    let user_res = match source_mode.as_str() {
        "file" => {
            let source_path = matches.get_one::<PathBuf>("path");
            if let Some(p) = source_path {
                let path_s = p.as_path().display().to_string();
                QuerySource::Path(path_s).get_users(group.to_string())
            } else {
                QuerySource::Path("/etc/group".to_string()).get_users(group.to_string())
            }
        }
        "getent" => QuerySource::GetentCommand.get_users(group.to_string()),
        _ => {
            eprintln!("couldnt match a query command using provided --source_mode and --path");
            std::process::exit(1)
        }
    };

    let users = user_res.unwrap_or_else(|err| {
        eprintln!("Error parsing users in group: {err}");
        std::process::exit(1)
    });

    if users.is_empty() {
        println!("No users in group");
        std::process::exit(0)
    }

    println!("{:?}", users);

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
