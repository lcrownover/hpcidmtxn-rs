use clap::{arg, command};
use std::error::Error;
use std::process::Command;

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

fn get_users_in_pirg(pirg: &String) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let command = Command::new("grep").args(&[pirg, "testgroups"]).output()?;

    let output_str = String::from_utf8(command.stdout)?;
    let users = output_str
        .split(':')
        .last()
        .expect("")
        .split(',')
        .map(|u| u.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    Ok(users)
}

fn main() {
    let matches = command!()
        .arg(arg!(-p --pirg <PIRG> "PIRG name").required(true))
        .arg(arg!(-s --server <SERVER> "hpcidmtxn server to connect to").required(true))
        .get_matches();

    let pirg = matches
        .get_one::<String>("pirg")
        .expect("pirg is required");

    let server = matches
        .get_one::<String>("server")
        .expect("server is required");

    let users: Vec<String> = get_users_in_pirg(pirg).unwrap_or_else(|err| {
        eprintln!("Error parsing users in pirg: {err}");
        std::process::exit(1)
    });

    if users.is_empty() {
        println!("No users in pirg");
        std::process::exit(0)
    }

    for user in users {
        let uid = get_uid_for_user(&user, &server).unwrap_or(0);
        if uid == 0 {
            println!("user {user} has no uid in AD")
        } else {
            println!("{user} : {uid}")
        }
    }
}
